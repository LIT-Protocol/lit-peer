use serde::{Deserialize, Serialize};

/// API endpoint version identifier.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum EndpointVersion {
    /// Original API version (no version prefix).
    #[default]
    Initial,
    /// Version 1 of the API.
    V1,
    /// Version 2 of the API.
    V2,
}

impl EndpointVersion {
    pub const fn as_str(&self) -> &str {
        match self {
            EndpointVersion::Initial => "",
            EndpointVersion::V1 | EndpointVersion::V2 => "/v2",
        }
    }
}
