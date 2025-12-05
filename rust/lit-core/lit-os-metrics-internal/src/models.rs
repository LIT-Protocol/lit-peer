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
