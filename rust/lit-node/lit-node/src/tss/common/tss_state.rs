use super::models::{NodeTransmissionDetails, RoundData};
use super::traits::cipherable::Cipherable;
use super::traits::dkg::BasicDkg;
use super::traits::signable::Signable;
use crate::common::key_helper::KeyCache;
use crate::config::chain::ChainDataConfigManager;
use crate::error::{Result, unexpected_err};
use crate::peers::PeerState;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::blsful::models::BlsState;
use crate::tss::common::curve_state::CurveState;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::storage::read_key_share_from_disk;
use crate::tss::ecdsa_damfast::DamFastState;
use crate::tss::frost::FrostState;
use crate::version::DataVersionReader;
use flume::Receiver;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_common::config::LitNodeConfig;
use lit_node_core::{CurveType, EcdsaSignedMessageShare, SigningScheme};
use lit_observability::channels::{TracedReceiver, TracedSender, new_traced_unbounded_channel};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tracing::instrument;

const EPOCH_CHANGE_BUFFER_SECS: u64 = 60;

#[derive(Debug, Clone)]
pub struct TssState {
    pub addr: String,
    pub port: u32,
    pub threshold: Arc<AtomicUsize>,
    pub peer_state: Arc<PeerState>,
    pub lit_config: Arc<ReloadableLitConfig>,
    pub chain_data_config_manager: Arc<ChainDataConfigManager>,
    pub tx_round_manager: Arc<flume::Sender<RoundData>>,
    pub tx_batch_manager: TracedSender<NodeTransmissionDetails>,
    pub key_cache: KeyCache,
}

impl TssState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        addr: String,
        mut peer_state: Arc<PeerState>,
        port: u32,
        lit_config: Arc<ReloadableLitConfig>,
        chain_data_manager: Arc<ChainDataConfigManager>,
        tx_round_manager: Arc<flume::Sender<RoundData>>,
        tx_batch_manager: TracedSender<NodeTransmissionDetails>,
    ) -> Arc<TssState> {
        Arc::new_cyclic(|tss| {
            if let Some(peer_state) = Arc::get_mut(&mut peer_state) {
                peer_state.tss_state = tss.clone();
            }
            TssState {
                addr,
                port,
                peer_state,
                lit_config,
                chain_data_config_manager: chain_data_manager,
                tx_round_manager,
                tx_batch_manager,
                threshold: Arc::new(AtomicUsize::new(0)),
                key_cache: KeyCache::with_capacity(KeyCache::DEFAULT_CAPACITY),
            }
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn init(
        peer_state: Arc<PeerState>,
        lit_config: Arc<ReloadableLitConfig>,
        chain_data_manager: Arc<ChainDataConfigManager>,
    ) -> Result<(
        Arc<TssState>,
        Receiver<RoundData>,
        TracedReceiver<NodeTransmissionDetails>,
    )> {
        let config = lit_config.load_full();
        let addr = config
            .external_addr()
            .expect_or_err("No node address set in config.")?; // expect ok, only called from main.rs
        let port = config
            .external_port()
            .expect_or_err("No external port set in config.")? as u32; // expect ok, only called from main.rs

        let (tx_round_manager, rx_round_manager) = flume::unbounded();
        let (tx_batch_manager, rx_batch_manager) = new_traced_unbounded_channel();
        let tx_round_manager = Arc::new(tx_round_manager);

        let tss_state = Self::new(
            addr,
            peer_state,
            port,
            lit_config,
            chain_data_manager,
            tx_round_manager,
            tx_batch_manager,
        );

        Ok((tss_state, rx_round_manager, rx_batch_manager))
    }

    pub fn init_channels(
        addr: String,
        peer_state: Arc<PeerState>,
        port: u32,
        lit_config: Arc<ReloadableLitConfig>,
        chain_data_manager: Arc<ChainDataConfigManager>,
    ) -> (
        Arc<TssState>,
        Receiver<RoundData>,
        TracedReceiver<NodeTransmissionDetails>,
    ) {
        let (tx_round_manager, rx_round_manager) = flume::unbounded();
        let (tx_batch_manager, rx_batch_manager) = new_traced_unbounded_channel();
        let tx_round_manager = Arc::new(tx_round_manager);
        (
            Self::new(
                addr,
                peer_state,
                port,
                lit_config,
                chain_data_manager,
                tx_round_manager,
                tx_batch_manager,
            ),
            rx_round_manager,
            rx_batch_manager,
        )
    }

    #[instrument(level = "debug", skip(self))]
    pub fn get_signing_state(&self, signing_scheme: SigningScheme) -> Result<Box<dyn Signable>> {
        let state = Arc::new(self.clone());
        let signing_state = match signing_scheme {
            SigningScheme::EcdsaK256Sha256
            | SigningScheme::EcdsaP256Sha256
            | SigningScheme::EcdsaP384Sha384 => {
                Box::new(DamFastState::new(state, signing_scheme)) as Box<dyn Signable>
            }
            SigningScheme::SchnorrK256Sha256
            | SigningScheme::SchnorrK256Taproot
            | SigningScheme::SchnorrP256Sha256
            | SigningScheme::SchnorrP384Sha384
            | SigningScheme::SchnorrEd25519Sha512
            | SigningScheme::SchnorrRistretto25519Sha512
            | SigningScheme::SchnorrEd448Shake256
            | SigningScheme::SchnorrRedJubjubBlake2b512
            | SigningScheme::SchnorrRedDecaf377Blake2b512
            | SigningScheme::SchnorrkelSubstrate => {
                Box::new(FrostState::new(state, signing_scheme)) as Box<dyn Signable>
            }
            SigningScheme::Bls12381G1ProofOfPossession => {
                Box::new(BlsState::new(state, signing_scheme)) as Box<dyn Signable>
            }
            _ => {
                return Err(unexpected_err(
                    "Unsupported key type when for Signable.",
                    None,
                ));
            }
        };

        Ok(signing_state)
    }

    pub fn get_cipher_state(&self, signing_scheme: SigningScheme) -> Result<Box<dyn Cipherable>> {
        let state = Arc::new(self.clone());
        let cipher_state = match signing_scheme {
            SigningScheme::Bls12381 => {
                Box::new(BlsState::new(state, signing_scheme)) as Box<dyn Cipherable>
            }
            _ => {
                return Err(unexpected_err(
                    "Unsupported key type when casting to Cipherable trait.",
                    None,
                ));
            }
        };

        Ok(cipher_state)
    }

    #[instrument(level = "debug", skip(self))]
    pub fn get_dkg_state(&self, curve_type: CurveType) -> Result<Box<dyn BasicDkg>> {
        let state = Arc::new(self.clone());
        Ok(Box::new(CurveState { state, curve_type }) as Box<dyn BasicDkg>)
    }

    pub async fn get_threshold_using_current_epoch_realm_peers_for_curve(
        &self,
        peers: &SimplePeerCollection,
        curve_type: CurveType,
        epoch: Option<u64>,
    ) -> Result<usize> {
        let self_peer = peers.peer_at_address(&self.addr)?;

        let dkg_state = self.get_dkg_state(curve_type)?;
        let root_keys = dkg_state.root_keys().await;

        if root_keys.is_empty() {
            return Err(unexpected_err(
                format!("No root keys exist for curve: {}", curve_type),
                None,
            ));
        }

        let staker_address = &bytes_to_hex(self_peer.staker_address.as_bytes());
        let realm_id = self.peer_state.realm_id();
        let epoch = match epoch {
            Some(e) => e,
            None => self.peer_state.epoch(),
        };

        let keyshare = read_key_share_from_disk::<KeyShare>(
            curve_type,
            &root_keys[0],
            staker_address,
            &self_peer.peer_id,
            epoch,
            realm_id,
            &self.key_cache,
        )
        .await
        .map_err(|e| {
            unexpected_err(
                e,
                Some(format!(
                    "Could not read key share (index/epoch) {}/{} from disk",
                    self_peer.peer_id, epoch,
                )),
            )
        })?;
        Ok(keyshare.threshold)
    }

    pub fn failed_message_share(&self) -> EcdsaSignedMessageShare {
        EcdsaSignedMessageShare {
            digest: "fail".to_string(),
            result: "fail".to_string(),
            signature_share: "".to_string(),
            share_id: "".to_string(),
            peer_id: "".to_string(),
            big_r: "".to_string(),
            compressed_public_key: "".to_string(),
            public_key: "".to_string(),
            sig_type: "".to_string(),
        }
    }

    pub async fn get_keyshare_epoch(&self) -> u64 {
        let (epoch_number, epoch_read_time) = DataVersionReader::read_field_unchecked(
            &self.chain_data_config_manager.peers.peers_for_current_epoch,
            |current_peers| (current_peers.epoch_number, current_peers.epoch_read_time),
        );
        // 2 is actually the first epoch (no keys are present in epoch 1)
        if epoch_number <= 2 {
            return 2;
        };

        let elapsed = match std::time::SystemTime::now().duration_since(epoch_read_time) {
            Ok(elapsed) => elapsed.as_secs(),
            Err(e) => {
                error!("Error getting elapsed time: {:?}", e);
                EPOCH_CHANGE_BUFFER_SECS + 1 // automatically use the current epoch?  This probably shouldn't happen.
            }
        };

        // if we've held the epoch in memory for at least 60 seconds, use the current epoch, otherwise use the prior one.
        if elapsed > EPOCH_CHANGE_BUFFER_SECS {
            epoch_number
        } else {
            epoch_number - 1
        }
    }

    pub async fn get_threshold(&self) -> usize {
        let threshold = self.threshold.load(Ordering::Acquire);
        if threshold > 0 {
            return threshold;
        }
        let peers = self.peer_state.peers();

        // if there are no peers, return 0.  This is a special case for the first epoch.
        if peers.0.is_empty() {
            return 0;
        }

        let curve_type = CurveType::K256;
        let epoch = self.get_keyshare_epoch().await;
        let rt = match self
            .get_threshold_using_current_epoch_realm_peers_for_curve(
                &peers,
                curve_type,
                Some(epoch),
            )
            .await
        {
            Ok(t) => t,
            Err(e) => {
                warn!("Error getting threshold: {:?}", e);
                return 0;
            }
        };
        self.set_threshold(rt);
        rt
    }

    pub fn set_threshold(&self, threshold: usize) {
        self.threshold.store(threshold, Ordering::Release);
    }
}
