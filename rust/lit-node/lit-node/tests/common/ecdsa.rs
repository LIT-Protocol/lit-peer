use crate::common::pkp::generate_data_to_send;
use crate::common::pkp::sign_message_with_pkp_custom_headers;
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::{
    get_identity_pubkeys_from_node_set, hit_ports_with_json_body_join_all,
};
use lit_node_testnet::validator::Validator;
use lit_node_testnet::validator::ValidatorCollection;
use sha2::Digest;

use ethers::{types::U256, utils::keccak256};
use futures::future::join_all;
use lit_node_core::SigningScheme;
use lit_node_core::response::JsonPKPSigningResponse;
use tracing::{info, warn};

pub async fn simple_single_sign_with_hd_key(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
    pubkey: String,
    signing_scheme: SigningScheme,
    validators_to_include: &Vec<&Validator>,
) -> bool {
    sign_with_hd_key(
        validator_collection,
        end_user,
        pubkey,
        false,
        false,
        1,
        None,
        signing_scheme,
        validators_to_include,
    )
    .await
}

#[doc = "Mint a new key and sign with it.  This is a helper function for the test_pkp_hd_sign_generic_key test."]
pub async fn sign_with_hd_key(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
    pubkey: String,
    concurrent_signing: bool,
    concurrent_randomization: bool,
    messages_to_sign: i32,
    message_to_sign: Option<String>,
    signing_scheme: SigningScheme,
    stakers_to_include: &Vec<&Validator>,
) -> bool {
    let actions = validator_collection.actions();
    let wallet = end_user.wallet.clone();

    let node_set = validator_collection
        .partially_random_threshold_nodeset(stakers_to_include)
        .await;
    let node_set_with_keys = get_identity_pubkeys_from_node_set(&node_set).await;

    let mut validation = false;
    let mut future_validations = Vec::new();
    let expected_responses = node_set_with_keys.len();
    let max_sleep_ms = 100; // a number between 1 and size of random number generator (currently a u8) ... creates concurrency when the rnd is above this value
    let realm_id = U256::from(1);
    let epoch = actions.get_current_epoch(realm_id).await.as_u64();

    for i in 0..messages_to_sign {
        let to_sign = match message_to_sign.clone() {
            Some(m) => m,
            None => format!("test message #{}", i),
        };

        info!("Testing message #{}: {:?}", i, to_sign);
        let to_sign = if signing_scheme.hash_prior_to_sending() {
            match signing_scheme {
                SigningScheme::EcdsaK256Sha256 => keccak256(to_sign.as_bytes()).to_vec(),
                SigningScheme::EcdsaP256Sha256 => keccak256(to_sign.as_bytes()).to_vec(),
                SigningScheme::EcdsaP384Sha384 => {
                    sha3::Keccak384::digest(to_sign.as_bytes()).to_vec()
                }
                _ => to_sign.as_bytes().to_vec(),
            }
        } else {
            to_sign.as_bytes().to_vec()
        };

        if concurrent_signing {
            let data_to_send =
                generate_data_to_send(&node_set, end_user, pubkey.clone(), to_sign, signing_scheme)
                    .await
                    .expect("Failed to generate PKP Signing Request.");
            let cmd = "web/pkp/sign/v2".to_string();

            let node_set_clone = node_set_with_keys.clone();
            let future_sign = tokio::spawn(async move {
                hit_ports_with_json_body_join_all::<_, JsonPKPSigningResponse>(
                    &node_set_clone,
                    cmd,
                    data_to_send,
                )
                .await
            });
            future_validations.push(future_sign);
            if concurrent_randomization {
                let mut sleep_time = rand::random::<u8>() as u64;
                if sleep_time > max_sleep_ms {
                    sleep_time = 0;
                }
                actions.sleep_millis(sleep_time).await;
            }
        } else {
            sign_message_with_pkp_custom_headers(
                &node_set_with_keys,
                wallet.clone(),
                to_sign,
                pubkey.clone(),
                epoch,
                signing_scheme,
            )
            .await
            .expect("Failed to sign message.");
            validation = true;
        }
    }

    if concurrent_signing {
        warn!("Waiting for concurrent signing to complete.");
        let validations = join_all(future_validations).await;
        for v in validations {
            let responses = v.unwrap();

            assert!(responses.is_ok());
            let responses = responses.unwrap();
            assert_eq!(responses.len(), expected_responses);
            assert!(responses.iter().all(|r| r.ok));
            debug!("Signature responses: {:?}", responses);

            validation = true;
        }
    }

    validation
}
