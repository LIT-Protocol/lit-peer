use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::staking::UncompressedK256Key;
use lit_blockchain::contracts::{
    lit_token::lit_token::LITToken,
    staking::{Staking, StakingErrors, Validator},
};
use lit_node_common::models::NodeStakingStatus;
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use crate::testnet::NodeAccount;

use super::super::PeerItem;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};

use super::Actions;

const DEFAULT_TIMELOCK_SECONDS: u64 = 86400 * 120; // 1 day

impl Actions {
    pub async fn get_current_validators(&self, realm_id: U256) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_current_epoch(realm_id)
            .call()
            .await
            .expect("Error getting validators from chain")
    }

    pub async fn get_current_validator_structs(&self, realm_id: U256) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_current_epoch(realm_id)
            .call()
            .await
            .expect("Error getting validator structs from chain")
    }

    pub async fn get_validator_struct(&self, staker_address: Address) -> Validator {
        self.contracts
            .staking
            .validators(staker_address)
            .call()
            .await
            .expect("Error getting validator struct from chain")
    }

    pub async fn get_next_validators(&self, realm_id: U256) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_next_epoch(realm_id)
            .call()
            .await
            .expect("Error getting next validators from chain")
    }

    pub async fn get_next_validator_structs(&self, realm_id: U256) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_next_epoch(realm_id)
            .call()
            .await
            .expect("Error getting next validator structs from chain")
    }

    pub async fn get_current_validator_count(&self, realm_id: U256) -> u32 {
        self.get_current_validators(realm_id).await.len() as u32
    }

    pub async fn send_approve_and_stake(
        &self,
        staker: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> Result<()> {
        // give some tokens to the staker

        let deployer_balance = self
            .contracts
            .lit_token
            .balance_of(self.deploy_address)
            .call()
            .await?;
        info!("Deployer balance is {}", deployer_balance);

        info!(
            "Balance before send: {:?}",
            self.lit_token_balance(staker.address()).await
        );

        let amount_to_send = ethers::utils::parse_units(4, 18).unwrap().into();
        let r = self
            .contracts
            .lit_token
            .transfer(staker.address(), amount_to_send);

        let res = r
            .send()
            .await
            .unwrap()
            .interval(Duration::from_millis(500))
            .await;
        if let Err(e) = res {
            panic!("Error sending LIT tokens: {:?}", e);
        }

        info!(
            "Balance after send: {:?}",
            self.lit_token_balance(staker.address()).await
        );

        let lit_token = LITToken::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.lit_token.address(),
            staker.clone(),
        );

        // spender is the deployed staking balances contract
        let spender = self.contracts.staking.address();
        let amount_to_approve = ethers::utils::parse_units(2, 18).unwrap().into();
        let r = lit_token.approve(spender, amount_to_approve);
        let r = r.send().await;
        if r.is_err() {
            panic!("Error Approving ERC20 : {:?}", r);
        }

        let receipt = r.unwrap().await;
        if receipt.is_err() {
            panic!("(Receipt) Error Approving ERC20 : {:?}", receipt);
        }

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            staker.clone(),
        );

        let stake_amount = staking.min_self_stake().call().await?;

        info!("Staking from {:?}", staker.address(),);

        let r = staking.stake(
            stake_amount,
            U256::from(DEFAULT_TIMELOCK_SECONDS),
            staker.address(),
        );

        let r = r.send().await;
        if let Err(e) = r {
            debug!(
                "Error doing stake.  Revert: {:?}",
                lit_blockchain::util::decode_revert(&e, staking.abi())
            );

            let revert: Option<StakingErrors> = e.decode_contract_revert();
            match revert {
                Some(r) => {
                    return Err(anyhow::anyhow!(
                        "Error doing stake: {:?}.  Revert: {:?}",
                        e,
                        r
                    ));
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Error doing stake: {:?}.  Could not decode revert reason.  Revert: {:?}",
                        &e,
                        lit_blockchain::util::decode_revert(&e, staking.abi())
                    ));
                }
            }
        }

        // make sure it's fully mined so we don't accidently advance then lock the next epoch before the user has actually staked
        let _receipt = r.unwrap().interval(Duration::from_millis(500)).await;

        Ok(())
    }

    pub async fn send_request_to_join(
        &self,
        realm_id: U256,
        staker: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        _ip: u32,
        _port: u32,
        node_info: &PeerItem,
    ) -> Result<()> {
        info!(
            "Staking from {:?} for with node_address {:?} - PeerItem {:?}",
            staker.address(),
            node_info.node_address,
            node_info
        );

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            staker.clone(),
        );

        info!(
            "request to join with sender pub key: {:?}",
            U256::from_big_endian(&node_info.sender_public_key[..])
        );

        let r = staking.request_to_join(realm_id);

        let r = r.send().await;
        if let Err(e) = r {
            debug!(
                "Error doing request_to_join for {:}.  Revert: {:?}",
                node_info.addr,
                lit_blockchain::util::decode_revert(&e, staking.abi())
            );

            let revert: Option<StakingErrors> = e.decode_contract_revert();
            match revert {
                Some(r) => {
                    return Err(anyhow::anyhow!(
                        "Error doing request_to_join {:} : {:?}.  Revert: {:?}",
                        node_info.addr,
                        e,
                        r
                    ));
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Error doing request_to_join {:} : {:?}.  Could not decode revert reason.  Revert: {:?}",
                        node_info.addr,
                        &e,
                        lit_blockchain::util::decode_revert(&e, staking.abi())
                    ));
                }
            }
        }

        // make sure it's fully mined so we don't accidently advance then lock the next epoch before the user has actually staked
        let _receipt = r.unwrap().interval(Duration::from_millis(500)).await;

        Ok(())
    }

    pub async fn ensure_node_staked_and_joined(
        &self,
        realm_id: U256,
        node_account: &NodeAccount,
        node_addr: &str,
        node_port: usize,
    ) -> Result<NodeStakingStatus> {
        let node_signer = node_account.signing_provider.clone();

        info!(
            "Checking if node {} is already staked...",
            node_signer.address()
        );

        // stake if not already
        let is_staked = self
            .contracts
            .staking
            .check_staking_amounts(node_account.staker_address)
            .call()
            .await;
        if let Ok(is_staked) = is_staked {
            if is_staked {
                info!("Node {} is already staked!", node_signer.address());
            } else {
                info!("Node {} is not staked.  Staking...", node_signer.address());
                self.send_approve_and_stake(node_signer.clone()).await?;
            }
        }

        // request to join if not already
        let next_validators = self
            .contracts
            .staking
            .get_validators_in_next_epoch(realm_id)
            .call()
            .await?;
        let is_joined = next_validators.contains(&node_account.staker_address);
        if !is_joined {
            info!("Node {} is not joined.  Joining...", node_signer.address());
            let peer_item = PeerItem {
                addr: node_addr.to_string(),
                node_address: node_account.node_address,
                sender_public_key: node_account.coms_keys.sender_public_key().to_bytes(),
                receiver_public_key: node_account.coms_keys.receiver_public_key().to_bytes(),
                staker_address: node_account.staker_address,
            };

            self.send_request_to_join(
                realm_id,
                node_signer,
                2130706433u32,
                node_port as u32,
                &peer_item,
            )
            .await?;
        }

        Ok(NodeStakingStatus::StakedAndJoined)
    }

    pub async fn get_node_attested_pubkey_mappings(
        &self,
        node_addresses: &Vec<H160>,
    ) -> Result<Vec<Option<UncompressedK256Key>>> {
        // Get the node's attested pubkey mappings from the staking contract
        let pubkey_mappings = self
            .contracts
            .staking
            .get_node_attested_pub_key_mappings(node_addresses.clone())
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Error getting node attested pubkey mappings: {:?}", e))?;

        // Turn into a map
        let pubkey_mappings = pubkey_mappings
            .into_iter()
            .map(|m| (m.node_address, m.pub_key))
            .collect::<HashMap<_, _>>();

        // Return the pubkey mappings for each node address
        Ok(node_addresses
            .into_iter()
            .map(|node_address| pubkey_mappings.get(&node_address).cloned())
            .collect())
    }
}
