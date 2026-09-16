use crate::{
    config::chain::ChainDataConfigManager,
    utils::{
        datil_contract::DatilContracts,
        encoding::{self, ipfs_cid_to_bytes, string_to_eth_address, string_to_u256},
    },
};

use crate::error::{EC::NodeUnknownError, Result, unexpected_err_code};
use crate::models::PubKeyRoutingData;
use ethers::types::U256;
use ethers::{prelude::*, utils::keccak256};
use lit_core::config::LitConfig;
use lit_node_core::CurveType;
use serde_json::{Value, json};
use tracing::instrument;

pub async fn datil_pkp_permissions_is_permitted(
    token_id_str: String,
    cfg: &LitConfig,
    method: String,
    params: Vec<Value>,
    key_set_id: &str,
    cdm: &ChainDataConfigManager,
) -> Result<bool> {
    let datil_contracts = DatilContracts::new(cdm, key_set_id).await?;
    let contract = datil_contracts.pkp_permissions;

    let token_id = match string_to_u256(token_id_str) {
        Ok(token_id) => token_id,
        Err(e) => {
            let msg = "Could not convert token id to u256";
            error!("{}", msg);
            return Err(unexpected_err_code(
                e,
                NodeUnknownError,
                Some(msg.to_owned()),
            ));
        }
    };
    let res;

    if method == "isPermittedAction" {
        let param_str = match params[0].as_str() {
            Some(param_str) => param_str,
            None => {
                let msg = "ipfs_id is not a string";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };
        let ipfs_id = match ipfs_cid_to_bytes(param_str.to_string()) {
            Ok(ipfs_id) => ipfs_id,
            Err(e) => {
                let msg = "Could not convert ipfs id to bytes";
                error!("{}", msg);
                return Err(unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(msg.to_owned()),
                ));
            }
        };

        res = contract
            .is_permitted_action(token_id, Bytes::from(ipfs_id.to_vec()))
            .call()
            .await;
    } else if method == "isPermittedAddress" {
        let param_str = match params[0].as_str() {
            Some(param_str) => param_str,
            None => {
                let msg = "address is not a string";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };
        let address = match string_to_eth_address(param_str) {
            Ok(address) => address,
            Err(e) => {
                let msg = "Could not convert eth address to bytes";
                error!("{}", msg);
                return Err(unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(msg.to_owned()),
                ));
            }
        };

        res = contract
            .is_permitted_address(token_id, address)
            .call()
            .await;
    } else if method == "isPermittedAuthMethod" {
        let param_str = match params[0].as_str() {
            Some(param_str) => param_str,
            None => {
                let msg = "auth_method_type is not a string";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };
        let auth_method_type = match string_to_u256(param_str) {
            Ok(auth_method_type) => auth_method_type,
            Err(e) => {
                let msg = "Could not convert auth_method_type to u256";
                error!("{}", msg);
                return Err(unexpected_err_code(
                    e,
                    NodeUnknownError,
                    Some(msg.to_owned()),
                ));
            }
        };
        let param_array = match params[1].as_array() {
            Some(param_array) => param_array,
            None => {
                let msg = "user_id is not an array";
                error!("{}", msg);
                return Err(unexpected_err_code(msg, NodeUnknownError, None));
            }
        };

        let mut user_id: Vec<u8> = Vec::new();
        for _user_id in param_array {
            match _user_id.as_u64() {
                Some(_user_id_u64) => user_id.push(_user_id_u64 as u8),
                None => {
                    return Err(unexpected_err_code(
                        "user_id is not an array of u8 bytes",
                        NodeUnknownError,
                        None,
                    ));
                }
            }
        }

        let user_id = Bytes::from(user_id);
        res = contract
            .is_permitted_auth_method(token_id, auth_method_type, user_id)
            .call()
            .await;
    } else {
        return Err(unexpected_err_code(
            format!("Method not found: {method}"),
            NodeUnknownError,
            None,
        ));
    }

    res.map_err(|e| {
        let msg = format!("Error calling {method}: {e}");
        error!("{}", msg);
        unexpected_err_code(e, NodeUnknownError, Some(msg))
    })
}

pub async fn datil_pkp_permissions_is_permitted_auth_method(
    token_id_str: String,
    cfg: &LitConfig,
    auth_method_type_str: String,
    user_id_vec: Vec<u8>,
    key_set_id: &str,
    cdm: &ChainDataConfigManager,
) -> Result<bool> {
    let datil_contracts = DatilContracts::new(cdm, key_set_id).await?;
    let contract = datil_contracts.pkp_permissions;

    let token_id = match string_to_u256(token_id_str) {
        Ok(token_id) => token_id,
        Err(e) => {
            let msg = "Could not convert token id to u256";
            error!("{}", msg);
            return Err(unexpected_err_code(
                e,
                NodeUnknownError,
                Some(msg.to_owned()),
            ));
        }
    };

    let auth_method_type = match string_to_u256(auth_method_type_str) {
        Ok(auth_method_type) => auth_method_type,
        Err(e) => {
            let msg = "Could not convert auth_method_type to u256";
            error!("{}", msg);
            return Err(unexpected_err_code(
                e,
                NodeUnknownError,
                Some(msg.to_owned()),
            ));
        }
    };

    let user_id = Bytes::from(user_id_vec);
    contract
        .is_permitted_auth_method(token_id, auth_method_type, user_id)
        .call()
        .await
        .map_err(|e| {
            let msg = format!("Error calling isPermittedAuthMethod: {e}");
            error!("{}", msg);
            unexpected_err_code(e, NodeUnknownError, Some(msg))
        })
}

pub async fn datil_pkp_permissions_get_permitted(
    method: String,
    cfg: &LitConfig,
    token_id_str: String,
    key_set_id: &str,
    cdm: &ChainDataConfigManager,
) -> Result<Vec<Value>> {
    let datil_contracts = DatilContracts::new(cdm, key_set_id).await?;
    let contract = datil_contracts.pkp_permissions;

    let token_id = string_to_u256(token_id_str).map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not convert token id to u256".into()),
        )
    })?;
    let ret_val;

    if method == "getPermittedAddresses" {
        let res = contract
            .get_permitted_addresses(token_id)
            .call()
            .await
            .map_err(|e| {
                unexpected_err_code(e, NodeUnknownError, Some(format!("Error calling {method}")))
            })?;
        ret_val = res
            .iter()
            .map(|x| json!(format!("0x{}", encoding::bytes_to_hex(x.as_bytes()))))
            .collect::<Vec<Value>>();
    } else if method == "getPermittedActions" {
        let res = contract
            .get_permitted_actions(token_id)
            .call()
            .await
            .map_err(|e| {
                unexpected_err_code(e, NodeUnknownError, Some(format!("Error calling {method}")))
            })?;
        ret_val = res
            .iter()
            .map(|x| {
                json!(encoding::bytes_to_ipfs_cid(x).expect("Could not convert bytes to ipfs cid"))
            })
            .collect::<Vec<Value>>();
    } else if method == "getPermittedAuthMethods" {
        let res = contract
            .get_permitted_auth_methods(token_id)
            .call()
            .await
            .map_err(|e| {
                unexpected_err_code(e, NodeUnknownError, Some(format!("Error calling {method}")))
            })?;
        ret_val = res.iter().map(|x| json!(x)).collect::<Vec<Value>>();
    } else {
        return Err(unexpected_err_code(
            format!("Method not found: {method}"),
            NodeUnknownError,
            None,
        ));
    }

    Ok(ret_val)
}

pub async fn datil_pkp_permissions_get_permitted_auth_method_scopes(
    token_id_str: String,
    cfg: &LitConfig,
    auth_method_type_str: String,
    id_vec: Vec<u8>,
    max_scope_id_int: u64,
    key_set_id: &str,
    cdm: &ChainDataConfigManager,
) -> Result<Vec<bool>> {
    let datil_contracts = DatilContracts::new(cdm, key_set_id).await?;
    let contract = datil_contracts.pkp_permissions;

    let token_id = string_to_u256(token_id_str).map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not convert token id to u256".into()),
        )
    })?;

    let auth_method_type = string_to_u256(auth_method_type_str).map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not convert auth_method_type to u256".into()),
        )
    })?;
    let id = Bytes::from(id_vec);
    let max_scope_id = U256::from(max_scope_id_int);

    contract
        .get_permitted_auth_method_scopes(token_id, auth_method_type, id, max_scope_id)
        .call()
        .await
        .map_err(|e| {
            let msg = format!("Error calling get_permitted_auth_method_scopes: {e}");
            error!("{}", msg);
            unexpected_err_code(e, NodeUnknownError, Some(msg))
        })
}

#[instrument(skip(cfg), level = "debug")]
pub async fn datil_get_pubkey_routing_data_from_pubkey(
    cdm: &ChainDataConfigManager,
    cfg: &LitConfig,
    pubkey: &str,
    key_set_id: &str,
) -> Result<PubKeyRoutingData> {
    let datil_contracts = DatilContracts::new(cdm, key_set_id).await?;
    let contract = datil_contracts.pubkey_router;
    let pubkey_bytes = encoding::hex_to_bytes(pubkey)?;
    let hashed_pubkey = keccak256(pubkey_bytes);
    let token_id = U256::from_big_endian(hashed_pubkey.as_slice());

    trace!("token_id: {}", token_id);
    let datil_pubkey_routing_data : lit_blockchain_lite::contracts::pubkey_router::PubkeyRoutingData   = contract.pubkeys(token_id).call().await.map_err(|e| {
        unexpected_err_code(
            e,
            NodeUnknownError,
            Some("Could not find token id in pubkey routing contract.".to_string()),
        )
    })?;

    let pubkey_routing_data = PubKeyRoutingData {
        pubkey: datil_pubkey_routing_data.pubkey.to_vec(),
        curve_type: CurveType::try_from(datil_pubkey_routing_data.key_type)
            .expect("Failed to convert curve type"),
        tweak_preimage: datil_pubkey_routing_data.derived_key_id,
        key_set_identifier: key_set_id.to_string(),
    };
    Ok(pubkey_routing_data)
}
