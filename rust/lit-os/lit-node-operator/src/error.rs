pub const PKG_NAME: &str = "lit_node_operator";

use derive_more::Display;
pub use lit_core::error::*;
use lit_core::generate_pkg_constructors;
use lit_core_derive::{Description, ErrorCode};
use std::fmt::Debug;

/// Error codes for the lit_node_operator module.
#[derive(Debug, Display, Description, ErrorCode)]
#[allow(dead_code)]
pub enum EC {
    /// A fault error occurred in the provisioning system
    #[code(kind = Unexpected, http_status = 500)]
    ProvFault,

    /// Failed to load the specified release
    #[code(kind = Validation, http_status = 502)]
    ProvReleaseLoadFailed,

    /// The release already exists
    #[code(kind = Validation, http_status = 400)]
    ProvReleaseExists,

    /// The release was not found
    #[code(kind = Validation, http_status = 404)]
    ProvReleaseNotFound,

    /// The release is not active and is not available for use
    #[code(kind = Validation, http_status = 404)]
    ProvReleaseNotActive,

    /// The release is invalid or has invalid elements
    #[code(kind = Validation, http_status = 404)]
    ProvReleaseInvalid,

    /// The release has failed to validate
    #[code(kind = Validation, http_status = 400)]
    ProvReleaseValidation,

    /// Failed to issue an IdBlock
    #[code(kind = Validation, http_status = 502)]
    ProvReleaseIdBlockIssueFailed,

    /// The attempt to issue an IdBlock met with invalidity
    #[code(kind = Validation, http_status = 400)]
    ProvReleaseIdBlockIssueInvalid,

    /// Error executing an external command (docker, iptables, script)
    #[code(kind = Unexpected, http_status = 500)]
    ExternalCommandFailed,

    /// An I/O error occurred during replica management
    #[code(kind = Unexpected, http_status = 500)]
    ReplicaIoError,

    /// A configuration value related to the replica is invalid
    #[code(kind = Unexpected, http_status = 500)]
    InvalidReplicaConfig,
}

generate_pkg_constructors!(PKG_NAME);
