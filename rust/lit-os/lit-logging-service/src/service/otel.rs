use char_device::CharDevice;
use lit_observability::opentelemetry::KeyValue;
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use crossbeam_channel::{Receiver, Sender, bounded, select};
use tokio::runtime::Runtime;

use lit_core::config::LitConfig;

use crate::config::LitLoggingServiceConfig;
use crate::error::{Result, timeout_err, unexpected_err};
use crate::metrics;

const INTERNAL_LOG_PREFIX: &str = "lit_logging_service::service::otel";

const FLUSH_WAIT_SLEEP_MS: u64 = 50;
const FLUSH_TIMEOUT_SLEEP_MS: u64 = 2000;
const DATA_BACKLOG_LEN: usize = 100000;
const DEQUEUE_WAIT_MS: u64 = 500;

pub(crate) enum OTELServiceValue {
    Log(ExportLogsServiceRequest),
    Metric(ExportMetricsServiceRequest),
    Trace(ExportTraceServiceRequest),
}

pub(crate) struct OTELService {
    queue_tx: Sender<OTELServiceValue>,
    queue_rx: Receiver<OTELServiceValue>,
    queue_quit: Option<Sender<bool>>,
    queue_handle: Option<JoinHandle<Result<()>>>,
}

impl OTELService {
    pub fn new() -> Self {
        let (tx, rx) = bounded(DATA_BACKLOG_LEN);

        Self { queue_tx: tx, queue_rx: rx, queue_quit: None, queue_handle: None }
    }

    pub fn start(mut self, cfg: &LitConfig) -> Result<Self> {
        let dev_path = cfg.otel_service_device();

        let (quit_tx, quit_rx) = bounded(1);

        let rx = self.queue_rx.clone();

        self.queue_quit = Some(quit_tx);
        self.queue_handle = Some(thread::spawn(move || {
            let rt = Runtime::new().map_err(|e| unexpected_err(e, None))?;
            rt.block_on(queue_worker(rx, quit_rx, &dev_path));

            Ok(())
        }));

        // After starting the queue worker, update the queue size metric.
        metrics::counter::add_value(
            metrics::queue::QueueMetrics::OtelServiceQueueSize,
            self.queue_rx.len() as u64,
            &[],
        );

        Ok(self)
    }

    pub fn send(&self, entry: OTELServiceValue) -> Result<()> {
        self.queue_tx.try_send(entry).map_err(|e| unexpected_err(e, None))
    }

    fn flush(&self) -> Result<()> {
        eprintln!("{INTERNAL_LOG_PREFIX}: Waiting for log service entries to flush...");

        let start = SystemTime::now();

        // Wait for any logs to be pushed into the queue.
        thread::sleep(Duration::from_millis(FLUSH_WAIT_SLEEP_MS));

        loop {
            let sofar =
                SystemTime::now().duration_since(start).map_err(|e| unexpected_err(e, None))?;

            if sofar.as_millis() >= FLUSH_TIMEOUT_SLEEP_MS as u128 {
                return Err(timeout_err("timed out waiting for log msg queue to drain", None));
            }

            if self.queue_rx.is_empty() {
                return Ok(());
            }

            // Still have items
            thread::sleep(Duration::from_millis(DEQUEUE_WAIT_MS));
        }
    }
}

impl Drop for OTELService {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            // DO _NOT_ use log* functions here.
            eprintln!("{INTERNAL_LOG_PREFIX}: Failed to flush otel service message queue: {e:?}");
        }

        if let Some(quit) = self.queue_quit.as_ref() {
            let _ = quit.send(true);

            if let Some(handle) = self.queue_handle.take() {
                let _ = handle.join();
            }
        }
    }
}

// Worker
async fn queue_worker(rx: Receiver<OTELServiceValue>, quit_rx: Receiver<bool>, dev_path: &Path) {
    // For now, we just use a single unified character device since we're not able to get /dev/ttyS4 opened yet.
    let mut unified_dev = CharDevice::open(dev_path)
        .unwrap_or_else(|_| panic!("failed to open device: {dev_path:?}"));

    loop {
        select! {
            recv(quit_rx) -> _ => {
                // Shutdown.
                break;
            }
            recv(rx) -> res => {
                if let Ok(entry) = res {
                    match entry {
                        OTELServiceValue::Log(r) => {
                            // Serialize and write to corresponding device.
                            match serde_json::to_string(&r) {
                                Ok(json) => {
                                    if let Err(e) = writeln!(unified_dev, "{json}") {
                                        eprintln!("{INTERNAL_LOG_PREFIX}: Failed to write log entry to device (dropping) - {e:?}")
                                    }
                                    metrics::counter::add_value(metrics::device::DeviceMetrics::WriteSize, json.len() as u64, &[KeyValue::new(
                                        "telemetry_type",
                                        "log",
                                    )]);
                                },
                                Err(e) => {
                                    eprintln!(
                                        "{INTERNAL_LOG_PREFIX}: Failed to serialize log entry (dropping) - {e:?}"
                                    )
                                }
                            }
                        }
                        OTELServiceValue::Metric(r) => {
                            // Serialize and write to corresponding device.
                            match serde_json::to_string(&r) {
                                Ok(json) => {
                                    if let Err(e) = writeln!(unified_dev, "{json}") {
                                        eprintln!("{INTERNAL_LOG_PREFIX}: Failed to write log entry to device (dropping) - {e:?}")
                                    }
                                    metrics::counter::add_value(metrics::device::DeviceMetrics::WriteSize, json.len() as u64, &[KeyValue::new(
                                        "telemetry_type",
                                        "metric",
                                    )]);
                                },
                                Err(e) => {
                                    eprintln!(
                                        "{INTERNAL_LOG_PREFIX}: Failed to serialize log entry (dropping) - {e:?}"
                                    )
                                }
                            }
                        }
                        OTELServiceValue::Trace(r) => {
                            // Serialize and write to corresponding device.
                            match serde_json::to_string(&r) {
                                Ok(json) => {
                                    if let Err(e) = writeln!(unified_dev, "{json}") {
                                        eprintln!("{INTERNAL_LOG_PREFIX}: Failed to write log entry to device (dropping) - {e:?}")
                                    }
                                    metrics::counter::add_value(metrics::device::DeviceMetrics::WriteSize, json.len() as u64, &[KeyValue::new(
                                        "telemetry_type",
                                        "trace",
                                    )]);
                                },
                                Err(e) => {
                                    eprintln!(
                                        "{INTERNAL_LOG_PREFIX}: Failed to serialize log entry (dropping) - {e:?}"
                                    )
                                }
                            }
                        }
                    }
                }

                // After reading the message, update the queue size metric.
                metrics::counter::add_value(metrics::queue::QueueMetrics::OtelServiceQueueSize, rx.len() as u64, &[]);
            }
        }
    }
}
