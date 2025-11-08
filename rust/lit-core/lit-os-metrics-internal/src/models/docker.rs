use super::OsMetric;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of a running docker container
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DockerRunningContainers {
    /// The command used to inside the container
    pub container_command: String,
    /// The container id
    pub container_id: String,
    /// The container labels
    pub container_labels: Vec<DockerContainerLabel>,
    /// The container disk mounts
    pub container_mounts: Vec<DockerContainerMount>,
    /// The container name
    pub container_name: String,
    /// The container networks
    pub container_networks: Vec<DockerContainerNetwork>,
    /// The container ports
    pub container_ports: Vec<DockerContainerPort>,
    /// The container start time
    pub container_start_time: Option<usize>,
    /// The container state
    pub container_state: String,
    /// The image created time
    pub image_created_time: Option<usize>,
    /// The image id
    pub image_id: String,
    /// The image name
    pub image_name: String,
    /// The image size
    pub image_size: Option<usize>,
    /// The image tags
    pub image_tags: String,
    /// The query time
    pub query_time: Option<usize>,
    /// The status
    pub status: String,
}

impl TryFrom<&BTreeMap<String, String>> for DockerRunningContainers {
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
            container_command: value
                .get("container_command")
                .ok_or("missing container_command")?
                .clone(),
            container_id: value.get("container_id").ok_or("missing container_id")?.clone(),
            container_labels: serde_json::from_str(
                &value.get("container_labels").ok_or("missing container_labels")?,
            )
            .unwrap_or_default(),
            container_mounts: serde_json::from_str(
                &value.get("container_mounts").ok_or("missing container_mounts")?,
            )
            .unwrap_or_default(),
            container_name: value.get("container_name").ok_or("missing container_name")?.clone(),
            container_networks: serde_json::from_str(
                &value.get("container_networks").ok_or("missing container_networks")?,
            )
            .unwrap_or_default(),
            container_ports: serde_json::from_str(
                &value.get("container_ports").ok_or("missing container_ports")?,
            )
            .unwrap_or_default(),
            container_start_time: parse_optional("container_start_time")?,
            container_state: value.get("container_state").ok_or("missing container_state")?.clone(),
            image_created_time: parse_optional("image_created_time")?,
            image_id: value.get("image_id").ok_or("missing image_id")?.clone(),
            image_name: value.get("image_name").ok_or("missing image_name")?.clone(),
            image_size: parse_optional("image_size")?,
            image_tags: value.get("image_tags").ok_or("missing image_tags")?.clone(),
            query_time: parse_optional("query_time")?,
            status: value.get("status").ok_or("missing status")?.clone(),
        })
    }
}

impl From<&DockerRunningContainers> for BTreeMap<String, String> {
    fn from(value: &DockerRunningContainers) -> Self {
        let mut map = BTreeMap::new();
        map.insert("container_command".to_string(), value.container_command.clone());
        map.insert("container_id".to_string(), value.container_id.clone());
        map.insert(
            "container_labels".to_string(),
            serde_json::to_string(&value.container_labels)
                .expect("Unable to serialize container_labels"),
        );
        map.insert(
            "container_mounts".to_string(),
            serde_json::to_string(&value.container_mounts)
                .expect("Unable to serialize container_mounts"),
        );
        map.insert("container_name".to_string(), value.container_name.clone());
        map.insert(
            "container_networks".to_string(),
            serde_json::to_string(&value.container_networks)
                .expect("Unable to serialize container_networks"),
        );
        map.insert(
            "container_ports".to_string(),
            serde_json::to_string(&value.container_ports)
                .expect("Unable to serialize container_ports"),
        );
        map.insert(
            "container_start_time".to_string(),
            value.container_start_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("container_state".to_string(), value.container_state.clone());
        map.insert(
            "image_created_time".to_string(),
            value.image_created_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("image_id".to_string(), value.image_id.clone());
        map.insert("image_name".to_string(), value.image_name.clone());
        map.insert(
            "image_size".to_string(),
            value.image_size.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("image_tags".to_string(), value.image_tags.clone());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("status".to_string(), value.status.clone());
        map
    }
}

impl OsMetric for DockerRunningContainers {
    const NAME: &'static str = "os.running_containers";
}

/// The structure of a docker container label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerLabel {
    pub key: String,
    pub value: String,
}

/// The structure of a docker container mount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerMount {
    pub mount_driver: String,
    pub mount_host_path: String,
    pub mount_mode: String,
    pub mount_name: String,
    pub mount_propagation: String,
    pub mount_rw: String,
    pub mount_type: String,
}

/// The structure of a docker container network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerNetwork {
    pub container_ip: String,
    pub container_ip_prefix_len: usize,
    pub container_ipv6_address: String,
    pub container_ipv6_prefix_len: usize,
    pub container_mac_address: String,
    pub endpoint_id: String,
    pub gateway: String,
    pub ipv6_gateway: String,
    pub network_id: String,
    pub network_name: String,
}

/// The structure of a docker container port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerPort {
    pub host_ip: String,
    pub host_port: String,
    pub port: String,
    pub port_type: String,
}
