use crate::error::{Result, unexpected_err};
use ethers::types::U256;
use lit_blockchain::contracts::staking::Version;

pub fn parse_version(version: &str) -> Result<Version> {
    let version_parts = version.split('.').collect::<Vec<&str>>();
    Ok(Version {
        major: U256::from_dec_str(version_parts[0])
            .map_err(|e| unexpected_err(e, Some("Failed to parse major version.".into())))?,
        minor: U256::from_dec_str(version_parts[1])
            .map_err(|e| unexpected_err(e, Some("Failed to parse minor version.".into())))?,
        patch: U256::from_dec_str(version_parts[2])
            .map_err(|e| unexpected_err(e, Some("Failed to parse patch version.".into())))?,
    })
}
