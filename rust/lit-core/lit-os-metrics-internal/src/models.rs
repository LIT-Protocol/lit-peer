use lit_observability::opentelemetry::KeyValue;

pub trait OsMetric {
    const NAME: &'static str;
}

/// Trait for metrics that have meaningful numeric values.
/// Implementing this trait allows the metric to be emitted as a gauge with a proper numeric value,
/// rather than a counter with an enumeration value.
pub trait GaugeMetric: OsMetric {
    /// Returns the primary gauge value for this metric.
    /// This should be the most important numeric value that represents the metric.
    fn gauge_value(&self) -> Option<f64>;

    /// Returns labels (key-value pairs) for this metric.
    /// These provide dimensional breakdown of the metric.
    fn gauge_labels(&self) -> Vec<KeyValue>;
}

/// Trait for metrics that represent metadata/attributes without meaningful numeric values.
/// Implementing this trait allows the metric to be emitted as an OpenTelemetry Non-Monotonic Sum
/// (Prometheus Info metric) with value 1 to indicate the presence/existence of a system with
/// these attributes.
///
/// This follows the [OpenTelemetry Prometheus compatibility specification](https://opentelemetry.io/docs/specs/otel/compatibility/prometheus_and_openmetrics/#info):
/// - Info metrics are converted to OTLP Non-Monotonic Sum (not Gauge)
/// - The value of 1 is intended to be viewed as a count, which should be summed together
///   when aggregating away labels
/// - Metric names MUST have the `_info` suffix to comply with the specification
///
/// The actual information is conveyed through the metric attributes/labels, not the numeric value.
pub trait InfoMetric: OsMetric {
    /// Returns labels (key-value pairs) for this metric.
    /// These provide dimensional breakdown of the metric and contain the actual information.
    fn info_labels(&self) -> Vec<KeyValue>;
}

mod cpu_info;
mod cron_job;
mod debian_package;
mod disk_info;
mod docker;
mod established_outbound;
mod interface_address;
mod iptables;
mod kernel_info;
mod listening_port;
mod load_average;
mod login_history;
mod memory_info;
mod os_info;
mod running_process;
mod system_info;
mod uptime;

pub use cpu_info::CpuInfo;
pub use cron_job::CronJob;
pub use debian_package::DebianPackage;
pub use disk_info::DiskInfo;
pub use docker::DockerRunningContainers;
pub use established_outbound::EstablishedOutbound;
pub use interface_address::InterfaceAddress;
pub use iptables::IptablesRule;
pub use kernel_info::KernelInfo;
pub use listening_port::ListeningPort;
pub use load_average::LoadAverage;
pub use login_history::LoginHistory;
pub use memory_info::MemoryInfo;
pub use os_info::OsInfo;
pub use running_process::RunningProcess;
pub use system_info::SystemInfo;
pub use uptime::Uptime;
