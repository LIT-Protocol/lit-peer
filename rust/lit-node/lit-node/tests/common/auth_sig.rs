use ethers::abi::AbiEncode;
use std::collections::{BTreeMap, HashMap};
use std::ops::Add;
use std::str::FromStr;

use anyhow::Result;
use chrono::{Duration, SecondsFormat};
use ed25519_dalek::Signer;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::{LocalWallet, Wallet};
use ethers::types::{Address, H256, U256};
use lit_api_core::error::Unexpected;
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::config::LitConfig;
use lit_node::models::auth::SessionKeySignedMessageV2;
use lit_node::payment::payed_endpoint::PayedEndpoint;
use lit_node::utils::encoding::{self, hex_to_bytes};
use lit_node_core::{
    AuthMethod, AuthSigItem, CurveType, JsonAuthSig, LitResourceAbilityRequest, LitResourcePrefix,
    NodeSet,
    constants::{AUTH_SIG_DERIVED_VIA_SESSION_SIG, AUTH_SIG_SESSION_SIG_ALGO},
    response::JsonSignSessionKeyResponseV2,
};
use lit_rust_crypto::blsful::{Bls12381G2Impl, PublicKey, Signature, SignatureShare};
use serde_json::Value;
use siwe::Message;
use siwe_recap::Capability;
use std::ops::Sub;
use tracing::info;

use crate::common::session_sigs::NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE;
use ethers::prelude::rand::rngs::OsRng as EthersOsRng;
use ethers::signers::Signer as WalletSigner;
use ethers::utils::to_checksum;
use lit_node_core::request::JsonSignSessionKeyRequestV2;
use lit_node_core::response::GenericResponse;
use rand_core::RngCore;

use super::session_sigs::SessionSigAndNodeSet;
use lit_node_testnet::node_collection::NodeIdentityKey;
use lit_rust_crypto::k256;
use lit_sdk::UrlPrefix;

pub fn node_wallet(cfg: &LitConfig) -> Result<Wallet<SigningKey>> {
    let secret_key = SigningKey::from_bytes(k256::FieldBytes::from_slice(
        &hex_to_bytes(cfg.blockchain_wallet_private_key(None)?)
            .expect_or_err("Failed to hex encode node.private_key config var")?,
    ))
    .expect_or_err("Could not convert node.private_key config var to SigningKey")?;
    let chain_id = cfg.blockchain_chain_id()?;
    Ok(LocalWallet::from(secret_key).with_chain_id(chain_id)) // if you don't use this with_chain_id() function, you will get an error when you try to sign a txn.
}

pub async fn generate_authsig_item(wallet: &Wallet<SigningKey>) -> Result<AuthSigItem> {
    let auth_sig = generate_authsig(wallet).await?;
    Ok(AuthSigItem::Single(auth_sig))
}

/// handy function, but probably belongs elsewhere!
pub async fn generate_authsig(wallet: &Wallet<SigningKey>) -> Result<JsonAuthSig> {
    let chain_id = wallet.chain_id();
    let address = to_checksum(&wallet.address(), None);
    let now = chrono::Utc::now();
    let issue_datetime = now.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expiration_datetime = now
        .add(Duration::days(1))
        .to_rfc3339_opts(SecondsFormat::Millis, true);
    let message = format!(
        "localhost wants you to sign in with your Ethereum account:
{}

This is a key for a Lit Action Test.

URI: https://localhost/
Version: 1
Chain ID: {}
Nonce: 1LF00rraLO4f7ZSIt
Issued At: {}
Expiration Time: {}",
        address, chain_id, issue_datetime, expiration_datetime
    );

    // Sign the message
    let signature = wallet.sign_message(&message).await?;
    let auth_sig = JsonAuthSig::new(
        format!("0x{:}", &signature.to_string()),
        "web3.eth.personal.sign".to_string(),
        message,
        address.clone(),
        None,
    );

    info!(
        "Successfully generated a SIWE compatible authSig for using wallet address: {}",
        address
    );

    Ok(auth_sig)
}

#[tokio::test]
async fn test_generate_authsig() {
    super::setup_logging();

    unsafe {
        std::env::set_var("LIT_CONFIG_FILE", "./config/test/lit_sig_cfg.toml");
    }
    let cfg = lit_node_common::config::load_cfg().expect("failed to load LitConfig");
    let loaded_config = &cfg.load_full();
    let wallet = node_wallet(loaded_config).expect("failed to get node wallet");
    let auth_sig = generate_authsig(&wallet).await;

    assert!(auth_sig.is_ok());

    let auth_sig = auth_sig.unwrap();

    // check the SIWE message format.
    let siwe_message = auth_sig.signed_message;
    info!("siwe_message: {}", &siwe_message);
    let message = Message::from_str(&siwe_message);

    if message.is_err() {
        let err = message.err().unwrap();
        error!("Error: {:?}", err);
        assert!(false);
    } else {
        assert!(message.is_ok());
    }
}

pub fn get_auth_sig_with_payment_resources(
    delegator_wallet: &Wallet<SigningKey>,
    delegate_to: &str,
    max_price: U256,
    scopes: Vec<PayedEndpoint>,
) -> JsonAuthSig {
    // Delegation related params
    let mut notabene = BTreeMap::new();
    notabene.insert(
        "delegate_to".to_string(),
        Value::from(vec![Value::from(delegate_to)]),
    );
    notabene.insert("max_price".to_string(), Value::from(max_price.encode_hex()));
    notabene.insert(
        "scopes".to_string(),
        Value::from(
            scopes
                .iter()
                .map(|s| Value::from(s.as_str()))
                .collect::<Vec<_>>(),
        ),
    );

    // Standard AuthSig entries
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let mut capabilities = Capability::<Value>::default();
    let resource = "Auth/Auth".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PD);
    let capabilities = capabilities
        .with_actions_convert(resource_prefix, [(resource, [notabene])])
        .unwrap();
    let siwe_message = capabilities
        .build_message(Message {
            domain: "example.com".parse().unwrap(),
            address: delegator_wallet.address().into(),
            statement: None,
            uri: "lit:capability:delegation".parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "mynonce1".into(),
            issued_at: siwe_issued_at
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .parse()
                .unwrap(),
            expiration_time: Some(
                siwe_expiration_time
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
                    .parse()
                    .unwrap(),
            ),
            not_before: None,
            request_id: None,
            resources: vec![],
        })
        .unwrap();

    let sig = delegator_wallet
        .sign_hash(H256::from(siwe_message.eip191_hash().unwrap()))
        .expect("Could not parse sig");

    JsonAuthSig::new(
        sig.to_string(),
        "web3.eth.personal.sign".to_string(),
        siwe_message.to_string(),
        encoding::bytes_to_hex(delegator_wallet.address()),
        None,
    )
}

/// Get an auth sig that can be used for generating session sigs.
pub fn get_auth_sig_for_session_sig(
    session_pub_key: String,
    auth_sig_wallet: Option<Wallet<SigningKey>>,
    resource_ability_requests: &Vec<LitResourceAbilityRequest>,
) -> JsonAuthSig {
    // Generate new wallet if one is not provided
    let wallet = auth_sig_wallet.unwrap_or(LocalWallet::new(&mut EthersOsRng));

    let mut capabilities = Capability::<Value>::default();

    for resource_ability in resource_ability_requests.iter() {
        let (resource, resource_prefix) = (
            "*/*".to_string(),
            format!("{}://*", resource_ability.resource.resource_prefix.clone()),
        );

        let _ = capabilities.with_actions_convert(resource_prefix, [(resource, [])]);
    }

    // Generate a SIWE message.
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let siwe_message = capabilities
        .build_message(Message {
            domain: "localhost:3000".parse().unwrap(),
            address: wallet.address().into(),
            statement: Some(r#"Some custom statement. "#.into()),
            uri: format!("lit:session:{}", session_pub_key).parse().unwrap(),
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: "JIsknRumpxsM9pqmc".into(),
            issued_at: siwe_issued_at
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .parse()
                .unwrap(),
            expiration_time: Some(
                siwe_expiration_time
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
                    .parse()
                    .unwrap(),
            ),
            not_before: None,
            request_id: None,
            resources: vec![],
        })
        .expect("Could not create SIWE");

    // Sign SIWE message with local wallet.
    let sig = wallet.sign_hash(H256::from(siwe_message.eip191_hash().unwrap()));
    JsonAuthSig::new(
        sig.expect("Could not parse sig").to_string(),
        "web3.eth.personal.sign".to_string(),
        siwe_message.to_string(),
        encoding::bytes_to_hex(wallet.address()),
        None,
    )
}

/// Get session sigs that can be used for auth.
pub fn get_session_sigs_for_auth(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    resource_ability_requests: Vec<LitResourceAbilityRequest>,
    auth_sig_wallet: Option<Wallet<SigningKey>>,
    additional_capabilities: Option<Vec<JsonAuthSig>>,
    max_price: Option<U256>,
) -> Vec<SessionSigAndNodeSet> {
    // Generate ed25519 keypair for signing.
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    // Sign SIWE first.
    let auth_sig = get_auth_sig_for_session_sig(
        session_pub_key.clone(),
        auth_sig_wallet,
        &resource_ability_requests,
    );

    // Generate messages to sign by session key.
    let now = chrono::Utc::now();
    let session_sig_issued_at = now.sub(Duration::days(1));
    let session_sig_expiration_time = now.add(Duration::days(1));

    let mut session_sigs_and_node_set: Vec<SessionSigAndNodeSet> = vec![];

    let mut capabilities = vec![auth_sig.clone()];
    if let Some(additional_capabilities) = additional_capabilities {
        capabilities.extend(additional_capabilities);
    }

    for (node, &identity_key) in node_set {
        let session_key_signed_message = SessionKeySignedMessageV2 {
            session_key: session_pub_key.clone(),
            resource_ability_requests: resource_ability_requests.clone(),
            capabilities: capabilities.clone(),
            issued_at: session_sig_issued_at.to_rfc3339_opts(SecondsFormat::Millis, true),
            expiration: session_sig_expiration_time.to_rfc3339_opts(SecondsFormat::Millis, true),
            node_address: node.socket_address.clone(),
            max_price: max_price.unwrap_or(U256::MAX),
        };

        let message = serde_json::to_string(&session_key_signed_message).unwrap();

        // Sign message with session key.
        let signature = signing_key.sign(message.as_bytes());

        session_sigs_and_node_set.push(SessionSigAndNodeSet {
            session_sig: JsonAuthSig::new(
                signature.to_string(),
                AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                message,
                session_pub_key.clone(),
                Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
            ),
            node: node.clone(),
            identity_key,
        });
    }

    session_sigs_and_node_set
}

pub async fn get_session_delegation_sig_for_pkp(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    auth_sig: &JsonAuthSig,
    pkp_pubkey: String,
    pkp_eth_address: Address,
    session_pub_key: String,
    resource_ability_requests: &Vec<LitResourceAbilityRequest>,
    code: Option<String>,
    js_params: Option<Value>,
    epoch: u64,
) -> Result<JsonAuthSig> {
    // create auth sig using /web/sign_session_key
    let eth_address = pkp_eth_address
        .as_bytes()
        .try_into()
        .expect("Expected an array of length 20");

    let mut resources = vec![];
    let mut resource_prefixes = vec![];

    for resource_ability_request in resource_ability_requests.iter() {
        let (resource, resource_prefix) = (
            "*/*".to_string(),
            format!(
                "{}://*",
                resource_ability_request.resource.resource_prefix.clone()
            ),
        );

        resources.push(resource);
        resource_prefixes.push(resource_prefix);
    }

    let responses = get_auth_sig_for_session_sig_from_nodes(
        node_set,
        &AuthSigItem::Single(auth_sig.clone()),
        false,
        &eth_address,
        &pkp_pubkey,
        session_pub_key,
        resources,
        resource_prefixes,
        code,
        js_params,
        epoch,
    )
    .await?;

    let parsed_responses = responses
        .into_iter()
        .map(|result| {
            assert!(result.ok);
            result.data.unwrap()
        })
        .collect::<Vec<_>>();

    let one_response_with_share = parsed_responses[0].clone();

    let shares = parsed_responses
        .iter()
        .map(|response| response.signature_share)
        .collect::<Vec<SignatureShare<Bls12381G2Impl>>>();

    let signature = Signature::from_shares(&shares)?;

    let bls_root_key = PublicKey::<Bls12381G2Impl>::try_from(
        &hex::decode(&one_response_with_share.bls_root_pubkey).expect("Failed to decode root key"),
    )
    .expect("Failed to convert bls public key from bytes");
    signature
        .verify(
            &bls_root_key,
            hex::decode(&one_response_with_share.data_signed)
                .expect("Could not decode data_signed")
                .as_slice(),
        )
        .expect("Failed to verify signature");

    let serialized_signature = match serde_json::to_string(&signature) {
        Ok(s) => s,
        Err(e) => panic!("Failed to serialize signature: {:?}", e),
    };

    Ok(JsonAuthSig::new(
        serialized_signature,
        "lit.bls".to_string(),
        one_response_with_share.siwe_message.clone(),
        encoding::bytes_to_hex(eth_address),
        Some("LIT_BLS".to_string()),
    ))
}

/// Get session sigs that can be used for auth.
pub async fn get_session_sigs_and_node_set_for_pkp(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    pkp_pubkey: String,
    pkp_eth_address: Address,
    resource_ability_requests: Vec<LitResourceAbilityRequest>,
    auth_sig_wallet: Wallet<SigningKey>,
    additional_capabilities: Option<Vec<JsonAuthSig>>,
    code: Option<String>,
    js_params: Option<Value>,
    epoch: u64,
    max_price: Option<U256>,
) -> Result<Vec<SessionSigAndNodeSet>> {
    // Generate ed25519 keypair for signing.
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    let pkp_owner_auth_sig = generate_authsig(&auth_sig_wallet).await?;

    // Sign SIWE first.
    let delegation_auth_sig = get_session_delegation_sig_for_pkp(
        node_set,
        &pkp_owner_auth_sig,
        pkp_pubkey,
        pkp_eth_address,
        session_pub_key.clone(),
        &resource_ability_requests,
        code,
        js_params,
        epoch,
    )
    .await?;

    // Generate message to sign by session key.
    let now = chrono::Utc::now();
    let session_sig_issued_at = now.sub(Duration::days(1));
    let session_sig_expiration_time = now.add(Duration::days(1));

    let mut session_sigs_and_node_set: Vec<SessionSigAndNodeSet> = vec![];

    let mut capabilities = vec![delegation_auth_sig.clone()];
    if let Some(additional_capabilities) = additional_capabilities {
        capabilities.extend(additional_capabilities);
    }

    for (node, identity_key) in node_set {
        let session_key_signed_message = SessionKeySignedMessageV2 {
            session_key: session_pub_key.clone(),
            resource_ability_requests: resource_ability_requests.clone(),
            capabilities: capabilities.clone(),
            issued_at: session_sig_issued_at.to_rfc3339_opts(SecondsFormat::Millis, true),
            expiration: session_sig_expiration_time.to_rfc3339_opts(SecondsFormat::Millis, true),
            node_address: node.socket_address.to_owned(),
            max_price: max_price.unwrap_or(U256::MAX),
        };

        let message = serde_json::to_string(&session_key_signed_message)?;

        // Sign message with session key.
        let signature = signing_key.sign(message.as_bytes());

        session_sigs_and_node_set.push(SessionSigAndNodeSet {
            session_sig: JsonAuthSig::new(
                signature.to_string(),
                AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                message,
                session_pub_key.clone(),
                Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
            ),
            node: node.clone(),
            identity_key: *identity_key,
        });
    }

    Ok(session_sigs_and_node_set)
}

pub async fn get_auth_sig_for_session_sig_from_nodes(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    auth_sig: &AuthSigItem,
    pass_auth_sig: bool,
    eth_address: &[u8; 20],
    pubkey: &str,
    session_pub_key: String,
    resources: Vec<String>,
    resource_prefixes: Vec<String>,
    code: Option<String>,
    js_params: Option<Value>,
    epoch: u64,
) -> Result<Vec<GenericResponse<JsonSignSessionKeyResponseV2>>> {
    let results = lit_sdk::HandshakeRequest::new()
        .node_set_from_iter(node_set.iter().map(|(n, _)| n))
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .challenge("0x1234123412341234123412341234123412341234123412341234123412341234".to_string())
        .client_public_key("blah".to_string())
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();

    // Get latest blockhash for the nonce
    let responses = results
        .results()
        .into_iter()
        .map(|result| {
            assert!(result.ok);
            result.data.as_ref().unwrap().to_owned()
        })
        .collect::<Vec<_>>();

    let latest_blockhash = &responses[0].latest_blockhash;

    let mut capabilities = Capability::<Value>::default();

    for (resource, resource_prefix) in resources.iter().zip(resource_prefixes.iter()) {
        let _ =
            capabilities.with_actions_convert(resource_prefix.clone(), [(resource.clone(), [])]);
    }

    // Generate a SIWE message.
    let now = chrono::Utc::now();
    let siwe_issued_at = now.sub(Duration::days(1));
    let siwe_expiration_time = now.add(Duration::days(7));
    let siwe_message = capabilities
        .build_message(Message {
            domain: "localhost:3000".parse()?,
            address: *eth_address,
            statement: Some(r#"I am delegating to a session key"#.into()),
            uri: format!("lit:session:{}", session_pub_key).parse()?,
            version: siwe::Version::V1,
            chain_id: 1,
            nonce: latest_blockhash.to_string(),
            issued_at: siwe_issued_at
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .parse()?,
            expiration_time: Some(
                siwe_expiration_time
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
                    .parse()?,
            ),
            not_before: None,
            request_id: None,
            resources: vec![],
        })
        .expect("Could not create SIWE");

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );
    let is_testing_without_auth_method = code == Some(session_sig_lit_action_code);

    let nodes = node_set
        .iter()
        .map(|(node, _)| node.clone())
        .collect::<Vec<NodeSet>>();
    let signing_request = JsonSignSessionKeyRequestV2 {
        auth_sig: if pass_auth_sig {
            Some(auth_sig.clone())
        } else {
            None
        },
        session_key: format!("lit:session:{}", session_pub_key).parse()?,
        auth_methods: if is_testing_without_auth_method {
            vec![]
        } else {
            vec![AuthMethod {
                auth_method_type: 1,
                access_token: serde_json::to_string(&auth_sig)?,
            }]
        },
        pkp_public_key: Some(pubkey.to_string()),
        siwe_message: siwe_message.to_string(),
        curve_type: CurveType::BLS,
        code,
        lit_action_ipfs_id: None,
        js_params,
        epoch,
        node_set: nodes,
        max_price: U256::MAX,
    };

    let mut secret_key = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut secret_key);
    let response = lit_sdk::SignSessionKeyRequest::new()
        .request(signing_request)
        .node_set_from_iter(node_set.iter())
        .url_prefix(UrlPrefix::Http)
        .build()
        .unwrap()
        .send(&secret_key)
        .await
        .unwrap();
    Ok(response.results().to_owned())
}
