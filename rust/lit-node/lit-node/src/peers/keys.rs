use crate::error::{EC, Result, blockchain_err_code, conversion_err};
use crate::utils::eth::EthereumAddress;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::Address;
use lit_blockchain::contracts::staking::Staking;
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_common::coms_keys::ComsKeys;
use lit_node_common::config::LitNodeConfig;
use lit_node_common::eth_wallet_keys::EthWalletKeys;
use lit_rust_crypto::k256::ecdsa::SigningKey;
use std::sync::Arc;

pub struct PeerKeys {
    pub comms_keys: ComsKeys,
    pub attested_wallet_key: EthWalletKeys,
}

/// Returns the peer keys and a boolean indicating if the keys were newly created.
///
/// The condition for creating a new key is either:
/// - When either key is missing on disk, or
/// - When a key exists on the disk but the nodeAddressToStakerAddress mapping on
///   the staking contract maps to the zero address using this key.
///
/// In other words, the condition for using the existing keys is when the nodeAddressToStakerAddress
/// mapping on the staking contract maps to a non-zero address, and specifically, this peer's
/// staker address.
pub async fn get_or_generate_keys(
    cfg: Arc<LitConfig>,
    staking_contract: Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
) -> Result<(PeerKeys, bool)> {
    // Get staker address
    let staker_address: Address = cfg
        .staker_address()?
        .parse()
        .map_err(|e| conversion_err(e, Some("Could not parse staker address".to_string())))?;
    let staker_address_hex = bytes_to_hex(staker_address.0);

    // First check if both the attested wallet key and comms keys on disk.
    if attested_wallet::get_full_path(&staker_address_hex)
        .exists()
        .await
        && comms_keys::get_full_path(&staker_address_hex)
            .exists()
            .await
    {
        let existing_attested_wallet_key = attested_wallet::read_attested_wallet_key_from_disk::<
            EthWalletKeys,
        >(&staker_address_hex)
        .await?;
        let existing_comms_keys =
            comms_keys::read_comms_keys_from_disk::<ComsKeys>(&staker_address_hex).await?;

        // Then check if the nodeAddressToStakerAddress mapping on the staking contract maps to
        // this staker address.
        // Note that an alternative approach would be to check contract events if there ever was a successful transaction
        // to register these keys to chain.
        let registered_staker_address = staking_contract
            .node_address_to_staker_address(
                existing_attested_wallet_key
                    .verifying_key()
                    .to_eth_address()?,
            )
            .call()
            .await
            .map_err(|e| blockchain_err_code(e, EC::NodeRpcError, None))?;
        if registered_staker_address == staker_address {
            // If it does, return the existing key.
            return Ok((
                PeerKeys {
                    attested_wallet_key: existing_attested_wallet_key,
                    comms_keys: existing_comms_keys,
                },
                false,
            ));
        }
    }

    // Otherwise, create new keys, write them to disk and return them.
    let new_attested_wallet_key = EthWalletKeys::generate(&cfg).await?;
    let new_comms_keys = ComsKeys::default();
    attested_wallet::write_attested_wallet_key_to_disk(
        &staker_address_hex,
        &new_attested_wallet_key,
    )
    .await?;
    comms_keys::write_comms_keys_to_disk(&staker_address_hex, &new_comms_keys).await?;
    Ok((
        PeerKeys {
            attested_wallet_key: new_attested_wallet_key,
            comms_keys: new_comms_keys,
        },
        true,
    ))
}

mod attested_wallet {
    use async_std::path::PathBuf;
    use lit_node_common::config::attested_wallet_key_path;
    use serde::{Serialize, de::DeserializeOwned};

    use crate::common::storage::{create_storage_dir, read_from_disk, write_to_disk};
    use crate::error::Result;

    pub const ATTESTED_WALLET_KEY_FILE_NAME: &str = "AttestedWalletKey.cbor";

    /// Writes the attested wallet key to disk.
    ///
    /// Note that this currently does not use the key cache. If and when we do use the key cache,
    /// we should protect it in memory using `KeyCacheType::Protected`.
    pub(super) async fn write_attested_wallet_key_to_disk<T>(
        staker_address: &str,
        attested_wallet_key: &T,
    ) -> Result<()>
    where
        T: Serialize + Sync,
    {
        let directory_path = attested_wallet_key_path(staker_address);

        // Ensure the directory path exists
        create_storage_dir(directory_path.clone()).await?;

        write_to_disk(
            directory_path,
            ATTESTED_WALLET_KEY_FILE_NAME,
            attested_wallet_key,
        )
        .await
    }

    /// Reads the attested wallet key from disk.
    ///
    /// Note that this currently does not use the key cache. If and when we do use the key cache,
    /// we should protect it in memory using `KeyCacheType::Protected`.
    pub(super) async fn read_attested_wallet_key_from_disk<T>(staker_address: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let directory_path = attested_wallet_key_path(staker_address);

        // Ensure `directory_path` exists
        create_storage_dir(directory_path.clone()).await?;

        read_from_disk(directory_path, ATTESTED_WALLET_KEY_FILE_NAME).await
    }

    pub(super) fn get_full_path(staker_address: &str) -> PathBuf {
        let mut directory_path = attested_wallet_key_path(staker_address);
        directory_path.push(ATTESTED_WALLET_KEY_FILE_NAME);
        directory_path
    }
}

mod comms_keys {
    use async_std::path::PathBuf;
    use lit_node_common::config::comms_key_path;
    use serde::{Serialize, de::DeserializeOwned};

    use crate::common::storage::{create_storage_dir, read_from_disk, write_to_disk};
    use crate::error::Result;

    const COMMS_KEYS_FILE_NAME: &str = "CommsKeys.cbor";

    /// Writes the comms keys to disk.
    ///
    /// Note that this currently does not use the key cache. If and when we do use the key cache,
    /// we should protect it in memory using `KeyCacheType::Protected`.
    pub(super) async fn write_comms_keys_to_disk<T>(
        staker_address: &str,
        comms_keys: &T,
    ) -> Result<()>
    where
        T: Serialize + Sync,
    {
        let directory_path = comms_key_path(staker_address);

        // Ensure the directory path exists
        create_storage_dir(directory_path.clone()).await?;

        write_to_disk(directory_path, COMMS_KEYS_FILE_NAME, comms_keys).await
    }

    /// Reads the comms keys from disk.
    ///
    /// Note that this currently does not use the key cache. If and when we do use the key cache,
    /// we should protect it in memory using `KeyCacheType::Protected`.
    pub(super) async fn read_comms_keys_from_disk<T>(staker_address: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let directory_path = comms_key_path(staker_address);

        // Ensure `directory_path` exists
        create_storage_dir(directory_path.clone()).await?;

        read_from_disk(directory_path, COMMS_KEYS_FILE_NAME).await
    }

    pub(super) fn get_full_path(staker_address: &str) -> PathBuf {
        let mut directory_path = comms_key_path(staker_address);
        directory_path.push(COMMS_KEYS_FILE_NAME);
        directory_path
    }
}
