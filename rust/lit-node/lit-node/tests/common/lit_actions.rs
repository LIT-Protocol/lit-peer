use crate::common::auth_sig::get_session_sigs_for_auth;
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::Wallet;
use ethers::types::U256;
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::{NodeIdentityKey, get_identity_pubkeys_from_node_set};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::testnet::actions::Actions;
use lit_node_testnet::validator::ValidatorCollection;

use super::session_sigs::SessionSigAndNodeSet;
use lazy_static::lazy_static;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_node::pkp::utils::pkp_permissions_get_permitted;
use lit_node_core::{
    AuthMethod, AuthSigItem, Invocation, LitAbility, LitResourceAbilityRequest,
    LitResourceAbilityRequestResource, LitResourcePrefix, NodeSet, SignableOutput, SigningScheme,
    request::JsonExecutionRequest,
    response::{GenericResponse, JsonExecutionResponse},
};
use lit_rust_crypto::{k256, p256, p384};
use rand::Rng;
use rand_core::OsRng;
use std::collections::HashMap;
use tracing::{error, info};

pub const HELLO_WORLD_LIT_ACTION_CODE: &str = "const go = async () => {
  // this requests a signature share from the Lit Node
  // the signature share will be automatically returned in the response from the node
  // and combined into a full signature by the LitJsSdk for you to use on the client
  // all the params (toSign, publicKey, sigName) are passed in from the LitJsSdk.executeJs() function

  let utf8Encode = new TextEncoder();
  const toSign = utf8Encode.encode('This message is exactly 32 bytes');
  const sigShare = await Lit.Actions.signEcdsa({ toSign, publicKey, sigName });
};
go();";

const CALL_CHILD_LIT_ACTION_CODE: &str = "const go = async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    const _ = await Lit.Actions.call({ ipfsId: 'QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm', params: {
        toSign: Array.from(toSign),
        publicKey,
        sigName
    }});
  };
  go();";

lazy_static! {
    static ref CONTRACT_CALL_LIT_ACTION_CODE: String = r#"
            const go = async () => {
                // https://sepolia.etherscan.io/address/0xD2f13AeACd77bB8D0aD79c6dB5F081e358b481C2#code
                const toContract = "0xD2f13AeACd77bB8D0aD79c6dB5F081e358b481C2";

                const abi = [{"inputs":[],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[{"internalType":"uint256","name":"a","type":"uint256"},{"internalType":"uint256","name":"b","type":"uint256"}],"name":"add","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"}];

                const contract = new ethers.Contract(toContract, abi);
                const rawTxn = await contract.populateTransaction.add(1,2);
                const txn = ethers.utils.serializeTransaction(rawTxn);
                
                const chain = "sepolia";

                const res = await LitActions.callContract({
                    chain,
                    txn
                });

                // decode response
                const decodedResult = contract.interface.decodeFunctionResult("add", res)[0].toString();

                Lit.Actions.setResponse({response: decodedResult});
            };
            go();
            "#.to_string();
}

pub const INFINITE_LOOP_LIT_ACTION_CODE: &str = "const go = async () => {
  while (true) {}
};
go();";

pub const INVALID_LIT_ACTION_CODE: &str = "const go = async () => {
  Invalid Code
};
go();";

pub async fn lit_action_params(
    lit_action_code: String,
    pubkey: String,
) -> Result<(
    String,
    Option<String>,
    Option<serde_json::Value>,
    Option<Vec<AuthMethod>>,
)> {
    let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.into());
    js_params.insert("sigName".to_string(), "sig1".into());

    Ok((
        lit_action_code,
        None,
        Some(serde_json::Value::Object(js_params)),
        None,
    ))
}

pub async fn sign_using_child_lit_action(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) -> Result<bool> {
    let actions = validator_collection.actions();

    let wallet = end_user.signing_provider().clone();

    let lit_action_code = CALL_CHILD_LIT_ACTION_CODE.to_string();

    let (pubkey, _token_id, _eth_address) = end_user.first_pkp().info();

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        lit_action_params(lit_action_code, pubkey).await?;

    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let realm_id = U256::from(1);
    let epoch = actions.get_current_epoch(realm_id).await.as_u64();

    let execute_resp = generate_session_sigs_and_execute_lit_action(
        &node_set,
        wallet.signer().clone(),
        Some(lit_action_code),
        ipfs_id,
        js_params,
        auth_methods,
        epoch,
    )
    .await?;

    assert_signed_action(validator_collection, execute_resp).await
}

pub async fn sign_from_file_system(
    validator_collection: &ValidatorCollection,
    testnet: &Testnet,
    end_user: &EndUser,
    file_name: &str,
    assert_sig: bool,
) -> Result<bool> {
    let path = std::path::Path::new(file_name);

    if !path.exists() {
        error!("File does not exist: {}", file_name);
        return Ok(false);
    }

    let realm_id = U256::from(1);
    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm_id)
        .await
        .as_u64();
    let node_set = &validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    // let node_set = &validator_collection.complete_node_set();

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        prepare_sign_from_file_parameters(end_user, file_name).await?;

    let wallet = testnet.deploy_account.signing_provider.signer();

    let execute_resp = generate_session_sigs_and_execute_lit_action(
        &node_set,
        wallet.clone(),
        Some(lit_action_code),
        ipfs_id,
        js_params,
        auth_methods,
        epoch,
    )
    .await?;

    match assert_sig {
        true => assert_signed_action(validator_collection, execute_resp).await,
        false => Ok(true),
    }
}

pub async fn generate_session_sigs_and_execute_lit_action(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    wallet: Wallet<SigningKey>,
    lit_action_code: Option<String>,
    ipfs_id: Option<String>,
    js_params: Option<serde_json::Value>,
    auth_methods: Option<Vec<AuthMethod>>,
    epoch: u64,
) -> Result<Vec<GenericResponse<JsonExecutionResponse>>> {
    info!("lit_action_code: {:?}", lit_action_code);
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        node_set,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        Some(wallet),
        None,
        None,
    );

    let execute_resp = execute_lit_action_session_sigs(
        lit_action_code,
        ipfs_id,
        js_params,
        auth_methods,
        &session_sigs_and_node_set,
        epoch,
    )
    .await;
    debug!("execute_resps: {:?}", execute_resp);
    execute_resp
}

pub async fn execute_lit_action_session_sigs(
    lit_action_code: Option<String>,
    ipfs_id: Option<String>,
    js_params: Option<serde_json::Value>,
    auth_methods: Option<Vec<AuthMethod>>,
    session_sigs_and_node_set: &[SessionSigAndNodeSet],
    epoch: u64,
) -> Result<Vec<GenericResponse<JsonExecutionResponse>>> {
    info!("executing lit action with session sigs");
    // Generate JSON body for each port
    let nodes = session_sigs_and_node_set
        .iter()
        .map(|sig_and_nodeset| sig_and_nodeset.node.clone())
        .collect::<Vec<_>>();
    let my_private_key = OsRng.r#gen();
    let response = lit_sdk::ExecuteFunctionRequest::new()
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .node_set(
            session_sigs_and_node_set
                .iter()
                .map(|sig_and_nodeset| {
                    let execute_request = JsonExecutionRequest {
                        auth_sig: AuthSigItem::Single(sig_and_nodeset.session_sig.clone()),
                        code: lit_action_code.clone(),
                        ipfs_id: ipfs_id.clone(),
                        js_params: Some(js_params.clone().unwrap_or_default()),
                        auth_methods: auth_methods.clone(),
                        epoch,
                        node_set: nodes.clone(),
                        invocation: Invocation::Sync,
                    };
                    lit_sdk::EndpointRequest {
                        node_set: sig_and_nodeset.node.clone(),
                        identity_key: sig_and_nodeset.identity_key,
                        body: execute_request,
                    }
                })
                .collect::<Vec<_>>(),
        )
        .build()?
        .send(&my_private_key)
        .await?;
    Ok(response.results().to_owned())
}

pub async fn prepare_sign_from_file_parameters(
    end_user: &EndUser,
    file_name: &str,
) -> Result<(
    String,
    Option<String>,
    Option<serde_json::Value>,
    Option<Vec<AuthMethod>>,
)> {
    info!("Attempting to run lit action from file: {}", file_name);
    let lit_action_code = std::fs::read_to_string(file_name)?;

    let (pubkey, _token_id, _eth_address) = end_user.first_pkp().info();

    Ok(lit_action_params(lit_action_code, pubkey).await?)
}

pub async fn execute_lit_action_auth_sig(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    lit_action_code: Option<String>,
    ipfs_id: Option<String>,
    js_params: Option<serde_json::Value>,
    auth_methods: Option<Vec<AuthMethod>>,
    auth_sig_item: AuthSigItem,
    epoch: u64,
) -> Vec<GenericResponse<JsonExecutionResponse>> {
    let execute_request = JsonExecutionRequest {
        auth_sig: auth_sig_item,
        code: lit_action_code,
        ipfs_id,
        js_params,
        auth_methods,
        epoch,
        node_set: node_set.iter().map(|(n, _)| n.clone()).collect(),
        invocation: Invocation::Sync,
    };
    let my_private_key = OsRng.r#gen();
    let response = lit_sdk::ExecuteFunctionRequest::new()
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .node_set(
            node_set
                .iter()
                .map(|(node_set, identity_key)| lit_sdk::EndpointRequest {
                    node_set: node_set.clone(),
                    identity_key: *identity_key,
                    body: execute_request.clone(),
                })
                .collect::<Vec<_>>(),
        )
        .build()
        .unwrap()
        .send(&my_private_key)
        .await
        .unwrap();

    response.results().to_owned()
}

pub async fn assert_signed_action(
    validator_collection: &ValidatorCollection,
    execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
) -> Result<bool> {
    let threshold = validator_collection.inferred_threshold();
    info!(
        "Threshold: {}  | responses: {} ",
        threshold,
        execute_resp.len()
    );
    assert_eq!(execute_resp.len(), threshold);

    let mut ecdsa_message_shares = Vec::new();

    info!("execute_resp: {:?}", &execute_resp);

    for r in execute_resp {
        if !r.ok {
            error!(
                "Error parsing response as ActionReturn: {}",
                r.error.as_ref().unwrap()
            );
            return Ok(false);
        }

        let result = r.data.as_ref().unwrap();
        info!("Result: {:?}", &result);

        let signed_data = result.signed_data.iter().last().unwrap().1;
        info!("Signed Data: {:?}", &signed_data);

        if !signed_data.signature_share.is_empty() {
            let signable_output: SignableOutput =
                serde_json::from_str(&signed_data.signature_share)?;
            let ecdsa_msg_share = signable_output.ecdsa_signed_message_share()?;
            info!("Ecdsa msg share: {:?}", &ecdsa_msg_share);
            ecdsa_message_shares.push(ecdsa_msg_share);
        }
    }

    assert!(
        ecdsa_message_shares.len() >= threshold,
        "Not enough sig shares.  Got: {} but expected {}",
        ecdsa_message_shares.len(),
        threshold
    );

    info!("Sig ecdsa_message_shares: {:?}", &ecdsa_message_shares);

    match ecdsa_message_shares[0].sig_type.parse() {
        Ok(signing_scheme) => match signing_scheme {
            SigningScheme::EcdsaK256Sha256 => Ok(
                lit_sdk::signature::verify_ecdsa_signing_package::<k256::Secp256k1>(
                    &ecdsa_message_shares,
                )
                .is_ok(),
            ),
            SigningScheme::EcdsaP256Sha256 => Ok(
                lit_sdk::signature::verify_ecdsa_signing_package::<p256::NistP256>(
                    &ecdsa_message_shares,
                )
                .is_ok(),
            ),
            SigningScheme::EcdsaP384Sha384 => Ok(
                lit_sdk::signature::verify_ecdsa_signing_package::<p384::NistP384>(
                    &ecdsa_message_shares,
                )
                .is_ok(),
            ),
            s => {
                panic!("Unsupported signing scheme type: {}", s);
            }
        },
        Err(e) => {
            error!("Error parsing signing scheme: {:?}", e);
            Ok(false)
        }
    }
}

pub async fn generate_pkp_check_get_permitted_pkp_action(
    ipfs_cid: &str,
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) -> Result<(String, Vec<serde_json::Value>)> {
    let config_file = validator_collection.config_files()[0].clone();

    unsafe {
        std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);
    }

    let cfg = lit_node_common::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let (pkp_pubkey, token_id, _) = end_user.first_pkp().info();

    let pkp = end_user.pkp_by_pubkey(pkp_pubkey.clone());
    let res = pkp
        .add_permitted_action_to_pkp(ipfs_cid, &[U256::from(1)])
        .await;

    assert!(res.is_ok());

    let res = pkp_permissions_get_permitted(
        String::from("getPermittedActions"),
        loaded_config.as_ref(),
        token_id.to_string(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Error getting permitted actions: {:?}", e));

    assert!(res.is_ok());
    Ok((pkp_pubkey, res?))
}

pub async fn generate_pkp_check_is_permitted_pkp_action(
    ipfs_cid: &str,
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) -> Result<bool> {
    let config_file = validator_collection.config_files()[0].clone();

    unsafe {
        std::env::set_var(ENV_LIT_CONFIG_FILE, config_file);
    }

    let cfg = lit_node_common::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();

    let (pkp_pubkey, token_id, _) = end_user.first_pkp().info();

    let pkp = end_user.pkp_by_pubkey(pkp_pubkey);
    let res = pkp
        .add_permitted_action_to_pkp(ipfs_cid, &[U256::from(1)])
        .await;
    assert!(res.is_ok());

    let res = lit_node::pkp::utils::pkp_permissions_is_permitted(
        token_id.to_string(),
        loaded_config.as_ref(),
        String::from("isPermittedAction"),
        [serde_json::Value::from(ipfs_cid)].to_vec(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Error getting permitted actions: {:?}", e));

    assert!(res.is_ok());
    res
}

#[cfg(feature = "lit-actions")]
pub async fn add_permitted_action_to_pkp(
    ipfs_cid: &str,
    token_id: U256,
    actions: &Actions,
) -> Result<bool> {
    use ethers::types::Bytes;

    let ipfs_cid = bs58::decode(ipfs_cid).into_vec()?;
    let ipfs_cid = Bytes::from(ipfs_cid);

    info!(
        "adding cid permission to tokenId: {} cid: {}",
        token_id, ipfs_cid
    );

    let mut scopes = Vec::new();
    scopes.push(token_id);

    let pacc = actions
        .contracts()
        .pkp_permissions
        .add_permitted_action(token_id, ipfs_cid, scopes);

    let tx = pacc.send().await;
    if tx.is_err() {
        error!("Error minting PKP: {:?}", tx.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tx = tx.unwrap();

    let tr = tx.await;
    if tr.is_err() {
        error!("Error minting PKP: {:?}", tr.unwrap_err());
        return Err(anyhow::anyhow!("Error minting PKP"));
    }
    let tr = tr?;
    if tr.is_none() {
        error!("Error minting PKP: No transaction receipt?");
        return Err(anyhow::anyhow!("Error minting PKP"));
    }

    Ok(true)
}
