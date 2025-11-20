pub mod grpc_client_pool;
mod keys;
pub mod peer_item;
pub mod peer_reviewer;
pub mod peer_state;

use self::peer_state::models::SimplePeer;
use crate::config::chain::ChainDataConfigManager;
use crate::error::{EC, Result, unexpected_err, unexpected_err_code};
use crate::models::PeerValidator;
use crate::p2p_comms::web::chatter_server::chatter::chatter_service_client::ChatterServiceClient;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tasks::peer_checker::PeerCheckerMessage;
use crate::tasks::presign_manager::models::PresignMessage;
use crate::tss::common::tss_state::TssState;
use crate::version::DataVersionReader;
use ethers::prelude::{SignerMiddleware, U256};
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use ethers::utils::public_key_to_address;
use grpc_client_pool::GrpcClientPool;
use keys::get_or_generate_keys;
use lit_blockchain::contracts::backup_recovery::BackupRecovery;
use lit_blockchain::contracts::staking::{self, Staking};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::ether::middleware::EIP2771GasRelayerMiddleware;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_common::config::LitNodeConfig as _;
use lit_node_common::eth_wallet_keys::EthWalletKeys;
use lit_node_core::PeerId;
use lit_observability::channels::TracedSender;
use peer_item::PeerData;
use peer_reviewer::PeerComplaint;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::sync::{Arc, Weak};
use tonic::transport::Channel;
use tracing::instrument;
use xor_name::XorName;

#[derive(Debug)]
pub struct PeerState {
    pub id: XorName,
    pub addr: String,
    /// This needs to match the node address that is registered on the staking contract
    pub node_address: Address,
    pub staking_address: Address, // address of the contract
    pub staker_address: Address,  // address of staking wallet
    pub rpc_url: String,
    pub backup_recovery_contract:
        BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    pub staking_contract:
        Staking<EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>>,
    pub complaint_channel: TracedSender<PeerComplaint>,
    pub chain_data_config_manager: Arc<ChainDataConfigManager>,
    pub comskeys: lit_node_common::coms_keys::ComsKeys,
    /// Use a RwLock because we need explicit lifetime control over when the grpc client is dropped
    /// vs using scc::HashMap
    pub client_grpc_channels: GrpcClientPool<GrpcClient>,
    pub wallet_keys: EthWalletKeys,
    pub lit_config: Arc<ReloadableLitConfig>,
    pub ps_tx: flume::Sender<PresignMessage>,
    pub tss_state: Weak<TssState>,
    pub auto_join: bool,
    pub peer_checker_tx: flume::Sender<PeerCheckerMessage>,
}

impl PeerState {
    pub async fn new(
        addr: String,
        complaint_channel: TracedSender<PeerComplaint>,
        lit_config: Arc<ReloadableLitConfig>,
        chain_data_config_manager: Arc<ChainDataConfigManager>,
        ps_tx: flume::Sender<PresignMessage>,
        peer_checker_tx: flume::Sender<PeerCheckerMessage>,
    ) -> Result<PeerState> {
        let cfg = lit_config.load_full();

        let resolver = ContractResolver::try_from(cfg.as_ref())
            .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
        let backup_recovery_contract = resolver.backup_recovery_contract_with_signer(&cfg).await?;
        let staking_contract = resolver.staking_contract_with_signer(&cfg).await?;
        let staker_address: Address = cfg
            .staker_address()?
            .parse()
            .expect_or_err("failed to parse staker_address")?;

        // set up the attested wallet
        let (peer_keys, are_new_keys) =
            get_or_generate_keys(cfg.clone(), staking_contract.clone()).await?;
        let attested_wallet_keys = peer_keys.attested_wallet_key;
        let comskeys = peer_keys.comms_keys;
        let attested_node_address = public_key_to_address(&attested_wallet_keys.verifying_key());

        if are_new_keys {
            // Register wallet on contract using the node wallet - this should be the only time when we are using the node wallet directly to sign.
            // All other updates to the contract should be with the attested wallet, which will in turn use the node wallet as a gas relayer.
            if let Err(e) = staking_contract
                .register_attested_wallet(
                    staker_address,
                    attested_node_address,
                    ethers::types::Bytes::from(
                        attested_wallet_keys.verifying_key_uncompressed_array(),
                    ),
                    U256::from(comskeys.sender_public_key().to_bytes()),
                    U256::from(comskeys.receiver_public_key().to_bytes()),
                )
                .send()
                .await
            {
                let decoded_err = e
                    .decode_contract_revert::<staking::StakingErrors>()
                    .expect_or_err("Could not decode staking contract error")?;
                return Err(unexpected_err_code(
                    format!("{:?}", decoded_err),
                    EC::NodeBlockchainError,
                    Some("Could not register attested wallet".to_string()),
                ));
            }
            info!(
                "Registered attested wallet {:?} for staker address {:?}",
                attested_node_address, staker_address
            );
        }

        let staking_contract_with_gas_relay = resolver
            .staking_contract_with_gas_relay(&cfg, attested_wallet_keys.signing_key().clone())
            .await?;

        Ok(PeerState {
            id: XorName::from_content(addr.clone().as_bytes()),
            addr,
            node_address: attested_node_address,
            staking_address: staking_contract.address(),
            staker_address,
            rpc_url: cfg.rpc_url()?,
            backup_recovery_contract,
            staking_contract: staking_contract_with_gas_relay,
            complaint_channel,
            chain_data_config_manager,
            comskeys,
            wallet_keys: attested_wallet_keys,
            lit_config,
            ps_tx,
            client_grpc_channels: GrpcClientPool::new(cfg.clone()),
            tss_state: Weak::new(),
            auto_join: true,
            peer_checker_tx,
        })
    }

    pub async fn connected_nodes(&self) -> Arc<PeerData> {
        let (tx, rx) = flume::bounded(1);
        if let Err(e) = self
            .peer_checker_tx
            .send_async(PeerCheckerMessage::GetPeers(tx))
            .await
        {
            warn!("Error sending peer checker message: {:?}", e);
            return Arc::new(PeerData::default());
        }
        let peer_data = match rx.recv_async().await {
            Ok(peer_data) => peer_data,
            Err(e) => {
                warn!("Error receiving peer data: {:?}", e);
                return Arc::new(PeerData::default());
            }
        };
        Arc::new(peer_data)
    }

    pub fn node_address(&self) -> Address {
        self.node_address
    }

    pub fn hex_staker_address(&self) -> String {
        bytes_to_hex(self.staker_address.0)
    }

    pub fn get_tss_state(&self) -> Result<Arc<TssState>> {
        self.tss_state.upgrade().ok_or_else(|| {
            unexpected_err(
                "Failed to upgrade TssState weak reference to Arc",
                Some("TssState weak reference is dropped".to_string()),
            )
        })
    }

    pub fn epoch_length(&self, realm_id: u64) -> u64 {
        let current_realm_id = self.realm_id();

        let peers_in_current_epoch = if realm_id == current_realm_id {
            &self
                .chain_data_config_manager
                .shadow_peers
                .peers_for_current_epoch
        } else {
            &self.chain_data_config_manager.peers.peers_for_current_epoch
        };

        DataVersionReader::read_field_unchecked(peers_in_current_epoch, |peers| peers.epoch_length)
    }

    pub fn realm_id(&self) -> u64 {
        DataVersionReader::read_field_unchecked(&self.chain_data_config_manager.realm_id, |realm| {
            realm.as_u64()
        })
    }

    pub fn shadow_realm_id(&self) -> u64 {
        DataVersionReader::read_field_unchecked(
            &self.chain_data_config_manager.shadow_realm_id,
            |realm| realm.as_u64(),
        )
    }

    pub fn peer_id_in_current_epoch(&self) -> Result<PeerId> {
        let v = self.get_validator_from_node_address(self.node_address)?;
        PeerId::from_slice(&v.wallet_public_key).map_err(|e| unexpected_err(e, None))
    }

    pub fn epoch(&self) -> u64 {
        DataVersionReader::read_field_unchecked(
            &self.chain_data_config_manager.peers.peers_for_current_epoch,
            |peers| peers.epoch_number,
        )
    }

    #[instrument(level = "debug", skip(self))]
    pub fn validators_in_current_epoch(&self) -> Vec<PeerValidator> {
        DataVersionReader::new_unchecked(
            &self.chain_data_config_manager.peers.peers_for_current_epoch,
        )
        .validators
        .clone()
    }

    #[instrument(level = "debug", skip(self))]
    pub fn validators_in_next_epoch(&self) -> Vec<PeerValidator> {
        DataVersionReader::new_unchecked(&self.chain_data_config_manager.peers.peers_for_next_epoch)
            .validators
            .clone()
    }

    #[instrument(level = "debug", skip(self))]
    pub fn validators_in_prior_epoch(&self) -> Vec<PeerValidator> {
        DataVersionReader::new_unchecked(
            &self.chain_data_config_manager.peers.peers_for_prior_epoch,
        )
        .validators
        .clone()
    }

    #[instrument(level = "debug", skip(self))]
    pub fn validators_in_current_shadow_epoch(&self) -> Vec<PeerValidator> {
        DataVersionReader::new_unchecked(
            &self
                .chain_data_config_manager
                .shadow_peers
                .peers_for_current_epoch,
        )
        .validators
        .clone()
    }

    #[instrument(level = "debug", skip(self))]
    pub fn validators_in_next_shadow_epoch(&self) -> Vec<PeerValidator> {
        DataVersionReader::new_unchecked(
            &self
                .chain_data_config_manager
                .shadow_peers
                .peers_for_next_epoch,
        )
        .validators
        .clone()
    }

    pub fn validators_in_prior_epoch_current_intersection(&self) -> Vec<PeerValidator> {
        let vin_current = BTreeSet::from_iter(self.validators_in_current_epoch());
        let vin_prior = BTreeSet::from_iter(self.validators_in_prior_epoch());
        let vi_union_of_epochs = vin_current.intersection(&vin_prior);

        let validators_in_union: Vec<PeerValidator> = vi_union_of_epochs
            .filter(|v| !v.is_kicked)
            .cloned()
            .collect();
        debug!(
            "Validators in union (prior, current) ({}): {:?}",
            validators_in_union.len(),
            validators_in_union
                .iter()
                .map(|v| (
                    v.socket_addr.clone(),
                    v.address.to_string()[..6].to_string()
                ))
                .collect::<Vec<(String, String)>>()
        );

        validators_in_union
    }

    pub fn validators_in_next_epoch_current_union(&self) -> Vec<PeerValidator> {
        let vin_current = BTreeSet::from_iter(self.validators_in_current_epoch());
        let vin_next = BTreeSet::from_iter(self.validators_in_next_epoch());
        let vin_next_shadow = BTreeSet::from_iter(self.validators_in_next_shadow_epoch());
        let vi_union_of_epochs = vin_current.union(&vin_next);

        let validators_in_union: Vec<PeerValidator> = vi_union_of_epochs
            .filter(|v| !v.is_kicked)
            .cloned()
            .collect();

        let vin_current_realm = BTreeSet::from_iter(validators_in_union);
        let vi_with_shadow = vin_current_realm.union(&vin_next_shadow);

        let validators_in_union_with_shadow: Vec<PeerValidator> =
            vi_with_shadow.filter(|v| !v.is_kicked).cloned().collect();

        trace!(
            "Validators in union with shadow: {:?}",
            validators_in_union_with_shadow
                .iter()
                .map(|v| (v.socket_addr.clone(), v.address))
                .collect::<Vec<(String, Address)>>()
        );

        validators_in_union_with_shadow
    }

    pub fn peer_node_addresses(&self) -> Vec<Address> {
        self.validators_in_current_epoch()
            .iter()
            .map(|peer| peer.address)
            .collect()
    }

    pub fn peers(&self) -> SimplePeerCollection {
        self.validators_in_current_epoch()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }

    pub fn peers_in_next_epoch(&self) -> SimplePeerCollection {
        self.validators_in_next_epoch()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }

    pub fn peers_in_prior_epoch(&self) -> SimplePeerCollection {
        self.validators_in_prior_epoch()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }

    pub fn peers_in_prior_epoch_current_intersection(&self) -> SimplePeerCollection {
        self.validators_in_prior_epoch_current_intersection()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }

    pub fn peers_in_current_shadow_epoch(&self) -> SimplePeerCollection {
        self.validators_in_current_shadow_epoch()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }

    pub fn peers_in_next_shadow_epoch(&self) -> SimplePeerCollection {
        self.validators_in_next_shadow_epoch()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }

    pub fn peers_in_next_epoch_current_union_including_shadow(&self) -> SimplePeerCollection {
        self.validators_in_next_epoch_current_union()
            .iter()
            .map(SimplePeer::from)
            .collect::<Vec<SimplePeer>>()
            .into()
    }
    // get a single Validator struct
    pub fn get_validator_from_node_address(&self, node_address: Address) -> Result<PeerValidator> {
        self.get_current_and_next_validators()
            .into_iter()
            .find(|v| v.address == node_address)
            .ok_or_else(|| {
                error!(
                    "get_validator_from_node_address:Failed to find validator with address: {:?}",
                    node_address
                );
                unexpected_err("Failed to find validator with address", None)
            })
    }

    fn get_current_and_next_validators(&self) -> Vec<PeerValidator> {
        let mut validators = self.validators_in_current_epoch();
        validators.append(&mut self.validators_in_next_epoch());
        validators
    }

    pub fn get_staker_address_from_socket_addr(&self, addr: &str) -> Result<Address> {
        let validator = self
            .get_current_and_next_validators()
            .into_iter()
            .find(|v| v.socket_addr == addr);

        match validator {
            Some(v) => {
                info!(
                    "Validator with socket addr: {:?} has staker address: {:?}",
                    addr, v.staker_address
                );
                Ok(v.staker_address)
            }
            None => {
                error!("Failed to find validator with socket addr: {:?}", addr);
                Err(unexpected_err("Failed to find validator with ip", None))
            }
        }
    }
}

type GrpcClient = ChatterServiceClient<Channel>;
