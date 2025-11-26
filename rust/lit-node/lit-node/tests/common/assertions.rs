use crate::common::{
    ecdsa::{sign_with_hd_key, simple_single_sign_with_hd_key},
    web_user_tests::{
        test_encryption_decryption_auth_sig, test_encryption_decryption_session_sigs,
    },
};
use tracing::{debug, info};

use lit_node_core::SigningScheme;
use lit_node_testnet::{
    end_user::EndUser, node_collection::get_identity_pubkeys_from_node_set, validator::Validator,
};
use lit_node_testnet::{
    node_collection::get_network_pubkey, testnet::actions::Actions, validator::ValidatorCollection,
};

/// This checker is intended to be used for checking the integrity of the network after notable network-wide
/// events such as epoch advancements.
#[derive(Debug)]
pub struct NetworkIntegrityChecker {
    initial_bls_pubkey: String,
    minted_pkp_pubkey: String,
    end_user: EndUser,
}

impl NetworkIntegrityChecker {
    pub async fn new(end_user: &EndUser, actions: &Actions) -> Self {
        let initial_bls_pubkey = get_network_pubkey(&actions).await;

        // Use the first PKP for the network integrity check.
        let (pubkey, token_id, _, _) = end_user.first_pkp().info();
        info!(
            "PKP for network integrity check: {:?} / token_id: {:?}",
            pubkey, token_id
        );

        Self {
            initial_bls_pubkey,
            minted_pkp_pubkey: pubkey,
            end_user: end_user.clone(),
        }
    }

    /// This function runs interpolation checks and performs decryption and signing operations on the network.
    pub async fn check(
        &self,
        validator_collection: &ValidatorCollection,
        validators_to_include: &Vec<&Validator>,
    ) {
        // Pubkey check.
        info!("Running network integrity checks");
        let latest_bls_pubkey = get_network_pubkey(validator_collection.actions()).await;
        info!(
            "Initial/Latest BLS pubkeys: {:?} / {:?}",
            self.initial_bls_pubkey, latest_bls_pubkey
        );
        assert_eq!(self.initial_bls_pubkey, latest_bls_pubkey);

        info!("Success:Initial BLS pubkey and latest BLS pubkey match.");
        // Decryption check.
        test_encryption_decryption_session_sigs(validator_collection, &self.end_user).await;

        info!("Success: Decryption checks passed");
        // Signing operation.
        assert!(
            simple_single_sign_with_hd_key(
                &validator_collection,
                &self.end_user,
                self.minted_pkp_pubkey.clone(),
                SigningScheme::EcdsaK256Sha256,
                validators_to_include
            )
            .await,
            "ECDSA Signing failed!"
        );

        info!("Success: ECDSA Signing checks passed");
        assert!(
            simple_single_sign_with_hd_key(
                &validator_collection,
                &self.end_user,
                self.minted_pkp_pubkey.clone(),
                SigningScheme::SchnorrEd25519Sha512,
                validators_to_include
            )
            .await,
            "Frost Signing failed!"
        );

        info!("Success: Frost Signing checks passed");

        info!("Success: Network integrity check passed");
    }

    /// This function runs interpolation checks and performs decryption and signing operations on the network.
    /// The signing operations are only asserted against when the presigns are completely drained.
    /// Instead of explicitly checking the logs of each deterministic subset of nodes - which is slightly complicated -
    /// we simply retry the operation up to a maximum number of times in an attempt to drain the presigns.
    // This should be removed once all nodes have updated to the new code that supports using BTs across boundaries.
    pub async fn check_with_drained_presigns(&self, validator_collection: &ValidatorCollection) {
        const MAX_TRIES: usize = 5;

        // Pubkey check.
        info!("Running pubkey checks");
        let latest_bls_pubkey = get_network_pubkey(validator_collection.actions()).await;
        assert_eq!(self.initial_bls_pubkey, latest_bls_pubkey);

        // Decryption check.
        info!("Running decryption checks");
        let node_set = &validator_collection.random_threshold_nodeset().await;
        let realm_id = ethers::types::U256::from(1);
        let epoch = validator_collection
            .actions()
            .get_current_epoch(realm_id)
            .await
            .as_u64();
        let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
        test_encryption_decryption_auth_sig(&node_set, epoch).await;

        // Signing check.
        info!("Running signing checks");
        for idx in 0..MAX_TRIES {
            if sign_with_hd_key(
                &validator_collection,
                &self.end_user,
                self.minted_pkp_pubkey.clone(),
                false,
                false,
                1,
                None,
                SigningScheme::EcdsaK256Sha256,
                &vec![],
            )
            .await
            {
                break;
            }
            debug!(
                "Failed {:?} try to sign with HD key (possibly due to bad, uncleared presigns being used) - retrying...",
                idx + 1
            );
        }

        info!("Network integrity check passed");
    }

    pub fn pkp_pubkey(&self) -> &str {
        &self.minted_pkp_pubkey
    }
}
