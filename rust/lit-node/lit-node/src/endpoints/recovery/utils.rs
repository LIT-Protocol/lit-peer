use ethers::types::H160;
use lit_blockchain::contracts::backup_recovery::NextStateDownloadable;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_core::JsonAuthSig;
use rocket::State;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::common::key_helper::KeyCache;
use crate::endpoints::recovery::models::RecoveryShares;
use crate::error::{Result, conversion_err, unexpected_err, validation_err};
use crate::tss::common::key_persistence::RECOVERY_DKG_EPOCH;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::restore::RestoreState;
use crate::tss::common::storage::read_key_share_from_disk;
use lit_node_core::CurveType;
use lit_node_core::PeerId;

const SHARE_DOWNLOAD_SIG_EXP: u64 = 10;

pub async fn check_auth_sig_for_dec_share_upload(
    cfg: &LitConfig,
    restore_state: &State<Arc<RestoreState>>,
    auth_sig: &JsonAuthSig,
) -> Result<()> {
    let party_members = restore_state.get_recovery_party_members().await?;
    check_auth_sig_for_recovery_party(cfg, auth_sig, &party_members)
}

pub fn check_auth_sig_for_share_download(
    cfg: &LitConfig,
    auth_sig: &JsonAuthSig,
    party_member: &H160,
) -> Result<()> {
    check_auth_sig_for_recovery_party(cfg, auth_sig, &[*party_member])
}

pub fn check_auth_sig_for_recovery_party(
    cfg: &LitConfig,
    auth_sig: &JsonAuthSig,
    party_members: &[H160],
) -> Result<()> {
    match ethers::types::Signature::from_str(&auth_sig.sig) {
        Ok(sig) => {
            let presented_address = ethers::types::Address::from_str(&auth_sig.address)
                .expect_or_err("Failed to parse auth sig address")?;
            if !party_members.contains(&presented_address) {
                return Err(validation_err(
                    "Error in signatures addresses",
                    Some("Address mismatch, aborting".into()),
                ));
            }

            sig.verify(auth_sig.signed_message.clone(), presented_address)
                .map_err(|e| {
                    validation_err(e, Some("Invalid signature verification".into()))
                        .add_msg_to_details()
                })?;

            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| {
                    unexpected_err(e, Some("Could not get current time from system".into()))
                })?;
            let sm = auth_sig.signed_message.parse::<u128>().map_err(|e| {
                conversion_err(
                    e,
                    Some("could not convert message timestamp to integer".into()),
                )
            })?;
            let sig_ts = Duration::from_millis(sm as u64);
            let delta = now - sig_ts;
            if delta.as_secs() > SHARE_DOWNLOAD_SIG_EXP {
                return Err(validation_err(
                    "Expiration error",
                    Some("expiration on signature reached aborting".into()),
                ));
            }
        }
        Err(e) => {
            // Here we're not providing the full error as a detail to the user for security.
            return Err(validation_err(
                e,
                Some("Error parsing the signature".into()),
            ));
        }
    };

    Ok(())
}

pub async fn resolve_key_shares_from_disk(
    next_backup_state: &NextStateDownloadable,
    peer_id: &PeerId,
    staker_address: &str,
    key_cache: &KeyCache,
    realm_id: u64,
) -> Result<RecoveryShares> {
    let mut shares = HashMap::with_capacity(CurveType::NUM_USED_CURVES);
    let types = CurveType::into_iter()
        .map(|x| (ethers::types::U256::from(x), x))
        .collect::<HashMap<ethers::types::U256, CurveType>>();

    info!(
        "registered recovery keys {:?}",
        next_backup_state.registered_recovery_keys
    );

    for key in next_backup_state.registered_recovery_keys.clone() {
        if let Some(curve_type) = types.get(&key.key_type) {
            trace!("Key type found to be {}: {:?}", curve_type, &key.pubkey);
            trace!("Attempting to read key share of type: {:?}", key.key_type);
            let share: KeyShare = match read_key_share_from_disk(
                *curve_type,
                bytes_to_hex(&key.pubkey).as_str(),
                staker_address,
                peer_id,
                RECOVERY_DKG_EPOCH,
                realm_id,
                key_cache,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(unexpected_err(
                        e,
                        Some(
                            "Error while getting next backup state from contract. aborting".into(),
                        ),
                    ));
                }
            };
            shares.insert(*curve_type, share);
        }
    }

    if shares.len() < CurveType::NUM_USED_CURVES {
        return Err(unexpected_err(
            format!(
                "Could not resolve key shares: Only {} shares were found when {} were expected",
                shares.len(),
                CurveType::NUM_USED_CURVES
            ),
            Some("Error while getting next backup state from contract. aborting".into()),
        ));
    }
    trace!("Shares found on disk, returning data");

    Ok(RecoveryShares {
        bls_encryption_share: shares
            .remove(&CurveType::BLS)
            .expect_or_err("BLS share not found")?,
        k256_signing_share: shares
            .remove(&CurveType::K256)
            .expect_or_err("K256 share not found")?,
        p256_signing_share: shares
            .remove(&CurveType::P256)
            .expect_or_err("P256 share not found")?,
        p384_signing_share: shares
            .remove(&CurveType::P384)
            .expect_or_err("P384 share not found")?,
        ed25519_signing_share: shares
            .remove(&CurveType::Ed25519)
            .expect_or_err("Ed25519 share not found")?,
        ristretto25519_signing_share: shares
            .remove(&CurveType::Ristretto25519)
            .expect_or_err("Ristretto25519 share not found")?,
        ed448_signing_share: shares
            .remove(&CurveType::Ed448)
            .expect_or_err("Ed448 share not found")?,
        jubjub_signing_share: shares
            .remove(&CurveType::RedJubjub)
            .expect_or_err("Jubjub share not found")?,
        decaf377_signing_share: shares
            .remove(&CurveType::RedDecaf377)
            .expect_or_err("Decaf377 share not found")?,
        bls12381g1_signing_share: shares
            .remove(&CurveType::BLS12381G1)
            .expect_or_err("BLS12381G1")?,
    })
}

// will be used for deletion once share verification is implemented
#[allow(dead_code)]
pub async fn delete_key_shares_from_disk(
    next_backup_state: &NextStateDownloadable,
    peer_id: &PeerId,
    staker_address: &str,
    realm_id: u64,
) -> Result<bool> {
    let key_cache = KeyCache::default();
    let mut result = true;
    for key in next_backup_state.registered_recovery_keys.clone() {
        let curve_type = CurveType::try_from(key.key_type).map_err(|e| {
            unexpected_err(
                e,
                Some(format!(
                    "Unable to convert key type {} to curve type",
                    key.key_type
                )),
            )
        })?;
        debug!("Key type found to be {}: {:?}", curve_type, &key.pubkey);
        debug!(
            "Attempting to delete decryption share of type: {}",
            curve_type
        );
        let pk = bytes_to_hex(&key.pubkey);
        result &= crate::tss::common::storage::delete_keyshare(
            curve_type,
            &pk,
            staker_address,
            peer_id,
            RECOVERY_DKG_EPOCH,
            realm_id,
            &key_cache,
        )
        .await
        .is_ok();
        result &= crate::tss::common::storage::delete_key_share_commitments(
            curve_type,
            &pk,
            staker_address,
            peer_id,
            RECOVERY_DKG_EPOCH,
            realm_id,
            &key_cache,
        )
        .await
        .is_ok();
    }

    Ok(result)
}
