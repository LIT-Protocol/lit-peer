use crate::server::hyper::handler::router::Router;
use http_body_util::BodyExt;
use hyper::header;
use hyperlocal::UnixListenerExt;
use sd_notify::NotifyState;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use tracing::{info, warn};

pub mod handler;

static REQUEST_COUNTERS: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();

fn increment_request_counter(request_type: &str) -> u64 {
    let counters = REQUEST_COUNTERS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = counters
        .lock()
        .unwrap_or_else(|e| panic!("request counter mutex poisoned: {:?}", e));
    let entry = guard.entry(request_type.to_string()).or_insert(0);
    *entry += 1;
    *entry
}

pub async fn bind_unix_socket(socket_path: PathBuf, r: Router) {
    let r = Arc::new(r);

    let t_socket_path = socket_path.clone();
    thread::spawn(move || {
        for _ in 0..100 {
            if t_socket_path.exists() {
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }

        if t_socket_path.exists() {
            if let Err(e) = sd_notify::notify(true, &[NotifyState::Ready]) {
                warn!(error = ?e, "failed to send systemd notify");
            }
        } else {
            warn!("gave up waiting for socket to appear, not sending systemd notify");
        }
    });

    let std_listener = std::os::unix::net::UnixListener::bind(&socket_path)
        .unwrap_or_else(|_| panic!("Unable to bind to Unix socket: {:?}", &socket_path));
    std_listener.set_nonblocking(true).unwrap_or_else(|_| {
        panic!("Unable to set non-blocking on Unix socket: {:?}", &socket_path)
    });

    loop {
        let listener = match std_listener.try_clone() {
            Ok(listener) => tokio::net::UnixListener::from_std(listener).unwrap_or_else(|_| {
                panic!("Unable to convert UnixListener to tokio: {:?}", &socket_path)
            }),
            Err(e) => {
                warn!(error = ?e, "failed to clone unix listener; retrying");
                continue;
            }
        };

        let serve_result = listener
            .serve(|| {
                |req| async {
                    let r = r.clone();
                    let (parts, body) = req.into_parts();
                    let method = parts.method.clone();
                    let uri = parts.uri.clone();
                    let version = parts.version;
                    let headers = parts.headers.clone();
                    let path = uri.path().to_string();
                    let query = uri.query().map(str::to_string);
                    let request_type = format!("{} {}", method, path);
                    let request_count = increment_request_counter(&request_type);
                    let content_length = headers
                        .get(header::CONTENT_LENGTH)
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);
                    let user_agent = headers
                        .get(header::USER_AGENT)
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);
                    let forwarded_for = headers
                        .get("x-forwarded-for")
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);
                    let real_ip = headers
                        .get("x-real-ip")
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);
                    let request_id = headers
                        .get("x-request-id")
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);

                    info!(
                        request_type = %request_type,
                        request_count,
                        method = %method,
                        uri = %uri,
                        path = %path,
                        query = query.as_deref(),
                        version = ?version,
                        content_length = content_length.as_deref(),
                        user_agent = user_agent.as_deref(),
                        forwarded_for = forwarded_for.as_deref(),
                        real_ip = real_ip.as_deref(),
                        request_id = request_id.as_deref(),
                        headers = ?headers,
                        "handling request"
                    );

                    let bytes = body.collect().await.unwrap().to_bytes();
                    let full_body = http_body_util::Full::new(bytes.into());
                    let new_req = hyper::Request::from_parts(parts, full_body);
                    r.route(new_req).await
                }
            })
            .await;

        if let Err(e) = serve_result {
            if is_broken_pipe_error(e.as_ref()) {
                warn!(error = ?e, "unix socket client dropped; continuing");
                continue;
            }

            panic!("unix socket server exited: {:?}", e);
        }

        break;
    }
}

fn is_broken_pipe_error(err: &(dyn Error + 'static)) -> bool {
    if let Some(io_err) = err.downcast_ref::<io::Error>() {
        if io_err.kind() == io::ErrorKind::BrokenPipe {
            return true;
        }
    }

    let mut source = err.source();
    while let Some(source_err) = source {
        if let Some(io_err) = source_err.downcast_ref::<io::Error>() {
            if io_err.kind() == io::ErrorKind::BrokenPipe {
                return true;
            }
        }
        source = source_err.source();
    }
    false
}
