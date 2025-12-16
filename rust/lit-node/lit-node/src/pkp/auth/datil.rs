use std::sync::Arc;

use crate::auth::auth_material::JsonAuthSigExtendedRef;
use crate::error::{self, Error, unexpected_err_code, validation_err, validation_err_code};
use crate::error::{EC, unexpected_err};
use crate::models;
use crate::pkp::auth::{is_any_user_address_format_permitted, serialize_auth_context_for_checking_against_contract_data};
use crate::utils::encoding;
use anyhow::Result;
use ethers::abi::AbiEncode;
use ethers::core::utils::to_checksum;
use ethers::prelude::*;
use ethers::types::Bytes;
use ethers::utils::keccak256;

use lit_blockchain_lite::contracts::pkp_permissions::{self, PKPPermissions};
use lit_blockchain_lite::contracts::contract_resolver::ContractResolver;
use lit_core::config::LitConfig;

use lit_core::error::Unexpected;
use lit_core::utils::ipfs::bytes_to_ipfs_cid;
use lit_node_core::{AuthMethod, JsonAuthSig};
use tracing::instrument;



#[instrument(level = "debug", name = "check_pkp_auth", skip_all)]
#[allow(clippy::too_many_arguments)]
pub async fn datil_check_pkp_auth(
    ipfs_id_option: Option<String>,
    auth_sig: Option<JsonAuthSig>,
    pkp_pubkey: String,
    auth_context: models::AuthContext,
    cfg: &LitConfig,
    required_scopes: &[usize],
    bls_root_pubkey: &str,
    key_set_id: &str,
) -> Result<bool, Error> {
    use std::io::{Error, ErrorKind};

    debug!("auth_context- {:?}", auth_context);

    debug!(
        "Checking PKP for ipfs_id {:?} and pkp_pubkey {:?} for scopes {:?}",
        ipfs_id_option, pkp_pubkey, required_scopes
    );

    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

    let token_id = U256::from(&keccak256(encoding::hex_to_bytes(&pkp_pubkey)?));

    trace!("token_id: {}", token_id.encode_hex());

    let pkp_permissions_contract = resolver.pkp_permissions_contract(cfg).await?;
    let pkp_nft_contract = resolver.pkp_nft_contract(cfg).await?;

    let permitted_auth_methods: Vec<pkp_permissions::AuthMethod> = pkp_permissions_contract
        .get_permitted_auth_methods(token_id)
        .call()
        .await
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("Error getting permitted auth methods".to_string()),
            )
        })?;

    debug!("Permitted Auth Methods- {:?}", permitted_auth_methods);

    let owner_address = pkp_nft_contract
        .owner_of(token_id)
        .call()
        .await
        .or_else(|e| {
            // OwnerOf reverts when it has been burnt
            if e.as_revert().is_some() {
                debug!("Token {} has been burnt", token_id.encode_hex());
                Ok(H160::zero())
            } else {
                Err(unexpected_err_code(
                    e,
                    EC::NodeContractResolverConversionFailed,
                    None,
                ))
            }
        })?;

    debug!("Owner Address: {:?}", owner_address);

    // check if any of the AuthMethods provided are valid
    for auth_method in auth_context.auth_method_contexts {
        debug!("Checking auth method: {:?}", auth_method);
        let auth_method_type = U256::from(auth_method.auth_method_type);
        let serialized_user_id = serialize_auth_context_for_checking_against_contract_data(
            &auth_method,
        )
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeContractResolverConversionFailed,
                Some("Error serializing auth context".into()),
            )
        })?;
        let serialized_user_id = Bytes::from(serialized_user_id);

        debug!(
            "Checking if permitted auth methods contains for auth_method_type: {:?}, serialized_user_id: {:?}, token_id: {:?}",
            auth_method_type,
            encoding::bytes_to_hex(&serialized_user_id),
            token_id.encode_hex()
        );

        let auth_method_is_permitted = permitted_auth_methods.iter().any(|permitted_auth_method| {
            permitted_auth_method.auth_method_type == auth_method_type
                && permitted_auth_method.id == serialized_user_id
        });
        debug!("Is Auth method permitted? {:?}", auth_method_is_permitted);

        match auth_method_is_permitted {
            true => {
                let has_scopes = datil_check_scopes(
                    required_scopes,
                    pkp_permissions_contract.clone(),
                    token_id,
                    auth_method_type,
                    serialized_user_id,
                )
                .await?;

                if has_scopes {
                    return Ok(true);
                }
            }
            false => {
                debug!(
                    "AuthMethod not permitted for token id: {:?}- {:?}",
                    token_id.encode_hex(),
                    auth_method
                );
            }
        };

        let owner_string_address = format!("0x{}", hex::encode(owner_address.as_bytes()));

        // Wallet address
        if auth_method_type == U256::from(1) {
            debug!("Checking for Eth Wallet AuthMethod");

            let user_wallet_address = encoding::string_to_eth_address(auth_method.user_id.clone())?;

            let user_wallet_address_string = to_checksum(&user_wallet_address, None); // Because the address is the auth_method.user_id may not be in the checked sum format

            match is_any_user_address_format_permitted(
                user_wallet_address_string,
                &owner_address,
                required_scopes,
                &permitted_auth_methods,
                pkp_permissions_contract.clone(),
                token_id,
            )
            .await?
            {
                true => return Ok(true),
                false => debug!("User address not PKP owner and not permitted either"),
            };
        }
    }

    // check if any of the Lit actions in AuthContext are valid
    for ipfs_id in auth_context.action_ipfs_id_stack {
        let lit_action_auth_method_type = U256::from(2); // AuthMethodType::Action
        let ipfs_id_bytes = encoding::ipfs_cid_to_bytes(ipfs_id.clone())?;

        debug!(
            "Checking if permitted lit actions contains lit action with token_id {} and ipfs_id_bytes {}",
            token_id.encode_hex(),
            ipfs_id_bytes.clone().encode_hex()
        );

        let auth_method_is_permitted = permitted_auth_methods.iter().any(|permitted_auth_method| {
            permitted_auth_method.auth_method_type == lit_action_auth_method_type // AuthMethodType::Action
                && permitted_auth_method.id == ipfs_id_bytes.to_vec()
        });

        match auth_method_is_permitted {
            true => {
                let has_scopes = datil_check_scopes(
                    required_scopes,
                    pkp_permissions_contract.clone(),
                    token_id,
                    lit_action_auth_method_type,
                    Bytes::from(ipfs_id_bytes.to_vec()),
                )
                .await?;

                if has_scopes {
                    return Ok(true);
                }
            }
            false => {
                debug!(
                    "Lit Action not permitted for token id: {:?}- {:?}",
                    token_id.encode_hex(),
                    ipfs_id
                );
            }
        };
    }

    #[cfg(feature = "lit-actions")]
    if let Some(ipfs_id) = ipfs_id_option {
        let lit_action_auth_method_type = U256::from(2); // AuthMethodType::Action
        let ipfs_id_bytes = encoding::ipfs_cid_to_bytes(ipfs_id.clone())?;

        debug!(
            "Checking if permitted auth methods contains lit action with token_id {} and ipfs_id_bytes {}",
            token_id.encode_hex(),
            ipfs_id_bytes.clone().encode_hex()
        );

        let auth_method_is_permitted = permitted_auth_methods.iter().any(|permitted_auth_method| {
            permitted_auth_method.auth_method_type == lit_action_auth_method_type // AuthMethodType::Action
                && permitted_auth_method.id == ipfs_id_bytes.to_vec()
        });

        match auth_method_is_permitted {
            true => {
                let has_scopes = datil_check_scopes(
                    required_scopes,
                    pkp_permissions_contract.clone(),
                    token_id,
                    lit_action_auth_method_type,
                    Bytes::from(ipfs_id_bytes.to_vec()),
                )
                .await?;

                if has_scopes {
                    return Ok(true);
                }
            }
            false => {
                debug!(
                    "Lit Action not permitted for token id: {:?}- {:?}",
                    token_id.encode_hex(),
                    ipfs_id
                );
            }
        };
    }

    if let Some(auth_sig) = auth_sig {
        let user_wallet_address_string = JsonAuthSigExtendedRef::from(&auth_sig)
            .user_address(bls_root_pubkey)
            .await?; // checked sum

        debug!(
            "Checking if permitted auth methods contains address for token_id {} and auth_sig.address {:?}",
            token_id.encode_hex(),
            user_wallet_address_string
        );

        match is_any_user_address_format_permitted(
            user_wallet_address_string,
            &owner_address,
            required_scopes,
            &permitted_auth_methods,
            pkp_permissions_contract.clone(),
            token_id,
        )
        .await?
        {
            true => return Ok(true),
            false => debug!("User address not PKP owner and not permitted either"),
        };

        debug!(
            "AuthSig not permitted for token id: {:?}- {:?}",
            token_id.encode_hex(),
            auth_sig
        );
    }

    Err(validation_err_code(
        Error::new(
            ErrorKind::Other,
            format!(
                "None of the AuthMethods, AuthSig or Lit Actions meet the required scope {:?}.",
                required_scopes
            ),
        ),
        EC::NodeAuthSigScopeTooLimited,
        None,
    ))
}

async fn datil_check_scopes(
    required_scopes: &[usize],
    contract: PKPPermissions<Provider<Http>>,
    token_id: U256,
    auth_method_type: U256,
    serialized_user_id: Bytes,
) -> Result<bool, Error> {
    // When no scope is required, return immediately.
    if required_scopes.is_empty() {
        return Ok(true);
    }

    // this returns an array with 32 entries, with each entry being a bool indicating if the scope is permitted
    let permitted_scopes = contract
        .get_permitted_auth_method_scopes(
            token_id,
            auth_method_type,
            serialized_user_id.clone(),
            U256::from(32),
        )
        .call()
        .await
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeContractResolverConversionFailed,
                Some("Error getting permitted auth method scopes".to_string()),
            )
        })?;
    debug!(
        "permitted_scopes from the chain for the auth method: {:?}",
        permitted_scopes
    );

    let all_scopes_permitted = required_scopes.iter().all(|scope| {
        let permitted_scope = permitted_scopes.get(*scope).unwrap_or(&false);

        // the weird || here is to allow the SignPersonalMessage scope (2) to be used if the SignAnything scope (1) is also permitted, since if they can sign anything, they can sign a personal message.  So even if (2) is required, but not present, we can still sign if (1) is present
        *permitted_scope
            || (*scope == AuthMethodScope::SignPersonalMessage as usize
                && *permitted_scopes
                    .get(AuthMethodScope::SignAnything as usize)
                    .unwrap_or(&false))
    });

    Ok(all_scopes_permitted)
}