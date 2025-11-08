use anyhow::Result;
use ethers::utils::keccak256;
use lit_node_core::SigningScheme;
use lit_node_core::request::JsonPKPSigningRequest;
use lit_node_core::response::GenericResponse;
use lit_node_core::response::JsonPKPSigningResponse;
use lit_node_core::{AuthMethod, AuthSigItem, JsonAuthSig, NodeSet};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::NodeIdentityKey;
use lit_node_testnet::{TestSetupBuilder, testnet::Testnet, validator::ValidatorCollection};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const INVALID_SESSION_SIG_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
})();"#;

pub const VALID_SESSION_SIG_LIT_ACTION_CODE: &str = r#"
    // Works with an AuthSig AuthMethod
    if (Lit.Auth.authMethodContexts.some(e => e.authMethodType === 1)) {
        LitActions.setResponse({response:"true"});
    } else {
        LitActions.setResponse({response:"false"});
    }
"#;

pub const CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE: &str = r#"
    if (Lit.Auth.authMethodContexts.some(e => e.authMethodType === 1)) {
        // Adds the Custom Auth Resource in the SessionSigs
        LitActions.setResponse({response:"(true, 'Anything your want to use in executeJs')"});
    } else {
        LitActions.setResponse({response:"false"});
    }
"#;

pub const VALID_PKP_SIGNING_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    console.log("Lit.Auth", Lit.Auth);

    // Signs only when the sessionSig was created by the below Custom Lit Action Authentication
    if (Lit.Auth.actionIpfsIdStack.includes("QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz")) {
        const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
    }
})();
"#;

pub const CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    console.log("Lit.Auth-", Lit.Auth);
    const isValidCustomAuthResource = Lit.Auth.customAuthResource === "(true, 'Anything your want to use in executeJs')";
    console.log("isValidCustomAuthResource-", isValidCustomAuthResource);

    // Checks the custom auth resource returned in the SessionSigs
    if (Lit.Auth.actionIpfsIdStack.includes("QmRxUzYX52zEko9nvvtkdA6k8jU36enwwTVgW9ZwbdsUHY") && isValidCustomAuthResource) {
        console.log("Custom Authorization Successful!");
        const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
    }
})();
"#;

pub const NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE: &str = r#"
    if (customAccessToken === 'lit') {
        LitActions.setResponse({response:"true"});
    }
"#;

pub const MGB_PKP_SESSION_SIG_LIT_ACTION_CODE: &str =
    r#"LitActions.setResponse({response:"true"});"#;

pub const NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    console.log("Lit.Auth", Lit.Auth);
    // Signs only when the sessionSig was created by the below Custom Lit Action Authentication
    if (Lit.Auth.actionIpfsIdStack.includes("QmWLP9ojXrHJrFHnvMJv12HScFoz7R8kcYAECjtcpaJM2Y")) {
        const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
    }
})();
"#;

pub const SIGN_ECDSA_LIT_ACTION_CODE: &str = r#"(async () => {
    let utf8Encode = new TextEncoder();
    const toSign = utf8Encode.encode('This message is exactly 32 bytes');
    const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
})();
"#;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSigAndNodeSet {
    pub session_sig: JsonAuthSig,
    pub node: NodeSet,
    pub identity_key: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningResponse {
    pub success: bool,
    pub signed_data: JsonSignSessionKeyResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponse {
    pub session_sig: JsonSignSessionKeyResponseShare,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyResponseShare {
    pub signature_share: String,
    pub share_index: u32,
    pub sig_type: String,
    pub siwe_message: String,
    pub data_signed: String,
    pub bigr: String,
    pub public_key: String,
    pub sig_name: String,
}

pub async fn init_test() -> (Testnet, ValidatorCollection, EndUser) {
    crate::common::setup_logging();
    let (testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    end_user.deposit_to_first_pkp_ledger_default().await;
    (testnet, validator_collection, end_user)
}

pub async fn get_pkp_sign(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    session_sigs_and_node_set: Option<Vec<SessionSigAndNodeSet>>,
    auth_sig: Option<AuthSigItem>,
    pass_as_auth_method: bool,
    to_sign: String,
    pubkey: String,
) -> Result<Vec<GenericResponse<JsonPKPSigningResponse>>> {
    let nodes = node_set
        .iter()
        .map(|(node_set, _)| node_set.clone())
        .collect::<Vec<NodeSet>>();
    if let Some(session_sigs_and_node_set) = session_sigs_and_node_set {
        let my_secret_key = rand::rngs::OsRng.r#gen();
        let response = lit_sdk::PKPSigningRequest::new()
            .url_prefix(lit_sdk::UrlPrefix::Http)
            .node_set(
                session_sigs_and_node_set
                    .iter()
                    .map(|session_sig_and_node_set| {
                        let data_to_send = JsonPKPSigningRequest {
                            auth_sig: AuthSigItem::Single(
                                session_sig_and_node_set.session_sig.clone(),
                            ),
                            to_sign: keccak256(to_sign.as_bytes()).into(),
                            pubkey: pubkey.clone(),
                            auth_methods: None,
                            signing_scheme: SigningScheme::EcdsaK256Sha256,
                            epoch: 2, // Hardcoded as at other places in the tests
                            node_set: nodes.clone(),
                        };

                        // json_body_vec.push(json_body);
                        lit_sdk::EndpointRequest {
                            identity_key: session_sig_and_node_set.identity_key,
                            node_set: session_sig_and_node_set.node.clone(),
                            body: data_to_send,
                        }
                    })
                    .collect(),
            )
            .build()
            .unwrap()
            .send(&my_secret_key)
            .await
            .unwrap();

        return Ok(response.results().to_owned());
    }

    if let Some(auth_sig) = auth_sig {
        let mut auth_methods = None;
        if pass_as_auth_method {
            auth_methods = Some(vec![AuthMethod {
                auth_method_type: 1,
                access_token: serde_json::to_string(&auth_sig)?,
            }]);
        }

        let data_to_send = JsonPKPSigningRequest {
            auth_sig,
            to_sign: keccak256(to_sign.as_bytes()).into(),
            pubkey: pubkey.clone(),
            auth_methods,
            signing_scheme: SigningScheme::EcdsaK256Sha256,
            epoch: 2, // Hardcoded as at other places in the tests
            node_set: nodes.clone(),
        };
        let my_secret_key = rand::rngs::OsRng.r#gen();
        let responses = lit_sdk::PKPSigningRequest::new()
            .url_prefix(lit_sdk::UrlPrefix::Http)
            .node_set(
                node_set
                    .iter()
                    .map(|(node, key)| lit_sdk::EndpointRequest {
                        node_set: node.clone(),
                        identity_key: *key,
                        body: data_to_send.clone(),
                    })
                    .collect(),
            )
            .build()
            .unwrap()
            .send(&my_secret_key)
            .await
            .unwrap();

        return Ok(responses.results().to_owned());
    }

    Err(anyhow::anyhow!("Provide either an AuthSig or SessionSigs"))
}
