//! Data structures for release metadata serialization.
//!
//! These structs define the JSON schema for release information that gets
//! published to GitHub and stored in the release-data-store.

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Root release metadata structure (schema v1.0.0)
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReleaseInfo {
    pub schema_version: String,
    pub release_id: String,
    pub version: String,
    pub network_name: String,
    pub release_date: DateTime<Utc>,
    pub status: String,
    pub source: SourceInfo,
    pub artifacts: ArtifactsInfo,
    pub verification: VerificationInfo,
}

/// Source repository information (git branches and commits)
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SourceInfo {
    pub lit_os: GitInfo,
    pub lit_assets: GitInfo,
}

/// Git repository metadata
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GitInfo {
    pub branch: String,
    pub commit: String,
}

/// Build artifacts (IPFS manifest + Docker image)
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtifactsInfo {
    pub manifest_cid: String,
    pub docker_image: String,
}

/// Cryptographic verification data
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VerificationInfo {
    pub signed_git_tag: String,
    pub gpg_key_fingerprint: String,
}
