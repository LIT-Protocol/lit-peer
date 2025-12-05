use super::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// The DeviceID of the CPU.
    pub device_id: String,
    /// The model of the CPU.
    pub model: String,
    /// The manufacturer of the CPU.
    pub manufacturer: String,
    /// The processor type, such as Central, Math, or Video.
    pub processor_type: String,
    /// The current operating status of the CPU.
    pub cpu_status: Option<usize>,
    /// The number of cores of the CPU.
    pub number_of_cores: String,
    /// The number of logical processors of the CPU.
    pub logical_processors: Option<usize>,
    /// The width of the CPU address bus.
    pub address_width: String,
    /// The current frequency of the CPU.
    pub current_clock_speed: Option<usize>,
    /// The maximum possible frequency of the CPU.
    pub max_clock_speed: Option<usize>,
    /// The assigned socket on the board for the given CPU.
    pub socket_designation: String,
    /// The availability and status of the CPU.
    pub availability: Option<String>,
    /// The current percentage of utilization of the CPU.
    pub load_percentage: Option<usize>,
    /// The number of efficiency cores of the CPU. Only available on Apple Silicon
    pub number_of_efficiency_cores: Option<usize>,
    /// The number of performance cores of the CPU. Only available on Apple Silicon
    pub number_of_performance_cores: Option<usize>,
}

impl TryFrom<&BTreeMap<String, String>> for CpuInfo {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let parse_optional = |key: &str| -> Result<Option<usize>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseIntError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None),
            }
        };

        Ok(Self {
            device_id: value.get("device_id").ok_or("missing device_id")?.clone(),
            model: value.get("model").ok_or("missing model")?.clone(),
            manufacturer: value.get("manufacturer").ok_or("missing manufacturer")?.clone(),
            processor_type: value.get("processor_type").ok_or("missing processor_type")?.clone(),
            cpu_status: parse_optional("cpu_status")?,
            number_of_cores: value.get("number_of_cores").ok_or("missing number_of_cores")?.clone(),
            logical_processors: parse_optional("logical_processors")?,
            address_width: value.get("address_width").ok_or("missing address_width")?.clone(),
            current_clock_speed: parse_optional("current_clock_speed")?,
            max_clock_speed: parse_optional("max_clock_speed")?,
            socket_designation: value
                .get("socket_designation")
                .ok_or("missing socket_designation")?
                .clone(),
            availability: value.get("availability").cloned(),
            load_percentage: parse_optional("load_percentage")?,
            number_of_efficiency_cores: parse_optional("number_of_efficiency_cores")?,
            number_of_performance_cores: parse_optional("number_of_performance_cores")?,
        })
    }
}

impl From<&CpuInfo> for BTreeMap<String, String> {
    fn from(value: &CpuInfo) -> Self {
        let mut map = BTreeMap::new();
        map.insert("device_id".to_string(), value.device_id.clone());
        map.insert("model".to_string(), value.model.clone());
        map.insert("manufacturer".to_string(), value.manufacturer.clone());
        map.insert("processor_type".to_string(), value.processor_type.clone());
        map.insert(
            "cpu_status".to_string(),
            value.cpu_status.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("number_of_cores".to_string(), value.number_of_cores.clone());
        map.insert(
            "logical_processors".to_string(),
            value.logical_processors.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("address_width".to_string(), value.address_width.clone());
        map.insert(
            "current_clock_speed".to_string(),
            value.current_clock_speed.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "max_clock_speed".to_string(),
            value.max_clock_speed.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("socket_designation".to_string(), value.socket_designation.clone());
        map.insert("availability".to_string(), value.availability.clone().unwrap_or_default());
        map.insert(
            "load_percentage".to_string(),
            value.load_percentage.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "number_of_efficiency_cores".to_string(),
            value.number_of_efficiency_cores.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "number_of_performance_cores".to_string(),
            value.number_of_performance_cores.map(|v| v.to_string()).unwrap_or_default(),
        );
        map
    }
}

impl OsMetric for CpuInfo {
    const NAME: &'static str = "os.cpu_info";
}

impl GaugeMetric for CpuInfo {
    fn gauge_value(&self) -> Option<f64> {
        self.load_percentage.map(|v| v as f64)
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("device_id", self.device_id.clone()),
            KeyValue::new("model", self.model.clone()),
            KeyValue::new("manufacturer", self.manufacturer.clone()),
            KeyValue::new("processor_type", self.processor_type.clone()),
            KeyValue::new("number_of_cores", self.number_of_cores.clone()),
            KeyValue::new(
                "logical_processors",
                self.logical_processors.map(|v| v.to_string()).unwrap_or_default(),
            ),
            KeyValue::new("socket_designation", self.socket_designation.clone()),
        ]
    }
}
