use crate::server::hyper::handler::router::Router;
use http_body_util::BodyExt;
use hyperlocal::UnixListenerExt;
use sd_notify::NotifyState;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::warn;

pub mod handler;

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

    let listener = tokio::net::UnixListener::from_std(std_listener)
        .unwrap_or_else(|_| panic!("Unable to convert UnixListener to tokio: {:?}", &socket_path));

    listener
        .serve(|| {
            |req| async {
                let r = r.clone();
                let (parts, body) = req.into_parts();

                let bytes = body.collect().await.unwrap().to_bytes();
                let full_body = http_body_util::Full::new(bytes.into());
                let new_req = hyper::Request::from_parts(parts, full_body);
                r.route(new_req).await
            }
        })
        .await
        .expect("failed to await on unix socket");
}
