#[allow(deprecated)]
use super::{
    contract::validate_eip1271_signature,
    session_sigs::{
        extract_requested_resources_from_session_sig, extract_wallet_sig,
        validate_and_extract_wallet_sig, validate_session_sig,
    },
    validators::wallet_sig::{validate_siwe_message, validate_wallet_sig_by_chain},
};
use crate::{
    error::{EC, Result, conversion_err, unexpected_err, validation_err, validation_err_code},
    utils::{encoding, web::pubkey_bytes_to_eth_address_bytes},
};
use ethers::types::Address;
use lit_core::{config::LitConfig, error::Unexpected};
use lit_node_common::config::LitNodeConfig as _;
use lit_node_core::{
    AuthMaterialType, AuthSigItem, EndpointVersion, JsonAuthSig, LitResource, LitResourceAbility,
    MultipleAuthSigs,
    constants::{
        CHAIN_CHEQD, CHAIN_COSMOS, CHAIN_ETHEREUM, CHAIN_JUNO, CHAIN_KYVE, CHAIN_SOLANA, Chain,
        is_evm_compatible_chain,
    },
    request::JsonSignSessionKeyRequestV2,
};
use rocket::form::{FromFormField, ValueField};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, form, request};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::sync::Arc;

/// ValidatedAddress is an address that has been validated against the corresponding chain.
///
/// Note: It can only be obtained by calling `validate_and_get_user_address`.
#[derive(Debug)]
pub struct ValidatedAddress {
    address_str: String,
    chain: Chain,
}

impl ValidatedAddress {
    pub(self) fn new(address_str: String, chain: Chain) -> Self {
        Self { address_str, chain }
    }

    pub fn from_signed_session_request(
        signed_session_request: JsonSignSessionKeyRequestV2,
    ) -> Self {
        let pubkey_bytes = match signed_session_request.pkp_public_key {
            Some(pkp_public_key) => encoding::hex_to_bytes(pkp_public_key).unwrap_or(vec![0; 32]),
            None => vec![0; 32],
        };

        let pkp_eth_address = match pubkey_bytes_to_eth_address_bytes(pubkey_bytes) {
            Ok(pkp_eth_address) => encoding::bytes_to_hex(pkp_eth_address),
            Err(..) => encoding::bytes_to_hex(vec![0; 20]),
        };

        Self {
            address_str: pkp_eth_address,
            chain: Chain::Ethereum,
        }
    }

    pub fn address_str(&self) -> &String {
        &self.address_str
    }

    /// Check whether the address is an EVM-compatible address.
    pub fn is_evm_user_address(&self) -> bool {
        is_evm_compatible_chain(&self.chain)
    }

    /// Get the EVM-compatible address.
    ///
    /// Note: This function will return `Err` if the address is not an EVM-compatible address.
    pub fn evm_address(&self) -> Result<Address> {
        if !self.is_evm_user_address() {
            return Err(unexpected_err(
                "Address is not EVM-compatible",
                Some("Unable to get EVM address".into()),
            ));
        }
        Ok(Address::from_slice(
            &encoding::hex_to_bytes(self.address_str()).map_err(|e| {
                conversion_err(e, Some("Error converting hex string to bytes".into()))
            })?,
        ))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthSigItemExtendedRef<'a>(pub &'a AuthSigItem);

impl AuthSigItemExtendedRef<'_> {
    pub(crate) async fn validate_and_get_user_address(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
        chain: &Option<String>,
        cfg: &LitConfig,
        bls_root_pubkey: &str,
        endpoint_version: &EndpointVersion,
    ) -> Result<ValidatedAddress> {
        let valid_wallet_sig = {
            match self.0 {
                AuthSigItem::Multiple(multiple_auth_sigs) => {
                    debug!("multiple sigs in auth_sig_item");
                    MultipleAuthSigsExtendedRef(multiple_auth_sigs)
                        .validate_all_and_extract_single_wallet_sig(
                            requested_lit_resource_ability,
                            cfg,
                            bls_root_pubkey,
                            endpoint_version,
                        )
                        .await
                        .map_err(|e| {
                            validation_err(
                                e,
                                Some("Unable to extract wallet sig from multiple auth sigs".into()),
                            )
                        })?
                }
                AuthSigItem::Single(single_auth_sig) => {
                    debug!("single sig in auth_sig_item");
                    JsonAuthSigExtendedRef(single_auth_sig)
                        .validate_and_get_wallet_sig(
                            requested_lit_resource_ability,
                            chain,
                            cfg,
                            bls_root_pubkey,
                            endpoint_version,
                        )
                        .await
                        .map_err(|e| {
                            validation_err_code(
                                e,
                                EC::NodeInvalidAuthSig,
                                Some("Invalid AuthSig".into()),
                            )
                        })?
                }
            }
        };

        let user_address = JsonAuthSigExtendedRef(&valid_wallet_sig)
            .user_address(bls_root_pubkey)
            .await
            .map_err(|e| {
                unexpected_err(e, Some("Unable to get user address from wallet sig".into()))
            })?;
        let chain = valid_wallet_sig
            .chain
            .ok_or(unexpected_err("Unable to get chain from wallet sig", None))?;
        Ok(ValidatedAddress::new(user_address, chain.to_owned()))
    }

    pub(crate) fn resources(&self) -> Result<Vec<Arc<dyn LitResource>>> {
        match self.0 {
            AuthSigItem::Multiple(multiple_auth_sigs) => {
                MultipleAuthSigsExtendedRef(multiple_auth_sigs).resources()
            }
            AuthSigItem::Single(single_auth_sig) => {
                JsonAuthSigExtendedRef(single_auth_sig).resources()
            }
        }
    }

    pub fn get_auth_type(&self) -> Result<&AuthMaterialType> {
        match self.0 {
            AuthSigItem::Single(json_auth_sig) => Ok(&json_auth_sig.auth_material_type),
            AuthSigItem::Multiple(_) => Err(validation_err_code(
                "Can't pass multiple AuthSigs",
                EC::NodeInvalidMultipleAuthSigs,
                None,
            )),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonAuthSigExtended {
    pub auth_sig: JsonAuthSig,
}

impl From<JsonAuthSigExtended> for lit_node_core::AdminAuthSig {
    fn from(value: JsonAuthSigExtended) -> Self {
        Self {
            auth_sig: value.auth_sig,
        }
    }
}

impl From<JsonAuthSig> for JsonAuthSigExtended {
    fn from(value: JsonAuthSig) -> Self {
        Self { auth_sig: value }
    }
}

impl From<&JsonAuthSig> for JsonAuthSigExtended {
    fn from(value: &JsonAuthSig) -> Self {
        Self {
            auth_sig: value.clone(),
        }
    }
}

impl std::ops::Deref for JsonAuthSigExtended {
    type Target = JsonAuthSig;

    fn deref(&self) -> &Self::Target {
        &self.auth_sig
    }
}

impl std::ops::DerefMut for JsonAuthSigExtended {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.auth_sig
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JsonAuthSigExtendedRef<'a>(pub &'a JsonAuthSig);

impl<'a> From<&'a JsonAuthSig> for JsonAuthSigExtendedRef<'a> {
    fn from(value: &'a JsonAuthSig) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for JsonAuthSigExtendedRef<'_> {
    type Target = JsonAuthSig;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl JsonAuthSigExtended {
    pub fn as_ref(&self) -> JsonAuthSigExtendedRef<'_> {
        JsonAuthSigExtendedRef(self)
    }
}

/// The JsonAuthSigExtended request guard is used to check for the existence of the x-auth-sig header.
/// If it is not present, the request is rejected as unauthorized.
/// If it is present, it is decoded from base64, deserialized as JSON and returned as a JsonAuthSig struct.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for JsonAuthSigExtended {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_sig = match request.headers().get_one("x-auth-sig") {
            Some(auth_sig) => auth_sig,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    json!({"error": "Missing x-auth-sig header"}),
                ));
            }
        };

        // Decode base64.
        let decoded_auth_sig = match data_encoding::BASE64.decode(auth_sig.as_bytes()) {
            Ok(decoded_auth_sig) => decoded_auth_sig,
            Err(e) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    json!({"error": "Unable to decode base64", "reason": e.to_string()}),
                ));
            }
        };

        // Deserialize JSON.
        let deserialized_auth_sig = match serde_json::from_slice::<JsonAuthSig>(&decoded_auth_sig) {
            Ok(deserialized_auth_sig) => deserialized_auth_sig,
            Err(e) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    json!({"error": "Unable to deserialize JSON", "reason": e.to_string()}),
                ));
            }
        };

        Outcome::Success(Self {
            auth_sig: deserialized_auth_sig,
        })
    }
}

impl<'r> FromFormField<'r> for JsonAuthSigExtended {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let v = data_encoding::BASE64
            .decode(field.value.as_bytes())
            .map_err(|e| {
                form::Error::validation(format!("auth field needs to be base64: {:?}", e))
            })?;
        let auth: JsonAuthSigExtended = serde_json::from_slice(&v).map_err(|e| {
            form::Error::validation(format!("auth failed to decode from JSON: {:?}", e))
        })?;

        Ok(auth)
    }
}

impl JsonAuthSigExtendedRef<'_> {
    /// Checks if the user address is an EVM-compatible address.
    ///
    /// NOTE: This should be used after running validation has been run on the auth sig,
    /// eg. with validate_and_get_wallet_sig.
    pub async fn is_evm_user_address(&self, bls_root_pubkey: &String) -> Result<bool> {
        if let Some(chain) = &self.0.chain {
            return Ok(is_evm_compatible_chain(chain));
        }

        if matches!(self.0.auth_material_type, AuthMaterialType::WalletSig)
            || matches!(self.0.auth_material_type, AuthMaterialType::ContractSig)
        {
            // Try to infer the chain from the address by doing hex decode.
            let user_address = self
                .user_address(bls_root_pubkey)
                .await
                .map_err(|e| unexpected_err(e, Some("Unable to get user address".into())))?;
            let user_address = encoding::hex_to_bytes(user_address).map_err(|e| {
                conversion_err(e, Some("Error converting hex string to bytes".into()))
            });
            if user_address.is_err() {
                return Ok(false);
            }

            return Ok(true);
        } else if matches!(self.0.auth_material_type, AuthMaterialType::SessionSig) {
            // Extract the inner wallet sig.
            let inner_wallet_sig = extract_wallet_sig(self.0, bls_root_pubkey).await?;
            return Box::pin(self.is_evm_user_address(bls_root_pubkey)).await;
        }

        Ok(false)
    }

    /// Returns the user address associated with the wallet sig of this auth sig object without performing ALL validation.
    pub async fn user_address(&self, bls_root_pubkey: &str) -> Result<String> {
        match self.0.auth_material_type {
            AuthMaterialType::WalletSig => Ok(self.0.address.clone()),
            AuthMaterialType::BLSNetworkSig => Ok(self.0.address.clone()),
            AuthMaterialType::ContractSig => Ok(self.0.address.clone()),
            AuthMaterialType::SessionSig => {
                Ok(extract_wallet_sig(self.0, bls_root_pubkey).await?.address)
            }
        }
    }

    /// Returns the wallet sig associated with this auth sig object without performing ALL validation.
    pub async fn wallet_sig(&self, bls_root_pubkey: &str) -> Result<JsonAuthSig> {
        match self.0.auth_material_type {
            AuthMaterialType::BLSNetworkSig => Ok(self.0.clone()),
            AuthMaterialType::WalletSig => Ok(self.0.clone()),
            AuthMaterialType::ContractSig => Ok(self.0.clone()),
            AuthMaterialType::SessionSig => extract_wallet_sig(self.0, bls_root_pubkey).await,
        }
    }

    /// Returns the wallet sig associated with this auth sig object after performing validation.
    ///
    /// Validation is done by checking capabilities for performing the requested abilities against the requested
    /// lit resources.
    pub async fn validate_and_get_wallet_sig(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
        chain: &Option<String>,
        cfg: &LitConfig,
        bls_root_pubkey: &str,
        endpoint_version: &EndpointVersion,
    ) -> Result<JsonAuthSig> {
        let mut new_auth_sig = match self.0.auth_material_type {
            AuthMaterialType::WalletSig => {
                validate_wallet_sig_by_chain(self.0, chain, cfg).await?;
                self.0.clone()
            }
            AuthMaterialType::BLSNetworkSig => {
                return Err(validation_err_code(
                    "BLSNetworkSig is not supported for wallet sig validation",
                    EC::NodeInvalidAuthSig,
                    None,
                ));
            }
            AuthMaterialType::ContractSig => {
                let enable_siwe_validation = matches!(cfg.enable_siwe_validation(), Ok(true));
                let _ = validate_siwe_message(self.0, enable_siwe_validation)?;
                validate_eip1271_signature(self.0, chain).await?;
                self.0.clone()
            }
            AuthMaterialType::SessionSig => {
                // TODO: This is only here for backwards compatibility. Once the new implementation on the
                // SDK has been stabilized, we should only use validate_session_sig.
                #[allow(deprecated)]
                if *endpoint_version == EndpointVersion::Initial {
                    if let Ok(valid_auth_sig) = validate_session_sig(
                        self.0,
                        requested_lit_resource_ability,
                        chain,
                        cfg,
                        bls_root_pubkey,
                    )
                    .await
                    {
                        valid_auth_sig
                    } else {
                        validate_and_extract_wallet_sig(self.0)?
                    }
                } else {
                    validate_session_sig(
                        self.0,
                        requested_lit_resource_ability,
                        chain,
                        cfg,
                        bls_root_pubkey,
                    )
                    .await?
                }
            }
        };

        // Set the chain if it's present.
        if let Some(c) = chain {
            new_auth_sig.chain = Some(
                Chain::from_str(c).map_err(|e| unexpected_err("Unable to parse chain", None))?,
            );
            Ok(new_auth_sig)
        } else {
            Ok(new_auth_sig)
        }
    }

    /// Returns the resources this auth sig has requested to operate on without performing any validation.
    pub fn resources(&self) -> Result<Vec<Arc<dyn LitResource>>> {
        match self.0.auth_material_type {
            AuthMaterialType::WalletSig => Ok(vec![]),
            AuthMaterialType::BLSNetworkSig => Ok(vec![]),
            AuthMaterialType::ContractSig => Ok(vec![]),
            AuthMaterialType::SessionSig => extract_requested_resources_from_session_sig(self.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MultipleAuthSigsExtended(pub MultipleAuthSigs);

#[derive(Clone, Copy, Debug)]
pub struct MultipleAuthSigsExtendedRef<'a>(pub &'a MultipleAuthSigs);

impl MultipleAuthSigsExtended {
    /// This validates each auth sig in the MultipleAuthSigs object that is not None.
    pub async fn validate_all_and_extract_wallet_sigs(
        &mut self,
        requested_lit_resource_ability: &LitResourceAbility,
        cfg: &LitConfig,
        bls_root_pubkey: &str,
        endpoint_version: &EndpointVersion,
    ) -> Result<()> {
        if self.0.ethereum.is_none()
            && self.0.solana.is_none()
            && self.0.cosmos.is_none()
            && self.0.kyve.is_none()
            && self.0.cheqd.is_none()
            && self.0.juno.is_none()
        {
            return Err(validation_err_code(
                "No auth sig detected",
                EC::NodeInvalidMultipleAuthSigs,
                None,
            )
            .add_source_to_details());
        }

        if let Some(auth_sig) = &self.0.ethereum {
            self.0.ethereum = Some(
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_ETHEREUM.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.0.solana {
            self.0.solana = Some(
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_SOLANA.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.0.cosmos {
            self.0.cosmos = Some(
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_COSMOS.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.0.kyve {
            self.0.kyve = Some(
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_KYVE.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.0.cheqd {
            self.0.cheqd = Some(
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_CHEQD.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        if let Some(auth_sig) = &self.0.juno {
            self.0.juno = Some(
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(CHAIN_JUNO.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?,
            );
        }

        Ok(())
    }

    pub fn as_ref(&self) -> MultipleAuthSigsExtendedRef<'_> {
        MultipleAuthSigsExtendedRef(&self.0)
    }
}

impl MultipleAuthSigsExtendedRef<'_> {
    /// This validates each auth sig in the MultipleAuthSigs object that is not None, and returns the LAST valid auth sig.
    pub async fn validate_all_and_extract_single_wallet_sig(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
        cfg: &LitConfig,
        bls_root_pubkey: &str,
        endpoint_version: &EndpointVersion,
    ) -> Result<JsonAuthSig> {
        let mut wallet_sig = None;
        for (auth_sig, chain) in [
            (&self.0.ethereum, CHAIN_ETHEREUM),
            (&self.0.solana, CHAIN_SOLANA),
            (&self.0.cosmos, CHAIN_COSMOS),
            (&self.0.kyve, CHAIN_KYVE),
            (&self.0.cheqd, CHAIN_CHEQD),
            (&self.0.juno, CHAIN_JUNO),
        ] {
            if let Some(auth_sig) = auth_sig {
                JsonAuthSigExtendedRef(auth_sig)
                    .validate_and_get_wallet_sig(
                        requested_lit_resource_ability,
                        &Some(chain.into()),
                        cfg,
                        bls_root_pubkey,
                        endpoint_version,
                    )
                    .await?;
                wallet_sig = Some(auth_sig.clone());
            }
        }

        wallet_sig.expect_or_err_code(EC::NodeInvalidMultipleAuthSigs, "No auth sig detected")
    }

    pub fn resources(&self) -> Result<Vec<Arc<dyn LitResource>>> {
        let mut resources = Vec::new();

        for auth_sig in [
            &self.0.ethereum,
            &self.0.solana,
            &self.0.cosmos,
            &self.0.kyve,
            &self.0.cheqd,
            &self.0.juno,
        ]
        .into_iter()
        .flatten()
        {
            resources = JsonAuthSigExtendedRef(auth_sig).resources()?;
        }

        Ok(resources)
    }
}

pub fn siwe_hash_to_bls_session_hash(siwe_hash: Vec<u8>) -> Vec<u8> {
    // for BLS, we don't just sign the raw SIWE message - we add a prefix then hash again.
    // we do this to namespace our signatures since we're using the BLS network key.
    // this is just a precaution to avoid signing the wrong data on the wrong code path
    let prefixed = format!("lit_session:{}", hex::encode(siwe_hash));
    let mut hasher = Sha256::new();
    hasher.update(prefixed);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use lit_node_core::{
        AuthMaterialType, JsonAuthSig,
        constants::{
            AUTH_SIG_DERIVED_VIA_CONTRACT_SIG, AUTH_SIG_DERIVED_VIA_SESSION_SIG,
            AUTH_SIG_SESSION_SIG_ALGO,
        },
    };

    struct SerTestCase {
        input: JsonAuthSig,
        expected: &'static str,
    }

    struct DeserTestCase {
        input: &'static str,
        expected: JsonAuthSig,
    }

    #[test]
    fn test_ser() {
        let test_cases = get_test_ser_test_cases();

        for test_case in test_cases {
            let serialized = serde_json::to_string(&test_case.input).expect("err ser");
            assert_eq!(serialized, test_case.expected);
        }
    }

    #[test]
    fn test_deser() {
        let test_cases = get_test_deser_test_cases();

        for test_case in test_cases {
            let deserialized: JsonAuthSig =
                serde_json::from_str(test_case.input).expect("err deser");
            assert_eq!(deserialized, test_case.expected);
        }
    }

    fn get_test_ser_test_cases() -> Vec<SerTestCase> {
        vec![
            SerTestCase {
                input: JsonAuthSig::new(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                ),
                expected: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address","algo":null}"#,
            },
            SerTestCase {
                input: JsonAuthSig::new(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    Some("algo".into()),
                ),
                expected: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address","algo":"algo"}"#,
            },
        ]
    }

    fn get_test_deser_test_cases() -> Vec<DeserTestCase> {
        vec![
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address"}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                    AuthMaterialType::WalletSig,
                    None,
                ),
            },
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"derived_via","signedMessage":"signed_message","address":"address","algo":null}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    "derived_via".into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                    AuthMaterialType::WalletSig,
                    None,
                ),
            },
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"EIP1271","signedMessage":"signed_message","address":"address","algo":null}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    AUTH_SIG_DERIVED_VIA_CONTRACT_SIG.into(),
                    "signed_message".into(),
                    "address".into(),
                    None,
                    AuthMaterialType::ContractSig,
                    None,
                ),
            },
            DeserTestCase {
                input: r#"{"sig":"sig","derivedVia":"litSessionSignViaNacl","signedMessage":"signed_message","address":"address","algo":"ed25519"}"#,
                expected: JsonAuthSig::new_with_type(
                    "sig".into(),
                    AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                    "signed_message".into(),
                    "address".into(),
                    Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                    AuthMaterialType::SessionSig,
                    None,
                ),
            },
        ]
    }
}

#[cfg(test)]
mod multiple_auth_sigs_tests {
    use crate::models::auth::SessionKeySignedMessageV2;
    use ethers::types::U256;
    use lit_node_core::{
        AuthMaterialType, JsonAuthSig, LitAbility, LitResourceAbilityRequest,
        LitResourceAbilityRequestResource, LitResourcePrefix, MultipleAuthSigs,
        constants::{AUTH_SIG_DERIVED_VIA_SESSION_SIG, AUTH_SIG_SESSION_SIG_ALGO},
    };

    use super::MultipleAuthSigsExtendedRef;

    #[derive(Debug)]
    struct TestResourcesTestCase {
        multiple_auth_sigs: MultipleAuthSigs,
        expected_resource_ids: Vec<String>,
        expected_resource_prefixes: Vec<LitResourcePrefix>,
    }

    fn get_test_resources_test_cases() -> Vec<TestResourcesTestCase> {
        let signed_message = serde_json::to_string(&SessionKeySignedMessageV2 {
            session_key: "pub_key".into(),
            resource_ability_requests: vec![LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251"
                        .into(),
                    resource_prefix: "lit-accesscontrolcondition".into(),
                },
                ability: LitAbility::AccessControlConditionDecryption.to_string(),
            }],
            capabilities: vec![],
            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
            expiration: "2024-01-01T00:00:00Z".to_string(),
            node_address: "localhost:7470".to_string(),
            max_price: U256::MAX,
        })
        .unwrap();

        vec![
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs::default(),
                expected_resource_ids: vec![],
                expected_resource_prefixes: vec![],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: None,
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec![
                    "524a697a410a417fb95a9f52d57cba5fa7c87b3acd3b408cf14560fa52691251".into(),
                ],
                expected_resource_prefixes: vec![LitResourcePrefix::ACC],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessageV2 {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                            max_price: U256::MAX,
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    cosmos: None,
                    kyve: None,
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessageV2 {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                            max_price: U256::MAX,
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    kyve: None,
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessageV2 {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                            max_price: U256::MAX,
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    cheqd: None,
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: None,
                    cheqd: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessageV2 {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                            max_price: U256::MAX,
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    juno: None,
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
            TestResourcesTestCase {
                multiple_auth_sigs: MultipleAuthSigs {
                    ethereum: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        signed_message.clone(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                    solana: None,
                    cosmos: None,
                    kyve: None,
                    cheqd: None,
                    juno: Some(JsonAuthSig::new_with_type(
                        "sig".into(),
                        AUTH_SIG_DERIVED_VIA_SESSION_SIG.into(),
                        serde_json::to_string(&SessionKeySignedMessageV2 {
                            session_key: "pub_key".into(),
                            resource_ability_requests: vec![LitResourceAbilityRequest {
                                resource: LitResourceAbilityRequestResource {
                                    resource: "123".into(),
                                    resource_prefix: "lit-pkp".into(),
                                },
                                ability: LitAbility::PKPSigning.to_string(),
                            }],
                            capabilities: vec![],
                            issued_at: "2023-05-01T15:41:08.640Z".to_string(),
                            expiration: "2024-01-01T00:00:00Z".to_string(),
                            node_address: "localhost:7470".to_string(),
                            max_price: U256::MAX,
                        })
                        .unwrap(),
                        "address".into(),
                        Some(AUTH_SIG_SESSION_SIG_ALGO.into()),
                        AuthMaterialType::SessionSig,
                        None,
                    )),
                },
                expected_resource_ids: vec!["123".into()],
                expected_resource_prefixes: vec![LitResourcePrefix::PKP],
            },
        ]
    }

    #[test]
    fn test_resources() {
        let test_cases = get_test_resources_test_cases();

        for test_case in test_cases {
            let resources = MultipleAuthSigsExtendedRef(&test_case.multiple_auth_sigs).resources();
            assert!(resources.is_ok());
            let resources = resources.unwrap();
            assert_eq!(resources.len(), test_case.expected_resource_ids.len());

            for (i, expected_resource_id) in test_case.expected_resource_ids.iter().enumerate() {
                assert_eq!(expected_resource_id, resources[i].get_resource_id());
                assert_eq!(
                    test_case.expected_resource_prefixes[i],
                    resources[i].get_resource_prefix()
                );
            }
        }
    }
}
