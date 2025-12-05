use crate::common::auth_sig::{generate_authsig_item, get_session_sigs_for_auth};
use anyhow::Result;
use ecdsa::{RecoveryId, Signature};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::types::U256;
use ethers::signers::Wallet;
use lit_core::utils::binary::hex_to_bytes;
use lit_node_core::{
    AuthSigItem, LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
    LitResourcePrefix, NodeSet, SignableOutput, SigningScheme,
    request::JsonPKPSigningRequest,
    response::{GenericResponse, JsonPKPSigningResponse},
};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::NodeIdentityKey;
use lit_sdk::signature::combine_and_verify_signature_shares;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignWithPKPReturn {
    success: bool,
    signed_data: Vec<u8>,
    signature_share: SignableOutput,
}

// copied from lit_ecdsa_wasm_combine
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignedDatak256 {
    pub sig_type: String,
    pub data_signed: k256::Scalar,
    pub signature_share: k256::Scalar,
    pub share_id: k256::Scalar,
    pub big_r: k256::AffinePoint,
    pub public_key: k256::AffinePoint,
    pub sig_name: String,
}
// copied from lit_ecdsa_wasm_combine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureRecidHex {
    pub r: String,
    pub s: String,
    pub recid: u8,
}

#[doc = "Sign a message with a PKP with custom headers"]
pub async fn sign_message_with_pkp_custom_headers(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    wallet: Wallet<SigningKey>,
    to_sign: Vec<u8>,
    pubkey: String,
    epoch: u64,
    signing_scheme: SigningScheme,
) -> Result<()> {
    let _ = sign_with_pkp_request(node_set, wallet, to_sign, pubkey, epoch, signing_scheme).await?;
    Ok(())
}

pub async fn generate_data_to_send(
    node_set: &[NodeSet],
    end_user: &EndUser,
    pubkey: String,
    to_sign: Vec<u8>,
    signing_scheme: SigningScheme,
) -> Result<JsonPKPSigningRequest> {
    let realm_id = U256::from(1);
    let epoch = end_user
        .actions()
        .get_current_epoch(realm_id)
        .await
        .as_u64();
    generate_data_to_send_with_epoch(&node_set, end_user, pubkey, to_sign, signing_scheme, epoch)
        .await
}

pub async fn generate_data_to_send_with_epoch(
    node_set: &[NodeSet],
    end_user: &EndUser,
    pubkey: String,
    to_sign: Vec<u8>,
    signing_scheme: SigningScheme,
    epoch: u64,
) -> Result<JsonPKPSigningRequest> {
    debug!(
        "generate_data_to_send_with_epoch: signing_scheme - {}",
        signing_scheme
    );
    let auth_sig = generate_authsig_item(&end_user.wallet).await?;
    let data_to_send = JsonPKPSigningRequest {
        auth_sig,
        to_sign,
        pubkey,
        auth_methods: None,
        signing_scheme,
        epoch,
        node_set: node_set.to_vec(),
    };
    Ok(data_to_send)
}

pub async fn generate_session_sigs_and_send_signing_requests(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    wallet: Wallet<SigningKey>,
    to_sign: Vec<u8>,
    pubkey: String,
    epoch: u64,
    signing_scheme: SigningScheme,
) -> Vec<GenericResponse<JsonPKPSigningResponse>> {
    let session_sigs = get_session_sigs_for_auth(
        &node_set,
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
        ],
        Some(wallet),
        None,
        None,
    );
    let nodes = node_set
        .iter()
        .map(|(node_set, _)| node_set.clone())
        .collect::<Vec<NodeSet>>();

    let my_secret_key = rand::rngs::OsRng.r#gen();

    let start = std::time::Instant::now();
    let responses = lit_sdk::PKPSigningRequest::new()
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .node_set(
            session_sigs
                .into_iter()
                .map(|sig_and_nodeset| {
                    let body = JsonPKPSigningRequest {
                        auth_sig: AuthSigItem::Single(sig_and_nodeset.session_sig),
                        to_sign: to_sign.clone(),
                        pubkey: pubkey.clone(),
                        auth_methods: None,
                        signing_scheme,
                        epoch,
                        node_set: nodes.clone(),
                    };
                    lit_sdk::EndpointRequest {
                        identity_key: sig_and_nodeset.identity_key,
                        node_set: sig_and_nodeset.node.clone(),
                        body,
                    }
                })
                .collect(),
        )
        .build()
        .unwrap()
        .send(&my_secret_key)
        .await
        .unwrap();
    debug!("Sign-only time elapsed: {:?}", start.elapsed());

    // Send out our signature request to all the nodes.
    responses.results().to_owned()
}

pub async fn sign_with_pkp_request(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    wallet: Wallet<SigningKey>,
    to_sign: Vec<u8>,
    pubkey: String,
    epoch: u64,
    signing_scheme: SigningScheme,
) -> Result<(String, String, String, RecoveryId)> {
    // Remember, for ECDSA signatures we need 100% participation (API responses) from the deterministic subset,
    // which has the size of `get_threshold_count(validator_set)`.
    let expected_responses = node_set.len();

    let endpoint_responses = generate_session_sigs_and_send_signing_requests(
        node_set,
        wallet,
        to_sign.clone(),
        pubkey.clone(),
        epoch,
        signing_scheme,
    )
    .await;
    info!("endpoint_responses: {:?}", endpoint_responses);

    assert!(endpoint_responses.len() >= expected_responses);

    if endpoint_responses.iter().any(|e| !e.ok) {
        // return default sig
        return Err(anyhow::anyhow!(
            "Could not sign with PKP, returning empty sig and false validation"
        ));
    }

    let responses: Vec<JsonPKPSigningResponse> = endpoint_responses
        .into_iter()
        .map(|x| {
            assert!(x.ok);
            assert!(x.data.is_some());
            x.data.unwrap()
        })
        .collect();

    trace!("json_responses: {:?}", responses);

    // collect the shares into a struct and a set of string that can be used to recombine using the WASM module.
    let lit_sdk::signature::SignedDataOutput {
        signature,
        verifying_key: public_key,
        signed_data: message,
        recovery_id: recid,
    } = decode_endpoint_responses(responses);

    info!(
        "signature: {}\n public key: {}\n, msg: {:?}, recid: {:?}",
        signature, public_key, message, recid
    );

    info!("pkp requested to be used: {:?}", &pubkey.clone());

    let id = recid
        .map(RecoveryId::from_byte)
        .unwrap_or_else(|| Some(RecoveryId::new(false, false)))
        .unwrap();
    Ok((signature, public_key, message, id))
}

#[doc = "Recombine a set of shares using code imported from the Lit ECDSA WASM module."]
pub fn recombine_shares_using_wasm(
    shares: Vec<String>,
) -> Result<(Signature<k256::Secp256k1>, RecoveryId)> {
    // use the WASM module
    info!("shares for wasm combine: {:?}", &shares);
    let result = lit_ecdsa_wasm_combine::combiners::k256_cait_sith::combine_signature(shares);
    info!("result: {:?}", &result);
    let wasm_sig = serde_json::from_str::<SignatureRecidHex>(&result)?;
    info!("wasm_sig: {:?}", &wasm_sig);
    let sig = format!("{}{}", &wasm_sig.r[2..], wasm_sig.s);
    let sig_slice = hex_to_bytes(sig)?;
    let signature = Signature::from_slice(&sig_slice)?;
    let recovery_id = k256::ecdsa::RecoveryId::try_from(wasm_sig.recid)?;
    Ok((signature, recovery_id))
}

#[doc = "Decode the responses from the nodes into a set of string based shares and a set of ECDSA signature shares."]
pub fn decode_endpoint_responses(
    endpoint_responses: Vec<JsonPKPSigningResponse>,
) -> lit_sdk::signature::SignedDataOutput {
    let mut shares = Vec::with_capacity(endpoint_responses.len());
    for r in endpoint_responses {
        shares.push(r.signature_share);
    }

    combine_and_verify_signature_shares(&shares).unwrap()
}
