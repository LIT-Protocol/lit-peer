use crate::auth::capabilities::recap::extract_and_verify_all_capabilities;
use crate::auth::validators::auth_sig::CapabilityAuthSigValidator;
use crate::auth::validators::siwe::SiweValidator;
use crate::error::blockchain_err_code;
use crate::error::unexpected_err_code;
use crate::error::{EC, Error, Result, parser_err_code};
use crate::models::auth::SessionKeySignedMessageV2;
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::payment::payed_endpoint::PayedEndpoint;
use crate::payment::payment_tracker::PaymentTracker;
use crate::payment::selection::check_payer_has_funds;
use ethers::prelude::*;
use ethers::types::U256;
use lit_blockchain::contracts::ledger::Ledger;
use lit_blockchain::contracts::payment_delegation::Restriction;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_core::LitResourcePrefix;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::debug;

pub struct PaymentDelegation {
    pub delegator: Address,
    pub max_price: U256,
    pub scopes: PaymentDelegationAllowedScopes,
}

#[derive(Default, Debug)]
pub struct PaymentDelegationAllowedScopes {
    pub encryption_sign: bool,
    pub lit_action: bool,
    pub pkp_sign: bool,
    pub sign_session_key: bool,
}

#[derive(Debug)]
pub struct RestrictedPayer {
    payer: Address,
    restriction: Restriction,
}

impl PaymentDelegationAllowedScopes {
    pub fn add_allowed_scope(&mut self, scope: &PayedEndpoint) {
        match scope {
            PayedEndpoint::EncryptionSign => self.encryption_sign = true,
            PayedEndpoint::LitAction => self.lit_action = true,
            PayedEndpoint::PkpSign => self.pkp_sign = true,
            PayedEndpoint::SignSessionKey => self.sign_session_key = true,
        }
    }

    pub fn does_allow(&self, scope: &PayedEndpoint) -> bool {
        match scope {
            PayedEndpoint::EncryptionSign => self.encryption_sign,
            PayedEndpoint::LitAction => self.lit_action,
            PayedEndpoint::PkpSign => self.pkp_sign,
            PayedEndpoint::SignSessionKey => self.sign_session_key,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn check_for_payment_db(
    user_address: &Address,
    session_sig_max_price: Option<U256>,
    required_funds: I256,
    threshold: usize,
    payment_tracker: &Arc<PaymentTracker>,
    delegation_usage_db: &DelegatedUsageDB,
    ledger: &Ledger<Provider<Http>>,
    cfg: &LitConfig,
) -> Result<Option<(Address, I256)>> {
    let all_payers_and_restrictions =
        get_all_payers_and_restrictions_via_payer_db(*user_address, cfg).await?;
    info!(
        "all_payers_and_restrictions for user {}: {:?}",
        user_address, all_payers_and_restrictions
    );

    charge_payer(
        user_address,
        session_sig_max_price,
        all_payers_and_restrictions,
        required_funds,
        threshold,
        payment_tracker,
        delegation_usage_db,
        ledger,
        cfg,
    )
    .await
}

/// Look for payment delegation and return the delegator if any.
pub async fn check_for_payment_delegation(
    user_address: &Address,
    session_key_signed_message: SessionKeySignedMessageV2,
    required_funds: I256,
    payment_tracker: &Arc<PaymentTracker>,
    required_scope: &PayedEndpoint,
    bls_root_pubkey: &str,
    ledger: &Ledger<Provider<Http>>,
) -> Result<Option<(Address, I256)>> {
    trace!("Getting authorized payers via delegation signature");

    // loop over capabilities and find any for lit-paymentdelegation://*
    for capability_auth_sig in session_key_signed_message.capabilities.iter() {
        let siwe_validator = SiweValidator::new();
        let signed_message =
            match siwe_validator.parse_siwe_message(&capability_auth_sig.signed_message) {
                Ok(signed_message) => signed_message,
                Err(e) => {
                    error!("{:?}", e);
                    continue;
                }
            };
        if let Err(e) = siwe_validator
            .validate_capability_auth_sig(capability_auth_sig, bls_root_pubkey)
            .await
        {
            error!("Error validating Capacity Delegation AuthSig: {:?}", e);
            continue;
        }

        if let Ok(Some(delegation)) =
            check_verified_siwe_for_a_payment_delegator(user_address, signed_message)
        {
            if let Ok((true, spending_limit)) = validate_delegation_requirements(
                &delegation,
                required_scope,
                required_funds,
                payment_tracker,
                session_key_signed_message.max_price,
                ledger,
            )
            .await
            {
                return Ok(Some((delegation.delegator, spending_limit)));
            }
        };
    }

    Ok(None)
}

/// Look for payment delegation and return the delegator if any.
pub fn check_verified_siwe_for_a_payment_delegator(
    user_address: &Address,
    signed_message: siwe::Message,
    // cfg: &LitConfig,
) -> Result<Option<PaymentDelegation>> {
    let user_address_hex = bytes_to_hex(user_address).to_ascii_lowercase();
    let delegator_address = Address::from(signed_message.address);

    let capabilities = extract_and_verify_all_capabilities(&signed_message)?;

    for capability in capabilities {
        for ability in capability.recap_abilities().iter() {
            if ability.0.scheme_str() == LitResourcePrefix::PD.to_string() {
                // loop over the restrictions
                for inner_ability in ability.1 {
                    if inner_ability.0.clone().into_inner() == *"Auth/Auth".to_string() {
                        // loop over the restrictions
                        for map in inner_ability.1.clone().into_iter() {
                            let delegation_res = construct_payment_delegation(
                                &user_address_hex,
                                delegator_address,
                                &map,
                            )?;
                            if let Some(delegation) = delegation_res {
                                return Ok(Some(delegation));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

async fn validate_delegation_requirements(
    delegation: &PaymentDelegation,
    required_scope: &PayedEndpoint,
    required_funds: I256,
    payment_tracker: &Arc<PaymentTracker>,
    session_sig_max_price: U256,
    ledger: &Ledger<Provider<Http>>,
) -> Result<(bool, I256)> {
    if session_sig_max_price > delegation.max_price {
        debug!(
            "SessionSig max_price: {:?} can not be greater than the delegation AuthSig max_price: {:?}",
            session_sig_max_price, delegation.max_price
        );
        return Ok((false, I256::from(0)));
    }

    if !delegation.scopes.does_allow(required_scope) {
        debug!(
            "Delegation scope does meet required scope: {:?}, {:?}",
            delegation.scopes, required_scope
        );
        return Ok((false, I256::from(0)));
    }

    let spending_limit = check_payer_has_funds(
        ledger,
        &delegation.delegator,
        required_funds,
        payment_tracker,
    )
    .await?;

    Ok((true, spending_limit))
}

async fn get_all_payers_and_restrictions_via_payer_db(
    user_address: Address,
    cfg: &LitConfig,
) -> Result<Vec<RestrictedPayer>> {
    trace!("Finding all payers in the PayerDB that have delegated to this user");

    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let payment_delegation_contract = resolver.payment_delegation_contract(cfg).await?;

    let returned_tuple = payment_delegation_contract
        .get_payers_and_restrictions(vec![user_address])
        .call()
        .await
        .map_err(|e| blockchain_err_code(e, EC::NodeBlockchainError, None))?;
    let payers = returned_tuple.0.clone();
    let restrictions = returned_tuple.1.clone();
    if payers.is_empty() {
        debug!(
            "No payers returned for wallet {}.  This should never happen, because the function should always return n elements for n users",
            bytes_to_hex(user_address)
        );
        return Ok(vec![]);
    }

    // payers is of type address[][] because we support multiple payers for a single user
    // restrictions is of type string[][] because each payer should have a corresponding restriction
    // we need to check again that we got at least 1 payer to continue
    let payers = payers[0].clone();
    let restrictions = restrictions[0].clone();
    if payers.is_empty() {
        debug!(
            "No payers for wallet {}.  This means there are just no payer entries for a user.",
            bytes_to_hex(user_address)
        );
        return Ok(vec![]);
    }

    if payers.len() != restrictions.len() {
        return Err(unexpected_err_code(
            format!(
                "Payers and restrictions arrays are not the same length.  Payers length is {} and restrictions length is {}",
                payers.len(),
                restrictions.len()
            ),
            EC::NodeBlockchainError,
            None,
        ));
    }

    Ok(payers
        .into_iter()
        .zip(restrictions.into_iter())
        .map(|(payer, restriction)| RestrictedPayer { payer, restriction })
        .collect())
}

#[allow(clippy::too_many_arguments)]
async fn charge_payer(
    user_address: &Address,
    session_sig_max_price: Option<U256>,
    all_payers_and_restrictions: Vec<RestrictedPayer>,
    required_funds: I256,
    threshold: usize,
    payment_tracker: &Arc<PaymentTracker>,
    delegation_usage_db: &DelegatedUsageDB,
    ledger: &Ledger<Provider<Http>>,
    cfg: &LitConfig,
) -> Result<Option<(Address, I256)>> {
    for restricted_payer in all_payers_and_restrictions.iter() {
        if let Some(session_sig_max_price) = session_sig_max_price {
            let max_price_per_node =
                restricted_payer.restriction.total_max_price / (threshold as u128);

            if session_sig_max_price > U256::from(max_price_per_node) {
                error!(
                    "SessionSig max price of {} should be less than the PaymentDB max price per node of {}.  The payer {} will not cover this transaction.",
                    session_sig_max_price, max_price_per_node, restricted_payer.payer
                );
                continue;
            }
        }

        let spending_limit = match check_payer_has_funds(
            ledger,
            &restricted_payer.payer,
            required_funds,
            payment_tracker,
        )
        .await
        {
            Ok(spending_limit) => spending_limit,
            Err(e) => {
                debug!(
                    "Payer: {:?} doesn't have enough funds: {:?}",
                    restricted_payer.payer, required_funds
                );
                error!("{:?}", e);
                continue;
            }
        };

        match update_payment_db(
            *user_address,
            restricted_payer.payer,
            &restricted_payer.restriction,
            delegation_usage_db,
        )
        .await
        {
            Ok(true) => return Ok(Some((restricted_payer.payer, spending_limit))),
            Ok(false) => info!("Exceeded Usage! Checking next payer..."),
            Err(e) => error!("Error updating the local PaymentDB: {:?}", e),
        }
    }

    Ok(None)
}

async fn update_payment_db(
    user_address: Address,
    payer: Address,
    restriction: &Restriction,
    delegation_usage_db: &DelegatedUsageDB,
) -> Result<bool> {
    // we need to create a hash of the current period start, the user address, and the payer address.
    // this will be used to restrict the number of requests a user can make in a given period
    // find the current start for the period.  the global start is jan 1st, 1970.
    // we consider than "n" periods have elapsed since the global start, where n is the number of periods that have elapsed since the global start
    let now_seconds = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("There was an error getting the system time".into()),
            )
        })?
        .as_secs();
    if restriction.period_seconds > U256::from(u64::MAX) {
        return Err(unexpected_err_code(
            format!(
                "Period seconds is greater than u64 max value.  This should never happen.  You may have accidently set the period seconds in the restriction to a giant number.  The number is: {}",
                restriction.period_seconds
            ),
            EC::NodeBlockchainError,
            None,
        ));
    }

    // by default, they are both zero, so only check restrictions if at least 1 value is nonzero
    // which means they set a restriction
    if restriction.requests_per_period.is_zero() || restriction.period_seconds.is_zero() {
        return Ok(false);
    }

    // this function will panic if the number is greater than a u64, and we check above.  so this is safe
    let period_seconds = restriction.period_seconds.as_u64();
    let period_start = now_seconds / period_seconds * period_seconds; // We do this to get to the start of the current period as all the current timestamps within the period should map to the period's start as it's used as a map key

    let mut hasher = Sha256::new();
    hasher.update(
        [
            user_address.as_bytes(),
            payer.as_bytes(),
            &period_start.to_le_bytes(),
        ]
        .concat(),
    );
    let delegation_uses_key = hasher.finalize().to_vec();

    let max_uses = match restriction.requests_per_period < U256::from(u64::MAX) {
        true => restriction.requests_per_period.as_u64(),
        false => {
            return Err(unexpected_err_code(
                format!(
                    "Could not convert requests per period to u64.  The number is too large.  The number is: {}",
                    restriction.requests_per_period
                ),
                EC::NodeBlockchainError,
                None,
            ));
        }
    };

    let already_used = delegation_usage_db
        .delegation_uses_map
        .get(&delegation_uses_key)
        .await
        .unwrap_or(0);
    info!(
        "Checking if payer should pay for user - User uses: {} and max uses: {}",
        already_used, max_uses,
    );

    if already_used as u64 >= max_uses {
        info!("Max usage per period reached!");
        return Ok(false);
    }

    delegation_usage_db
        .delegation_uses_map
        .insert(delegation_uses_key, already_used + 1)
        .await;

    Ok(true)
}

fn construct_payment_delegation(
    user_address_str: &str,
    delegator_address: Address,
    map: &BTreeMap<std::string::String, serde_json::Value>,
) -> Result<Option<PaymentDelegation>> {
    // possible restrictions:
    // "delegate_to" = the addresses to delegate this to
    // "max_price" = the upper limit of the accepted price range
    // "scopes" = list of allowed endpoints
    match map.get("delegate_to") {
        // loop over all items in delegate_to array and check if the user is in there
        Some(delegate_to) => {
            let delegate_to_arr = delegate_to
                .as_array()
                .ok_or(siwe_conversion_error("", "delegate_to is not an array"))?;
            let user_is_delegate = delegate_to_arr.iter().any(
                |x| matches!(x.as_str(), Some(s) if s.to_ascii_lowercase() == user_address_str),
            );
            let err_msg = format!(
                "User {} is delegate: {} and delegate_to_arr: {:?}",
                user_address_str, user_is_delegate, delegate_to_arr,
            );
            debug!("{}", &err_msg);
            if !user_is_delegate {
                return Err(siwe_conversion_error("User is not delegated to", &err_msg));
            }
        }
        None => {
            return Err(siwe_conversion_error(
                "",
                "delegate_to is not set in payment delegation",
            ));
        }
    }

    // skip this one if the user has already gone over the max uses
    let max_price = match map.get("max_price") {
        Some(max_price) => max_price
            .as_str()
            .ok_or(siwe_conversion_error(
                "",
                "Could not convert max_price in delegation recap to string",
            ))?
            .parse::<U256>()
            .map_err(|e| {
                siwe_conversion_error(
                    &e.to_string(),
                    "Could not convert max_price in delegation recap to U256",
                )
            })?,
        None => {
            return Err(siwe_conversion_error(
                "",
                "max_price is not set in payment delegation",
            ));
        }
    };

    // Check for the allowed scopes
    let mut allowed_scopes = PaymentDelegationAllowedScopes::default();
    let scopes_arr = match map.get("scopes") {
        None => Err(siwe_conversion_error(
            "",
            "scopes is not set in payment delegation",
        ))?,
        Some(scopes) => scopes
            .as_array()
            .ok_or(siwe_conversion_error("", "`scopes` is not an array"))?,
    };

    for s in scopes_arr.iter() {
        match s.as_str() {
            None => {
                return Err(siwe_conversion_error(
                    "",
                    &format!("`{}` is not a valid payment delegation scope", s),
                ));
            }
            Some(s) => match s.parse() {
                Err(e) => {
                    return Err(siwe_conversion_error(
                        "",
                        &format!("`{}` is not a valid payment delegation scope", s),
                    ));
                }
                Ok(s) => allowed_scopes.add_allowed_scope(&s),
            },
        }
    }

    debug!(
        "PaymentDelegation: {:?}, {:?}, {:?}",
        delegator_address, max_price, allowed_scopes
    );

    Ok(Some(PaymentDelegation {
        delegator: delegator_address,
        max_price,
        scopes: allowed_scopes,
    }))
}

fn siwe_conversion_error(e: &str, err_msg: &str) -> Error {
    error!("{}; {}", e, err_msg);
    parser_err_code(e, EC::NodeSIWESigConversionError, Some(err_msg.to_string()))
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::ops::Add;

    use chrono::Duration;
    use ethers::abi::AbiEncode;
    use ethers::core::k256::ecdsa::SigningKey;
    use ethers::signers::{LocalWallet, Signer as WalletSigner, Wallet};
    use ethers::types::U256;
    use rand_core::OsRng;
    use serde_json::Value;
    use siwe_recap::Capability;
    use std::ops::Sub;

    //use crate::auth::auth_material::JsonAuthSig;
    use crate::payment::payment_delegation::{
        PayedEndpoint, PayedEndpoint::*, check_verified_siwe_for_a_payment_delegator,
    };
    use crate::utils::encoding::bytes_to_hex;
    use lit_node_core::LitResourcePrefix;

    pub fn get_siwe_with_payment_delegation(
        delegator_wallet: &Wallet<SigningKey>,
        user_address: &str,
        max_price: U256,
        scopes: &[PayedEndpoint],
    ) -> siwe::Message {
        let mut notabene = BTreeMap::new();
        //notabene.insert("nft_id".to_string(), Value::from(vec![Value::from(nft_id)]));
        notabene.insert(
            "delegate_to".to_string(),
            Value::from(vec![Value::from(user_address)]),
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
        let now = chrono::Utc::now();
        let siwe_issued_at = now.sub(Duration::days(1));
        let siwe_expiration_time = now.add(Duration::days(7));
        let mut capabilities = Capability::<Value>::default();
        let resource = "Auth/Auth".to_string();
        let resource_prefix = format!("{}://*", LitResourcePrefix::PD);
        let capabilities = capabilities
            .with_actions_convert(resource_prefix, [(resource, [notabene])])
            .unwrap();
        capabilities
            .build_message(siwe::Message {
                domain: "example.com".parse().unwrap(),
                address: delegator_wallet.address().into(),
                statement: None,
                uri: "lit:capability:delegation".parse().unwrap(),
                version: siwe::Version::V1,
                chain_id: 1,
                nonce: "mynonce1".into(),
                issued_at: siwe_issued_at
                    .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                    .parse()
                    .unwrap(),
                expiration_time: Some(
                    siwe_expiration_time
                        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                        .parse()
                        .unwrap(),
                ),
                not_before: None,
                request_id: None,
                resources: vec![],
            })
            .unwrap()
    }

    struct TestCase {
        max_price: &'static str,
        scopes: Vec<PayedEndpoint>,
    }

    fn get_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                max_price: "abcd12",
                scopes: vec![],
            },
            TestCase {
                max_price: "3456cdef",
                scopes: vec![EncryptionSign, LitAction, PkpSign],
            },
            TestCase {
                max_price: "abcdef78",
                scopes: vec![EncryptionSign],
            },
            TestCase {
                max_price: "1234567890",
                scopes: vec![LitAction],
            },
            TestCase {
                max_price: "abcdef",
                scopes: vec![PkpSign],
            },
            TestCase {
                max_price: "abcd567890",
                scopes: vec![EncryptionSign, PkpSign],
            },
        ]
    }

    #[test]
    pub fn test_parse_siwe_with_payment_delegation() {
        let chain_id: u64 = 12345;
        let delegator_wallet = LocalWallet::new(&mut OsRng).with_chain_id(chain_id);
        let user_wallet = LocalWallet::new(&mut OsRng).with_chain_id(chain_id);
        for test in get_test_cases() {
            let max_price = test.max_price.parse::<U256>().unwrap();
            let siwe_message = get_siwe_with_payment_delegation(
                &delegator_wallet,
                &bytes_to_hex(user_wallet.address()),
                max_price,
                &test.scopes,
            );
            let payment_delegation =
                check_verified_siwe_for_a_payment_delegator(&user_wallet.address(), siwe_message)
                    .unwrap()
                    .unwrap();
            assert_eq!(payment_delegation.delegator, delegator_wallet.address());
            assert_eq!(payment_delegation.max_price, max_price);
            assert_eq!(
                payment_delegation.scopes.encryption_sign,
                test.scopes.contains(&EncryptionSign)
            );
            assert_eq!(
                payment_delegation.scopes.lit_action,
                test.scopes.contains(&LitAction)
            );
            assert_eq!(
                payment_delegation.scopes.pkp_sign,
                test.scopes.contains(&PkpSign)
            );
        }
    }

    #[test]
    // Reject a delegation that is for someone other than the intended user.
    pub fn test_reject_payment_delegation_without_given_user() {
        let chain_id: u64 = 12345;
        let delegator_wallet = LocalWallet::new(&mut OsRng).with_chain_id(chain_id);
        let user_wallet_1 = LocalWallet::new(&mut OsRng).with_chain_id(chain_id);
        let user_wallet_2 = LocalWallet::new(&mut OsRng).with_chain_id(chain_id);
        let max_price = "abcd1234".parse::<U256>().unwrap();
        let siwe_message = get_siwe_with_payment_delegation(
            &delegator_wallet,
            &bytes_to_hex(user_wallet_1.address()),
            max_price,
            &[],
        );
        let payment_delegation_res =
            check_verified_siwe_for_a_payment_delegator(&user_wallet_2.address(), siwe_message);
        assert!(payment_delegation_res.is_err());
    }
}
