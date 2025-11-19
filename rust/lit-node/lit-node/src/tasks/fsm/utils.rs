use crate::config::chain::CachedRootKey;
use crate::error::unexpected_err;
use crate::p2p_comms::CommsManager;
use crate::peers::PeerState;
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::tss_state::TssState;
use crate::utils::key_share_proof::{
    KeyShareProofs, compute_key_share_proofs, verify_key_share_proofs,
};
use crate::version::get_version;
use ethers::types::U256;
use lit_blockchain::contracts::staking::Version;
use lit_core::error::Result;
use lit_core::error::Unexpected;
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_current_and_new_peer_addresses(
    is_shadow: bool,
    peer_state: Arc<PeerState>,
) -> Result<(SimplePeerCollection, SimplePeerCollection)> {
    let (current_peers, new_peers) = if is_shadow {
        (
            peer_state.peers_in_current_shadow_epoch(),
            peer_state.peers_in_next_shadow_epoch(),
        )
    } else {
        (
            peer_state.peers(),
            peer_state.peers_in_next_epoch().active_peers(),
        )
    };

    let shadow_text = if is_shadow { "shadow" } else { "main" };
    let realm_id = match is_shadow {
        false => peer_state.realm_id(),
        true => peer_state.shadow_realm_id(),
    };

    debug!(
        "Current/new peers for {} realm {} epoch change: ( {}/{} )  {} / {} ",
        shadow_text,
        realm_id,
        &current_peers.0.len(),
        &new_peers.0.len(),
        &current_peers.debug_addresses(),
        &new_peers.debug_addresses(),
    );

    debug!(
        "Validators in realm {} for next epoch are locked: {} ",
        peer_state.peers_in_next_epoch().realm_id()?,
        peer_state
            .validators_for_next_epoch_locked(realm_id)
            .await?,
    );

    Ok((current_peers, new_peers))
}

pub(crate) async fn check_version_compatibility(peer_state: Arc<PeerState>) -> Result<bool> {
    let min_valid_version = peer_state
        .chain_data_config_manager
        .get_min_version_requirement()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get min version requirement".into())))?;
    let max_valid_version = peer_state
        .chain_data_config_manager
        .get_max_version_requirement()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get max version requirement".into())))?;
    is_compatible_version(
        &get_version().to_string(),
        min_valid_version,
        max_valid_version,
    )
}

fn is_compatible_version(
    version: &str,
    min_valid_version: Version,
    max_valid_version: Version,
) -> Result<bool> {
    trace!(
        "Checking version compatibility: version: {}, min_valid_version: {:?}, max_valid_version: {:?}",
        version, min_valid_version, max_valid_version
    );

    // Parse version (e.g. "0.2.14"), otherwise known as NODE_VERSION_UNMARKED!
    let version_parts = version.split('.').collect::<Vec<&str>>();
    if version_parts.len() != 3 {
        return Err(unexpected_err(
            format!("Invalid version: {}", version),
            None,
        ));
    }
    let curr_major = U256::from_dec_str(version_parts[0]).map_err(|e| unexpected_err(e, None))?;
    let curr_minor = U256::from_dec_str(version_parts[1]).map_err(|e| unexpected_err(e, None))?;
    let curr_patch = U256::from_dec_str(version_parts[2]).map_err(|e| unexpected_err(e, None))?;

    // If the min_valid_version is set to default values, that means the version is not set on-chain, so we should not check against
    // the minimum version requirement.
    if min_valid_version != Version::default()
        && (curr_major < min_valid_version.major
            || (curr_major == min_valid_version.major && curr_minor < min_valid_version.minor)
            || (curr_major == min_valid_version.major
                && curr_minor == min_valid_version.minor
                && curr_patch < min_valid_version.patch))
    {
        return Ok(false);
    }

    // If the max_valid_version is set to default values, that means the version is not set on-chain, so we should not check against
    // the maximum version requirement.
    if max_valid_version != Version::default()
        && (curr_major > max_valid_version.major
            || (curr_major == max_valid_version.major && curr_minor > max_valid_version.minor)
            || (curr_major == max_valid_version.major
                && curr_minor == max_valid_version.minor
                && curr_patch > max_valid_version.patch))
    {
        return Ok(false);
    }

    Ok(true)
}

pub(crate) async fn fsm_realm_id(peer_state: &Arc<PeerState>, is_shadow: bool) -> u64 {
    if is_shadow {
        peer_state.shadow_realm_id()
    } else {
        let realm_id = peer_state.realm_id();
        if realm_id == 0 {
            trace!("Node is not yet assigned to a realm.  Waiting for realm assignment.");
        }
        realm_id
    }
}

pub(crate) async fn key_share_proofs_check(
    tss_state: &Arc<TssState>,
    root_key_res: &Result<Vec<CachedRootKey>>,
    peers: &SimplePeerCollection,
    latest_dkg_id: &str,
    realm_id: u64,
    epoch: u64,
    lifecycle_id: u64,
) -> Result<()> {
    Ok(())
}

pub(crate) fn parse_epoch_number_from_dkg_id<T>(dkg_id: T) -> Result<U256>
where
    T: AsRef<str>,
{
    let dkg_id = dkg_id.as_ref();
    let epoch_number = dkg_id
        .split('_')
        .nth(2)
        .expect_or_err("Failed to parse epoch number")?;
    let epoch_number_u128 = epoch_number
        .parse::<u128>()
        .expect_or_err("Failed to parse epoch number as u128")?;
    Ok(U256::from(epoch_number_u128))
}

#[cfg(test)]
mod tests {
    use super::is_compatible_version;
    use super::parse_epoch_number_from_dkg_id;
    use crate::tasks::utils::parse_version;
    use crate::version::get_unmarked_version;
    use lit_blockchain::contracts::staking::Version;

    struct TestCase {
        node_version: String,
        min_valid_version: Version,
        max_valid_version: Version,
        expected_result: bool,
    }

    #[test]
    fn test_version_compatibility() {
        let test_cases = get_version_compability_test_cases();
        for (i, test_case) in test_cases.iter().enumerate() {
            let min_valid_version = test_case.min_valid_version.clone();
            let max_valid_version = test_case.max_valid_version.clone();
            let result = is_compatible_version(
                &test_case.node_version,
                min_valid_version,
                max_valid_version,
            )
            .expect("Failed to check version compatibility");
            assert_eq!(
                result,
                test_case.expected_result,
                "Test case {} failed",
                i + 1
            );
        }
    }

    fn get_version_compability_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            // Test patch version
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.15").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                max_valid_version: parse_version("0.2.15").expect("Unable to parse version"),
                expected_result: true,
            },
            // Test minor version
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                max_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                expected_result: true,
            },
            // Test major version
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                max_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
        ]
    }

    #[test]
    fn test_parse_epoch_number() {
        let epoch_number = parse_epoch_number_from_dkg_id("EPOCH_DKG_10_151")
            .expect("Failed to parse epoch number");
        assert_eq!(epoch_number.as_usize(), 10usize);
    }
}
