use lit_core::utils::binary::bytes_to_hex;
use lit_node_core::{AccessControlConditionItem, EVMContractConditionItem, SolRpcCondition, SolRpcConditionItem, SolRpcConditionItemV0, SolRpcConditionV2Options, UnifiedAccessControlConditionItem};
use lit_rust_crypto::k256::sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

use anyhow::Result;
use tracing::{debug, trace};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestConditions {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
}



pub fn hash_access_control_conditions(req: RequestConditions) -> Result<String> {
    // hash the access control condition and thing to decrypt
    let mut hasher = Sha256::new();

    // we need to check if we got passed an access control condition or an evm contract condition
    if let Some(access_control_conditions) = &req.access_control_conditions {
        let stringified_access_control_conditions =
            serde_json::to_string(access_control_conditions)?;
        trace!(
            "stringified_access_control_conditions: {:?}",
            stringified_access_control_conditions
        );
        hasher.update(stringified_access_control_conditions.as_bytes());
    } else if let Some(evm_contract_conditions) = &req.evm_contract_conditions {
        let stringified_access_control_conditions =
            serde_json::to_string(evm_contract_conditions)?;
        trace!(
            "stringified_access_control_conditions: {:?}",
            stringified_access_control_conditions
        );
        hasher.update(stringified_access_control_conditions.as_bytes());
    } else if let Some(sol_rpc_conditions) = &req.sol_rpc_conditions {
        // hash differently if this is v1 or v2 conditions
        let mut is_v2 = false;
        for condition_item in sol_rpc_conditions {
            if let SolRpcConditionItem::Condition(condition) = condition_item
                && condition.pda_params.is_some()
            {
                is_v2 = true;
                break;
            }
        }
        if is_v2 {
            // we can just hash directly
            let stringified_access_control_conditions =
                serde_json::to_string(&req.sol_rpc_conditions)?;
            debug!(
                "stringified_access_control_conditions: {:?}",
                stringified_access_control_conditions
            );
            hasher.update(stringified_access_control_conditions.as_bytes());
        } else {
            // need to massage into v1 condition array
            let v1_conditions = convert_sol_rpc_conditions_to_v1(sol_rpc_conditions);
            let stringified_access_control_conditions =
                serde_json::to_string(&v1_conditions)?;
            debug!(
                "stringified_access_control_conditions: {:?}",
                stringified_access_control_conditions
            );
            hasher.update(stringified_access_control_conditions.as_bytes());
        }
    } else if let Some(unified_access_control_conditions) = &req.unified_access_control_conditions {
        let stringified_access_control_conditions =
            serde_json::to_string(unified_access_control_conditions)?;
        trace!(
            "stringified_access_control_conditions: {:?}",
            stringified_access_control_conditions
        );
        hasher.update(stringified_access_control_conditions.as_bytes());
    } else {
        return Err(anyhow::anyhow!("Missing access control conditions"));
    }

    let hashed_access_control_conditions = bytes_to_hex(hasher.finalize());
    debug!(
        "hashed access control conditions: {:?}",
        hashed_access_control_conditions
    );
    Ok(hashed_access_control_conditions)
}


pub fn convert_sol_rpc_conditions_to_v1(
    sol_rpc_conditions: &Vec<SolRpcConditionItem>,
) -> Vec<SolRpcConditionItemV0> {
    // need to massage into v1 condition array
    let mut v1_conditions: Vec<SolRpcConditionItemV0> = Vec::new();
    for condition_item in sol_rpc_conditions {
        match condition_item {
            SolRpcConditionItem::Condition(condition) => {
                v1_conditions.push(SolRpcConditionItemV0::Condition(
                    sol_rpc_condition_v2_to_v1(condition),
                ));
            }
            SolRpcConditionItem::Operator(operator) => {
                v1_conditions.push(SolRpcConditionItemV0::Operator(*operator));
            }
            SolRpcConditionItem::Group(group) => {
                v1_conditions.push(SolRpcConditionItemV0::Group(group.clone()));
            }
        }
    }
    v1_conditions
}


pub fn sol_rpc_condition_v2_to_v1(condition: &SolRpcConditionV2Options) -> SolRpcCondition {
    SolRpcCondition {
        method: condition.method.clone(),
        params: condition.params.clone(),
        chain: condition.chain.clone(),
        return_value_test: condition.return_value_test.clone(),
    }
}
