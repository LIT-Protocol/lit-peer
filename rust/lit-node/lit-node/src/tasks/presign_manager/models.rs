use crate::error::{Result, unexpected_err};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::tss_state::TssState;
use crate::utils::traits::SignatureCurve;
use flume::{Receiver, Sender};
use lit_fast_ecdsa::PreSignature;
use lit_node_core::{
    CurveType, PeerId, SigningScheme,
    hd_keys_curves_wasm::{HDDerivable, HDDeriver},
};
use lit_rust_crypto::{
    elliptic_curve::{CurveArithmetic, PrimeCurve},
    group::GroupEncoding,
    k256, p256, p384,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::Arc;
use std::{collections::VecDeque, hash::Hash};

#[derive(Debug)]
pub struct XorFilterWithThreshold {
    pub filter: xorf::BinaryFuse16,
    pub threshold: usize,
}

#[derive(Debug)]
pub struct PresignManager {
    pub min_presigns: u64,
    pub max_presigns: u64,
    pub max_presign_concurrency: u64,
    pub rx: Receiver<PresignMessage>,
    pub tx: Sender<PresignMessage>,
    pub tss_state: Arc<TssState>,
    pub current_generation_count: [u64; 3],
    pub generating_txn_ids: Vec<u64>,
    pub last_generated: [std::time::Instant; 3], // used to throttle generation
    pub xor_filters: HashMap<PeerGroupId, XorFilterWithThreshold>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresignPeerId {
    pub staker_hash: u64,
    pub peer_id: PeerId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Presign {
    pub share: PreSignatureValue,
    // hash of the peers used to generate the pairs
    pub peer_group_id: PeerGroupId,
    pub xor_filter: xorf::BinaryFuse16,
    // added to use across epochs (staker_hash & peer_ids)
    pub staker_hash: u64,
    pub peer_ids: Vec<PresignPeerId>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PreSignatureValue {
    K256(PreSignature<k256::Secp256k1>),
    P256(PreSignature<p256::NistP256>),
    P384(PreSignature<p384::NistP384>),
}

impl From<PreSignature<k256::Secp256k1>> for PreSignatureValue {
    fn from(p: PreSignature<k256::Secp256k1>) -> Self {
        Self::K256(p)
    }
}

impl From<PreSignature<p256::NistP256>> for PreSignatureValue {
    fn from(p: PreSignature<p256::NistP256>) -> Self {
        Self::P256(p)
    }
}

impl From<PreSignature<p384::NistP384>> for PreSignatureValue {
    fn from(p: PreSignature<p384::NistP384>) -> Self {
        Self::P384(p)
    }
}

impl PreSignatureValue {
    /// WARNING: be sure to use the correct generic when calling this method.
    pub fn unwrap<C>(&self) -> &PreSignature<C>
    where
        C: PrimeCurve + CurveArithmetic + SignatureCurve,
        C::ProjectivePoint: GroupEncoding + HDDerivable,
        C::Scalar: HDDeriver,
    {
        assert_eq!(C::CURVE_TYPE, self.curve_type());
        // Use unsafe here because in reality it should just be converting to itself
        // so this is really just to satisfy the type checker
        // plus the assert checks that the curve type is correct
        match self {
            Self::K256(p) => unsafe {
                std::mem::transmute::<&PreSignature<k256::Secp256k1>, &PreSignature<C>>(p)
            },
            Self::P256(p) => unsafe {
                std::mem::transmute::<&PreSignature<p256::NistP256>, &PreSignature<C>>(p)
            },
            Self::P384(p) => unsafe {
                std::mem::transmute::<&PreSignature<p384::NistP384>, &PreSignature<C>>(p)
            },
        }
    }

    pub fn threshold(&self) -> usize {
        match self {
            Self::K256(p) => p.threshold,
            Self::P256(p) => p.threshold,
            Self::P384(p) => p.threshold,
        }
    }
    pub fn tag(&self) -> String {
        match self {
            Self::K256(p) => p.tag(),
            Self::P256(p) => p.tag(),
            Self::P384(p) => p.tag(),
        }
    }
    pub fn curve_type(&self) -> CurveType {
        match self {
            Self::K256(_) => CurveType::K256,
            Self::P256(_) => CurveType::P256,
            Self::P384(_) => CurveType::P384,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignRequest {
    pub message_bytes: Vec<u8>,
    pub request_id: Vec<u8>,
    pub txn_prefix: String,
    pub peers: SimplePeerCollection,
    pub signing_scheme: SigningScheme,
    pub threshold: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct PresignRequestKey {
    pub message_bytes: Vec<u8>,
    pub request_id: Vec<u8>,
}

#[derive(Debug)]
pub enum PresignMessage {
    Generate(u64, SimplePeerCollection, CurveType),
    Clear,
    Count,
    Store(Box<Presign>, u64),
    RequestPresign(PresignRequest, Sender<Result<Option<Presign>>>),
    FullfillPresignRequest(
        PresignRequestKeyHash,
        PresignLeaderResponse,
        SimplePeerCollection,
        Sender<Result<Option<Presign>>>,
        SigningScheme,
    ),
    InformNonParticipants(u64, SimplePeerCollection),
    PregenVerified(u64, SimplePeerCollection, u64, CurveType),
    RemoveGenerationHash(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PregenSignal {
    pub request_hash: u64,
    pub curve_type: CurveType,
    pub remaining_presigns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PregenConfirmation {}

#[derive(Debug, Clone)]
pub enum TxnPrefix {
    PregenSignal,
    GetPresign(u64),
    PregenPresign(u64, CurveType),
    ConfirmPregenPresign(u64),
    RealTimePresign(u64, CurveType),
}

impl TxnPrefix {
    pub fn as_str(&self) -> String {
        match self {
            Self::GetPresign(hash) => format!("GET_PRESIGN_{}", hash),
            Self::PregenSignal => "PREGEN_SIGNAL".to_string(),
            Self::PregenPresign(hash, curve_type) => {
                format!("PREGEN_PRESIGN_{}_{}", hash, curve_type)
            }
            Self::ConfirmPregenPresign(hash) => format!("CONFIRM_PREGEN_PRESIGN_{}", hash),
            Self::RealTimePresign(hash, curve_type) => {
                format!("RT_PRESIGN_{}_{}", hash, curve_type)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PresignLeaderResponse {
    pub presign_storage_key: PresignStorageKey,
    pub remaining_presigns: u64,
}

// hash of a presign request key
pub type PresignRequestKeyHash = u64;
// hash of peers used to generate a presign
pub type PeerGroupId = u64;
pub type PresignStorageKey = String;
pub type PresignList = VecDeque<String>;
// list of presign
pub type PresignListByGroup = HashMap<PeerGroupId, PresignList>;

pub type LeaderPresignRequestRemaining = u64;

pub trait PresignListByGroupTrait {
    fn add_storage_key(&mut self, key: PeerGroupId, value: PresignStorageKey);
    fn total_shares_count(&self) -> u64;
    fn shares_count_for_peerset(&self, key: u64) -> u64;
    fn presign_list_values(&self) -> VecDeque<String>;
}

impl PresignListByGroupTrait for PresignListByGroup {
    fn add_storage_key(&mut self, key: PeerGroupId, value: PresignStorageKey) {
        self.entry(key)
            .and_modify(|v| v.push_back(value.clone()))
            .or_insert(VecDeque::from(vec![value]));
    }

    fn total_shares_count(&self) -> u64 {
        let mut total = 0;
        for (_, v) in self.iter() {
            total += v.len() as u64;
        }
        total
    }
    fn shares_count_for_peerset(&self, key: u64) -> u64 {
        match self.get(&key) {
            None => 0,
            Some(v) => v.len() as u64,
        }
    }

    fn presign_list_values(&self) -> VecDeque<String> {
        self.values().flatten().cloned().collect()
    }
}

pub trait SimpleHash {
    fn hash(&self) -> u64;
}

impl SimpleHash for PresignRequestKey {
    fn hash(&self) -> u64 {
        generate_hash(self)
    }
}

pub fn generate_hash<T: Hash>(input: T) -> u64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish()
}

impl PresignRequest {
    pub fn get_presign_request_key(&self) -> PresignRequestKey {
        PresignRequestKey {
            message_bytes: self.message_bytes.clone(),
            request_id: self.request_id.clone(),
        }
    }
}

impl Presign {
    pub fn new(
        share: PreSignatureValue,
        staker_hash: u64,
        peers: &SimplePeerCollection,
    ) -> Result<Self> {
        let peer_group_id = peers.peer_group_id();
        let peer_keys: Vec<u64> = peers.peer_keys();

        let xor_filter = match xorf::BinaryFuse16::try_from(&peer_keys) {
            Ok(f) => f,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Could not create xor filter from peer keys".into()),
                ));
            }
        };

        let mut peer_ids = Vec::with_capacity(peers.0.len());
        for peer in &peers.0 {
            peer_ids.push(PresignPeerId {
                staker_hash: peer.key_hash,
                peer_id: peer.peer_id,
            });
        }

        Ok(Presign {
            share,
            peer_group_id,
            xor_filter,
            staker_hash,
            peer_ids,
        })
    }

    pub fn ids_from_peers(&self, peers: &SimplePeerCollection) -> Vec<PeerId> {
        let mut peer_ids = Vec::with_capacity(peers.0.len());

        // get peer_ids from self.peer_ids where staker_hash matches a peer in peers
        for peer in &peers.0 {
            for bt_peer_id in &self.peer_ids {
                if bt_peer_id.staker_hash == peer.key_hash {
                    peer_ids.push(bt_peer_id.peer_id);
                }
            }
        }

        peer_ids
    }
}
