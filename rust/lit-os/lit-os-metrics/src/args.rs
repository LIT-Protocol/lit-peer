use clap::{Parser, ValueEnum};
use std::str::FromStr;

// Helper function to parse key=value pairs
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    s.split_once('=')
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .ok_or_else(|| format!("invalid KEY=value: no '=' found in '{}'", s))
}

#[derive(Parser, Debug)]
pub struct CmdArgs {
    #[clap(short, long, default_value = "/var/osquery/osquery.em")]
    pub osquery_socket: String,
    #[clap(short, long, default_value = "lit-os")]
    pub meter_name: String,
    #[clap(short, long, value_enum, value_parser)]
    pub query: Vec<Query>,
    #[clap(short, long, default_value_t = false)]
    pub plain: bool,
    /// A list of key-value pairs to be added as resource attributes to all telemetry.
    /// Useful for adding context to differentiate telemetry from different environments here 'guest' and 'host',
    /// e.g., --resource-attribute system_context=host
    #[clap(long = "resource-attribute", value_parser = parse_key_val, value_name = "KEY=VALUE")]
    pub resource_attributes: Vec<(String, String)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, ValueEnum)]
pub enum Query {
    RunningProcess,
    EstablishedOutbound,
    CronJob,
    LoginHistory,
    OsInfo,
    InterfaceAddress,
    DockerRunningContainers,
    DebianPackage,
    CpuInfo,
    DiskInfo,
    MemoryInfo,
    LoadAverage,
    ListeningPorts,
    KernelInfo,
    Uptime,
    Iptables,
    SystemInfo,
}

impl FromStr for Query {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "running-process" => Ok(Self::RunningProcess),
            "established-outbound" => Ok(Self::EstablishedOutbound),
            "cron-job" => Ok(Self::CronJob),
            "login-history" => Ok(Self::LoginHistory),
            "os-info" => Ok(Self::OsInfo),
            "interface-address" => Ok(Self::InterfaceAddress),
            "docker-running-containers" => Ok(Self::DockerRunningContainers),
            "debian-package" => Ok(Self::DebianPackage),
            "cpu-info" => Ok(Self::CpuInfo),
            "disk-info" => Ok(Self::DiskInfo),
            "memory-info" => Ok(Self::MemoryInfo),
            "load-average" => Ok(Self::LoadAverage),
            "listening-ports" => Ok(Self::ListeningPorts),
            "kernel-info" => Ok(Self::KernelInfo),
            "uptime" => Ok(Self::Uptime),
            "iptables" => Ok(Self::Iptables),
            "system-info" => Ok(Self::SystemInfo),
            _ => Err(format!("Invalid query, '{}'", s)),
        }
    }
}
