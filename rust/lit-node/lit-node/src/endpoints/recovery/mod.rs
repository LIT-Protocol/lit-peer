use self::utils::resolve_key_shares_from_disk;
use crate::common::key_helper::KeyCache;
use crate::endpoints::recovery::utils::delete_key_shares_from_disk;
use crate::error::{config_err, conversion_err, unexpected_err};
use crate::peers::peer_state::models::SimplePeer;
use crate::tss::common::tss_state::TssState;
use blsful::inner_types::G1Projective;
use ed448_goldilocks::EdwardsPoint;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::H160,
};
use jubjub::SubgroupPoint;
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::backup_recovery::{BackupRecovery, NextStateDownloadable};
use lit_core::{config::LitConfig, utils::binary::bytes_to_hex};
use lit_node_common::config::LitNodeConfig as _;
use lit_node_core::CurveType;
use lit_recovery::models::DownloadedShareData;
use std::sync::Arc;
use vsss_rs::curve25519::{WrappedEdwards, WrappedRistretto};

pub mod endpoints;
mod models;
mod utils;

pub async fn do_share_download_from_rec_dkg(
    tss_state: &Arc<TssState>,
    cfg: &LitConfig,
    party_members: &H160,
    recovery_contract: &BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
) -> lit_core::error::Result<Vec<DownloadedShareData>> {
    let subnet_id = cfg
        .subnet_id()
        .map_err(|e| unexpected_err(e, Some("Error while getting subnet id from config".into())))?;

    trace!("Pulling octals from chain state");
    let recovery_peer_addresses = match recovery_contract
        .get_staker_addresses_for_dkg()
        .call()
        .await
    {
        Ok(staker_addrs) => staker_addrs,
        Err(e) => {
            return Err(unexpected_err(e, None));
        }
    };

    trace!("reading staker address from config");
    let staking_address = match cfg.staker_address() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(config_err(
                e,
                Some("Error while loading staker address".into()),
            ));
        }
    };

    let staking_addr: H160 = match staking_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(conversion_err(
                e,
                Some("Could not convert staking address to H160 type".into()),
            ));
        }
    };

    let mut index: Option<u16> = None;
    for (i, addr) in recovery_peer_addresses.iter().enumerate() {
        if *addr == staking_addr {
            index = Some((i + 1) as u16);
            break;
        }
    }

    if index.is_none() {
        return Err(unexpected_err(
            "Could not find wallet address in peer set",
            None,
        ));
    }
    let index = index.expect("Index should not be None");

    if index as usize > recovery_peer_addresses.len() {
        return Err(unexpected_err(
            "Could not find wallet address in peer set",
            None,
        ));
    }

    let next_backup_state: NextStateDownloadable =
        match recovery_contract.get_next_backup_state().await {
            Ok(nbs) => nbs,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Error while getting next backup state from contract. aborting".into()),
                ));
            }
        };
    let peer_item = tss_state
        .peer_state
        .get_peer_item_from_staker_addr(staking_addr)?;
    let validator = tss_state
        .peer_state
        .get_validator_from_node_address(peer_item.node_address)?;
    let peer = SimplePeer::from(&validator);

    trace!("Found next state in contract, pulling shares from disk");
    let peer_id = peer.peer_id;
    let key_cache = KeyCache::default();
    let realm_id = tss_state.peer_state.realm_id();
    let recovery_shares = match resolve_key_shares_from_disk(
        &next_backup_state,
        &peer_id,
        &staking_address,
        &key_cache,
        realm_id,
    )
    .await
    {
        Ok(shares) => shares,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some("Error while getting shares from disk".into()),
            ));
        }
    };

    // k256 and bls public points (public keys)
    let bls_pub_key = recovery_shares.bls_encryption_share.public_key_as_bytes()?;
    let k256_pub_key = recovery_shares.k256_signing_share.public_key_as_bytes()?;
    let p256_pub_key = recovery_shares.p256_signing_share.public_key_as_bytes()?;
    let p384_pub_key = recovery_shares.p384_signing_share.public_key_as_bytes()?;
    let ed25519_pub_key = recovery_shares
        .ed25519_signing_share
        .public_key_as_bytes()?;
    let ristretto25519_pub_key = recovery_shares
        .ristretto25519_signing_share
        .public_key_as_bytes()?;
    let ed448_pub_key = recovery_shares.ed448_signing_share.public_key_as_bytes()?;
    let jubjub_pub_key = recovery_shares.jubjub_signing_share.public_key_as_bytes()?;
    let decaf377_pub_key = recovery_shares
        .decaf377_signing_share
        .public_key_as_bytes()?;
    let bls12381g1_pub_key = recovery_shares
        .bls12381g1_signing_share
        .public_key_as_bytes()?;

    let session_id = next_backup_state.session_id.to_string();

    Ok(vec![
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.bls_encryption_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .bls_encryption_share
                    .default_share::<G1Projective>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::BLS.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.k256_signing_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .k256_signing_share
                    .default_share::<k256::ProjectivePoint>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::K256.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.p256_signing_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .p256_signing_share
                    .default_share::<p256::ProjectivePoint>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::P256.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.p384_signing_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .p384_signing_share
                    .default_share::<p384::ProjectivePoint>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::P384.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.ed25519_signing_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .ed25519_signing_share
                    .default_share::<WrappedEdwards>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::Ed25519.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares
                .ristretto25519_signing_share
                .hex_public_key
                .clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .ristretto25519_signing_share
                    .default_share::<WrappedRistretto>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::Ristretto25519.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.ed448_signing_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .ed448_signing_share
                    .default_share::<EdwardsPoint>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::Ed448.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares.jubjub_signing_share.hex_public_key.clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .jubjub_signing_share
                    .default_share::<SubgroupPoint>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::RedJubjub.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares
                .decaf377_signing_share
                .hex_public_key
                .clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .decaf377_signing_share
                    .default_share::<decaf377::Element>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::RedDecaf377.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            session_id: session_id.clone(),
            encryption_key: recovery_shares
                .bls12381g1_signing_share
                .hex_public_key
                .clone(),
            decryption_key_share: serde_json::to_string(
                &recovery_shares
                    .bls12381g1_signing_share
                    .default_share::<G1Projective>()?,
            )
            .map_err(|e| unexpected_err(e, None))?,
            curve: CurveType::BLS12381G1.to_string(),
            subnet_id: subnet_id.clone(),
        },
    ])
}

pub async fn do_delete_share_from_disk(
    tss_state: &Arc<TssState>,
    cfg: &LitConfig,
    party_members: &H160,
    recovery_contract: &BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
) -> lit_core::error::Result<bool> {
    let subnet_id = match cfg.subnet_id() {
        Ok(id) => id,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some("Error while getting subnet id from config".into()),
            ));
        }
    };

    trace!("Pulling octals from chain state");
    let recovery_peer_addresses = match recovery_contract
        .get_staker_addresses_for_dkg()
        .call()
        .await
    {
        Ok(staker_addrs) => staker_addrs,
        Err(e) => {
            return Err(unexpected_err(e, None));
        }
    };

    trace!("reading staker address from config");
    let staking_address = match cfg.staker_address() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(config_err(
                e,
                Some("Error while loading staker address".into()),
            ));
        }
    };

    let staking_addr: H160 = match staking_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(conversion_err(
                e,
                Some("Could not convert staking address to H160 type".into()),
            ));
        }
    };

    let mut index: Option<u16> = None;
    for (i, addr) in recovery_peer_addresses.iter().enumerate() {
        if *addr == staking_addr {
            index = Some((i + 1) as u16);
            break;
        }
    }
    if index.is_none() {
        return Err(unexpected_err(
            "Could not find wallet address in peer set",
            None,
        ));
    }
    let index = index.expect("Index should not be None");

    if index as usize > recovery_peer_addresses.len() {
        return Err(unexpected_err(
            "Could not find wallet address in peer set",
            None,
        ));
    }

    let next_backup_state: NextStateDownloadable =
        match recovery_contract.get_next_backup_state().await {
            Ok(nbs) => nbs,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Error while getting next backup state from contract. aborting".into()),
                ));
            }
        };

    let peer_item = tss_state
        .peer_state
        .get_peer_item_from_staker_addr(staking_addr)?;
    let validator = tss_state
        .peer_state
        .get_validator_from_node_address(peer_item.node_address)?;
    let peer = SimplePeer::from(&validator);
    let peer_id = peer.peer_id;
    let realm_id = tss_state.peer_state.realm_id();
    let are_deleted =
        delete_key_shares_from_disk(&next_backup_state, &peer_id, &staking_address, realm_id)
            .await?;

    Ok(are_deleted)
}

pub fn get_staker_address(cfg: &LitConfig) -> crate::error::Result<String> {
    let staker_address = match cfg.staker_address() {
        Ok(addr) => addr,
        Err(e) => return Err(unexpected_err(e, None)),
    };

    let staker_address: ethers::types::H160 = match staker_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(conversion_err(
                e,
                Some(format!(
                    "Could not convert staking address to H160 type from {}",
                    staker_address
                )),
            ));
        }
    };

    Ok(bytes_to_hex(staker_address))
}
