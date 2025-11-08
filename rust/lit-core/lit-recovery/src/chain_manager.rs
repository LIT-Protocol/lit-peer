use std::fmt;
use std::sync::Arc;

use crate::{
    config::RecoveryConfig,
    error::{Error, RecoveryResult},
};
use bulletproofs::k256::{SecretKey, ecdsa::SigningKey};
use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::H160,
};
use lit_blockchain::contracts::{
    backup_recovery::{BackupRecovery, NextStateDownloadable, NodeRecoveryStatusMap},
    contract_resolver::ContractResolver,
    staking::{AddressMapping, Staking, Validator},
};

use reqwest::Url;

pub struct ChainManager<M> {
    pub backup_recovery: BackupRecovery<M>,
    pub staking: Staking<M>,
    #[allow(dead_code)]
    signer_or_provider: Arc<M>,
}

impl ChainManager<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub async fn new_with_signer(private_key: &[u8], cfg: &RecoveryConfig) -> Result<Self, Error> {
        let resolver_address = match &cfg.resolver_address {
            None => {
                println!(
                    "Looks like your network context is not set. Run contract-resolver address=<network contract resolver address>"
                );
                return Err(crate::error::Error::Contract(
                    "Contract Resolver Address not set, aborting".into(),
                ));
            }
            Some(address) => address,
        };

        let provider = match _build_rpc_client(cfg) {
            Ok(p) => p,
            Err(e) => {
                return Err(crate::Error::InvalidRequest(e.to_string()));
            }
        };
        let bytes = bulletproofs::k256::FieldBytes::from_slice(private_key);
        let sk = match SecretKey::from_bytes(bytes) {
            Ok(key) => key,
            Err(e) => {
                return Err(crate::Error::General(e.to_string()));
            }
        };
        let chain_id = cfg.get_chain_id_or_default();
        let env = cfg.get_env_or_default();

        println!("using chain id: {}", chain_id);
        println!("using contract resolver address: {}", resolver_address.clone());
        let wallet = LocalWallet::from(sk).with_chain_id(chain_id);

        let sm = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

        let resolver = ContractResolver::new(
            resolver_address.clone().parse::<H160>().map_err(|e| Error::General(e.to_string()))?,
            sm.clone(),
        );

        let contract = match resolver.staking_contract().call().await {
            Ok(contract) => contract,
            Err(e) => {
                return Err(Error::Contract(e.to_string()));
            }
        };

        let staking_contract_address = match resolver.get_contract(contract, env).call().await {
            Ok(contract) => contract,
            Err(e) => {
                return Err(Error::Contract(e.to_string()));
            }
        };

        let contract = match resolver.backup_recovery_contract().call().await {
            Ok(contract) => contract,
            Err(e) => {
                return Err(Error::Contract(e.to_string()));
            }
        };

        let backup_recovery_address = match resolver.get_contract(contract, env).call().await {
            Ok(contract) => contract,
            Err(e) => {
                return Err(Error::Contract(e.to_string()));
            }
        };

        println!("Staking contract address: {}", staking_contract_address);
        println!("Backup recovery contract address: {}", backup_recovery_address);
        let backup_recovery_contract = BackupRecovery::new(backup_recovery_address, sm.clone());
        let staking_contract = Staking::new(staking_contract_address, sm.clone());
        Ok(ChainManager {
            backup_recovery: backup_recovery_contract,
            staking: staking_contract,
            signer_or_provider: sm.clone(),
        })
    }

    pub async fn get_validator_struct_for_recovery_share(&self) -> RecoveryResult<Validator> {
        let backup_parties = match self.backup_recovery.get_next_backup_party_members().call().await
        {
            Ok(addresses) => addresses,
            Err(e) => {
                let reason = lit_blockchain::util::decode_revert(&e, self.backup_recovery.abi())
                    .replace("\0,\0", "")
                    .replace("\0,", "")
                    .replace(",\0", "")
                    .replace("\0", "");
                return Err(Error::Contract(reason));
            }
        };
        println!("Backup parties: {:?}", backup_parties);

        let node_address = match self.backup_recovery.get_node_for_backup_member().call().await {
            Ok(node_address) => node_address,
            Err(e) => {
                let reason = lit_blockchain::util::decode_revert(&e, self.backup_recovery.abi())
                    .replace("\0,\0", "")
                    .replace("\0", "");
                return Err(Error::Contract(reason));
            }
        };

        let staker_mappings: Vec<AddressMapping> =
            match self.staking.get_node_staker_address_mappings(vec![node_address]).call().await {
                Ok(mappings) => {
                    println!("staker mappings: {:?}", mappings);
                    mappings
                }
                Err(e) => {
                    let reason = lit_blockchain::util::decode_revert(&e, self.staking.abi());
                    return Err(Error::Contract(reason.replace("\0", "")));
                }
            };
        let node_staker_address = staker_mappings[0].staker_address;

        self.get_validator_struct_from_staker_address(node_staker_address).await
    }

    pub async fn get_validator_struct_from_staker_address(
        &self, staker_address: H160,
    ) -> RecoveryResult<Validator> {
        println!("Getting comm address of the staker: {}", staker_address);
        let val_struct: Vec<Validator> =
            match self.staking.get_validators_structs(vec![staker_address]).call().await {
                Ok(vs) => vs,
                Err(e) => {
                    println!("Failed to get validators struct");
                    let reason = lit_blockchain::util::decode_revert(&e, self.staking.abi());
                    return Err(Error::Contract(reason.replace("\0", "")));
                }
            };

        if val_struct.is_empty() {
            return Err(Error::Contract("Could not find validator with given address".into()));
        }

        Ok(val_struct[0].clone())
    }

    pub async fn submit_pub_info_to_chain(
        &self, next_state: NextStateDownloadable,
    ) -> RecoveryResult<()> {
        let func = self
            .backup_recovery
            .receive_new_key_set(next_state.registered_recovery_keys, next_state.session_id);
        // println!("Sending txn to chain: {:?}", func.tx);
        match func.send().await {
            Ok(res) => {
                println!(
                    "Successfully uploaded pub keys with txn hash: {}",
                    hex::encode(res.tx_hash().as_bytes())
                );
            }
            Err(e) => {
                let reason = lit_blockchain::util::decode_revert(&e, self.backup_recovery.abi());
                return Err(Error::Contract(reason.replace("\0", "")));
            }
        };
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn submit_proof_bls(&self, proof_bytes: Vec<u8>) -> RecoveryResult<bool> {
        let func =
            self.backup_recovery.receive_proof_bls_12381g1(ethers::types::Bytes::from(proof_bytes));

        let tx = func.send().await;
        let res = match tx {
            Ok(tx_res) => {
                println!("Submit bls proof txn hash: {}", hex::encode(tx_res.tx_hash().as_bytes()));
                true
            }
            Err(e) => {
                return Err(crate::Error::Contract(e.to_string()));
            }
        };
        Ok(res)
    }

    #[allow(dead_code)]
    pub async fn submit_proof_ecdsa(&self, proof_bytes: Vec<u8>) -> RecoveryResult<bool> {
        let func =
            self.backup_recovery.receive_proofs_k256(ethers::types::Bytes::from(proof_bytes));
        let tx = func.send().await;
        let res = match tx {
            Ok(tx_res) => {
                println!(
                    "Submit ecdsa proof txn hash: {}",
                    hex::encode(tx_res.tx_hash().as_bytes())
                );
                true
            }
            Err(_) => {
                return Err(Error::Contract("Error while validating proof".to_string()));
            }
        };
        Ok(res)
    }

    pub async fn get_node_recovery_status(
        &self,
    ) -> RecoveryResult<Vec<NodeRecoveryStatusMapInternal>> {
        match self.backup_recovery.get_node_recovery_status().call().await {
            Ok(vec) => Ok(vec.into_iter().map(|item| item.into()).collect()),
            Err(e) => Err(Error::Contract(
                lit_blockchain::util::decode_revert(&e, self.backup_recovery.abi())
                    .replace("\0", ""),
            )),
        }
    }
}

fn _build_rpc_client(cfg: &RecoveryConfig) -> Result<Provider<Http>, Error> {
    let client = match reqwest::Client::builder().build() {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::InvalidRequest(e.to_string()));
        }
    };

    let rpc_url = cfg.get_rpc_url_or_default();

    let url = match Url::parse(rpc_url.as_str()) {
        Ok(u) => u,
        Err(e) => {
            println!("Could not build rpc url");
            return Err(Error::InvalidRequest(e.to_string()));
        }
    };
    let provider = Provider::new(Http::new_with_client(url, client));

    Ok(provider as Provider<Http>)
}

#[derive(Debug)]
pub enum NodeRecoveryStatus {
    Null,
    StartedInRestoreState,
    BackupsAreLoaded,
    AllKeysAreRestored,
    StoppedDueToNetworkState,
}

impl From<u8> for NodeRecoveryStatus {
    fn from(byte: u8) -> Self {
        match byte {
            1 => NodeRecoveryStatus::StartedInRestoreState,
            2 => NodeRecoveryStatus::BackupsAreLoaded,
            3 => NodeRecoveryStatus::AllKeysAreRestored,
            4 => NodeRecoveryStatus::StoppedDueToNetworkState,
            _ => NodeRecoveryStatus::Null,
        }
    }
}

pub struct NodeRecoveryStatusMapInternal {
    pub node_address: ::ethers::core::types::Address,
    pub status: NodeRecoveryStatus,
}

impl From<NodeRecoveryStatusMap> for NodeRecoveryStatusMapInternal {
    fn from(item: NodeRecoveryStatusMap) -> Self {
        Self { node_address: item.node_address, status: NodeRecoveryStatus::from(item.status) }
    }
}

impl fmt::Debug for NodeRecoveryStatusMapInternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", hex::encode(self.node_address.as_bytes()), self.status)
    }
}
