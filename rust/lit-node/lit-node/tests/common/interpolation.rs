use lit_core::utils::binary::bytes_to_hex;
use lit_node::common::key_helper::KeyCache;
use lit_node::error::Result;
use lit_node::peers::peer_state::models::{SimplePeer, SimplePeerCollection};
use lit_node::tss::common::key_persistence::KeyPersistence;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::common::storage::{read_key_share_from_disk, write_key_share_to_disk};
use lit_node_core::{CompressedBytes, CurveType, PeerId};
use lit_rust_crypto::{
    blsful::inner_types::{G1Projective, Scalar},
    decaf377, ed448_goldilocks,
    ff::PrimeField,
    group::{Group, GroupEncoding},
    jubjub, k256, p256, p384, pallas, vsss_rs,
    vsss_rs::{
        DefaultShare, IdentifierPrimeField, ReadableShareSet, ValuePrimeField,
        curve25519::{WrappedEdwards, WrappedRistretto, WrappedScalar},
    },
};

pub async fn get_secret_and_shares<G>(
    curve_type: CurveType,
    pubkey: &str,
    peers: &SimplePeerCollection,
    epoch: u64,
    realm_id: u64,
) -> (
    CurveScalar,
    Vec<DefaultShare<IdentifierPrimeField<G::Scalar>, ValuePrimeField<G::Scalar>>>,
)
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let secret = interpolate_secret(curve_type, peers, pubkey, epoch, realm_id).await;
    let shares = load_secret_shares::<G>(curve_type, pubkey, peers, epoch, realm_id).await;
    (secret, shares)
}

#[derive(Copy, Clone, Debug)]
pub enum CurveScalar {
    Bls(Scalar),
    K256(k256::Scalar),
    P256(p256::Scalar),
    P384(p384::Scalar),
    Ed25519(WrappedScalar),
    Ristretto25519(WrappedScalar),
    Ed448(ed448_goldilocks::Scalar),
    Jubjub(jubjub::Scalar),
    Pallas(pallas::Scalar),
    Decaf377(decaf377::Fr),
    Schnorrkel(WrappedScalar),
}

impl Eq for CurveScalar {}

impl PartialEq for CurveScalar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bls(a), Self::Bls(b)) => a == b,
            (Self::K256(a), Self::K256(b)) => a == b,
            (Self::P256(a), Self::P256(b)) => a == b,
            (Self::P384(a), Self::P384(b)) => a == b,
            (Self::Ed25519(a), Self::Ed25519(b)) => a == b,
            (Self::Ristretto25519(a), Self::Ristretto25519(b)) => a == b,
            (Self::Ed448(a), Self::Ed448(b)) => a == b,
            (Self::Jubjub(a), Self::Jubjub(b)) => a == b,
            (Self::Decaf377(a), Self::Decaf377(b)) => a == b,
            (Self::Schnorrkel(a), Self::Schnorrkel(b)) => a == b,
            (Self::Pallas(a), Self::Pallas(b)) => a == b,
            _ => false,
        }
    }
}

impl From<Scalar> for CurveScalar {
    fn from(scalar: Scalar) -> Self {
        Self::Bls(scalar)
    }
}

impl From<k256::Scalar> for CurveScalar {
    fn from(scalar: k256::Scalar) -> Self {
        Self::K256(scalar)
    }
}

impl From<p256::Scalar> for CurveScalar {
    fn from(scalar: p256::Scalar) -> Self {
        Self::P256(scalar)
    }
}

impl From<p384::Scalar> for CurveScalar {
    fn from(scalar: p384::Scalar) -> Self {
        Self::P384(scalar)
    }
}

impl From<WrappedScalar> for CurveScalar {
    fn from(scalar: WrappedScalar) -> Self {
        Self::Ed25519(scalar)
    }
}

impl From<ed448_goldilocks::Scalar> for CurveScalar {
    fn from(scalar: ed448_goldilocks::Scalar) -> Self {
        Self::Ed448(scalar)
    }
}

impl From<jubjub::Scalar> for CurveScalar {
    fn from(scalar: jubjub::Scalar) -> Self {
        Self::Jubjub(scalar)
    }
}

impl CurveScalar {
    pub(crate) fn to_bytes(self) -> Vec<u8> {
        let repr: Box<dyn AsRef<[u8]>> = match self {
            Self::Bls(scalar) => Box::new(scalar.to_repr()),
            Self::K256(scalar) => Box::new(scalar.to_repr()),
            Self::P256(scalar) => Box::new(scalar.to_repr()),
            Self::P384(scalar) => Box::new(scalar.to_repr()),
            Self::Ed25519(scalar) => Box::new(scalar.to_repr()),
            Self::Ristretto25519(scalar) => Box::new(scalar.to_repr()),
            Self::Ed448(scalar) => Box::new(scalar.to_repr()),
            Self::Jubjub(scalar) => Box::new(scalar.to_repr()),
            Self::Decaf377(scalar) => Box::new(scalar.to_repr()),
            Self::Pallas(scalar) => Box::new(scalar.to_repr()),
            Self::Schnorrkel(scalar) => Box::new(scalar.to_repr()),
        };
        (*repr).as_ref().to_vec()
    }
}

pub async fn remap_secret_to_new_peer_ids(
    curve_type: CurveType,
    old_peers: &SimplePeerCollection,
    new_peers: &SimplePeerCollection,
    pubkey: &str,
    read_epoch: u64,
    write_epoch: u64,
) -> Result<()> {
    let realm_id = 1;
    match curve_type {
        CurveType::BLS => {
            remap_secret_helper::<G1Projective>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::K256 => {
            remap_secret_helper::<k256::ProjectivePoint>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::P256 => {
            remap_secret_helper::<p256::ProjectivePoint>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::Ed25519 => {
            remap_secret_helper::<WrappedEdwards>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::Ed448 => {
            remap_secret_helper::<ed448_goldilocks::EdwardsPoint>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::P384 => {
            remap_secret_helper::<p384::ProjectivePoint>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::Ristretto25519 => {
            remap_secret_helper::<WrappedRistretto>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::RedJubjub => {
            remap_secret_helper::<jubjub::SubgroupPoint>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::RedDecaf377 => {
            remap_secret_helper::<decaf377::Element>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::BLS12381G1 => {
            remap_secret_helper::<G1Projective>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
        CurveType::RedPallas => {
            remap_secret_helper::<pallas::Point>(
                curve_type,
                old_peers,
                new_peers,
                pubkey,
                read_epoch,
                write_epoch,
                realm_id,
            )
            .await
        }
    }
}

async fn remap_secret_helper<G>(
    curve_type: CurveType,
    old_peers: &SimplePeerCollection,
    new_peers: &SimplePeerCollection,
    pubkey: &str,
    read_epoch: u64,
    write_epoch: u64,
    realm_id: u64,
) -> Result<()>
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let mut threshold = 0;
    let mut dkg_id = String::new();
    let mut old_shares = Vec::with_capacity(old_peers.0.len());
    for peer in old_peers.0.iter() {
        let key_share = read_key_share_from_disk_from_test_harness::<KeyShare>(
            peer, pubkey, read_epoch, curve_type, realm_id,
        )
        .await
        .expect("Failed to load key share");
        let private_share = key_share.secret::<G>()?;
        let share = DefaultShare {
            identifier: IdentifierPrimeField(G::Scalar::from(key_share.peer_id)),
            value: IdentifierPrimeField(private_share),
        };
        threshold = key_share.threshold;
        dkg_id = key_share.txn_prefix.clone();
        old_shares.push(share);
    }
    let secret = old_shares.combine().unwrap();
    let ids = new_peers
        .0
        .iter()
        .map(|peer| IdentifierPrimeField(G::Scalar::from(peer.peer_id)))
        .collect::<Vec<_>>();
    let participant_generator = vsss_rs::ParticipantIdGeneratorType::list(&ids);
    let new_shares = vsss_rs::shamir::split_secret_with_participant_generator::<
        DefaultShare<IdentifierPrimeField<G::Scalar>, ValuePrimeField<G::Scalar>>,
    >(
        threshold,
        new_peers.0.len(),
        &secret,
        rand::thread_rng(),
        &[participant_generator],
    )
    .unwrap();
    for (share, peer) in new_shares.iter().zip(new_peers.0.iter()) {
        write_key_share_to_disk_from_test_harness::<G>(
            peer,
            new_peers,
            pubkey,
            write_epoch,
            threshold,
            new_peers.0.len(),
            &dkg_id,
            curve_type,
            share.value.0,
            realm_id,
        )
        .await?;
    }
    Ok(())
}

pub async fn interpolate_secret(
    curve_type: CurveType,
    peers: &SimplePeerCollection,
    pubkey: &str,
    epoch: u64,
    realm_id: u64,
) -> CurveScalar {
    match curve_type {
        CurveType::BLS => CurveScalar::Bls(
            interpolate_secret_for_key::<G1Projective>(peers, pubkey, epoch, curve_type, realm_id)
                .await,
        ),
        CurveType::K256 => CurveScalar::K256(
            interpolate_secret_for_key::<k256::ProjectivePoint>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::P256 => CurveScalar::P256(
            interpolate_secret_for_key::<p256::ProjectivePoint>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::Ed25519 => CurveScalar::Ed25519(
            interpolate_secret_for_key::<WrappedEdwards>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::Ed448 => CurveScalar::Ed448(
            interpolate_secret_for_key::<ed448_goldilocks::EdwardsPoint>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::P384 => CurveScalar::P384(
            interpolate_secret_for_key::<p384::ProjectivePoint>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::Ristretto25519 => CurveScalar::Ristretto25519(
            interpolate_secret_for_key::<WrappedRistretto>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::RedJubjub => CurveScalar::Jubjub(
            interpolate_secret_for_key::<jubjub::SubgroupPoint>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::RedDecaf377 => CurveScalar::Decaf377(
            interpolate_secret_for_key::<decaf377::Element>(
                peers, pubkey, epoch, curve_type, realm_id,
            )
            .await,
        ),
        CurveType::BLS12381G1 => CurveScalar::Bls(
            interpolate_secret_for_key::<G1Projective>(peers, pubkey, epoch, curve_type, realm_id)
                .await,
        ),
        CurveType::RedPallas => CurveScalar::Pallas(
            interpolate_secret_for_key::<pallas::Point>(peers, pubkey, epoch, curve_type, realm_id)
                .await,
        ),
    }
}

pub fn splice_secret(
    secret: CurveScalar,
    peers: &SimplePeerCollection,
    threshold: usize,
) -> Vec<(PeerId, CurveScalar)> {
    match secret {
        CurveScalar::K256(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::K256),
        CurveScalar::P256(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::P256),
        CurveScalar::P384(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::P384),
        CurveScalar::Ed25519(s) => {
            split_secret_with_peers(s, peers, threshold, CurveScalar::Ed25519)
        }
        CurveScalar::Ristretto25519(s) => {
            split_secret_with_peers(s, peers, threshold, CurveScalar::Ristretto25519)
        }
        CurveScalar::Ed448(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::Ed448),
        CurveScalar::Jubjub(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::Jubjub),
        CurveScalar::Decaf377(s) => {
            split_secret_with_peers(s, peers, threshold, CurveScalar::Decaf377)
        }
        CurveScalar::Bls(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::Bls),
        CurveScalar::Schnorrkel(s) => {
            split_secret_with_peers(s, peers, threshold, CurveScalar::Schnorrkel)
        }
        CurveScalar::Pallas(s) => split_secret_with_peers(s, peers, threshold, CurveScalar::Pallas),
    }
}

fn split_secret_with_peers<F, M>(
    secret: F,
    peers: &SimplePeerCollection,
    threshold: usize,
    mapper: M,
) -> Vec<(PeerId, CurveScalar)>
where
    F: PrimeField + From<PeerId>,
    M: Fn(F) -> CurveScalar,
{
    let limit = peers.0.len();
    let inner_secret = IdentifierPrimeField(secret);
    let participant_ids: Vec<IdentifierPrimeField<F>> = peers
        .0
        .iter()
        .map(|peer| IdentifierPrimeField(F::from(peer.peer_id)))
        .collect();
    let participant_generator = vsss_rs::ParticipantIdGeneratorType::list(&participant_ids);
    let shares = vsss_rs::shamir::split_secret_with_participant_generator::<
        DefaultShare<IdentifierPrimeField<F>, ValuePrimeField<F>>,
    >(
        threshold,
        limit,
        &inner_secret,
        rand::thread_rng(),
        &[participant_generator],
    )
    .unwrap();
    shares
        .into_iter()
        .zip(peers.0.iter())
        .map(|(share, peer)| (peer.peer_id, mapper(share.value.0)))
        .collect()
}

pub async fn load_secret_shares<G>(
    curve_type: CurveType,
    pubkey: &str,
    peers: &SimplePeerCollection,
    epoch: u64,
    realm_id: u64,
) -> Vec<DefaultShare<IdentifierPrimeField<G::Scalar>, ValuePrimeField<G::Scalar>>>
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let mut shares = Vec::with_capacity(peers.0.len());

    for peer in &peers.0 {
        let (_, share, _, _) = load_key_share::<G>(peer, pubkey, epoch, curve_type, realm_id).await;
        shares.push(share);
    }

    shares
}

// this function duplicates how a node determines key folders. Because the test doesn't run in the context
// of a node (unless it's a component test), we need to use the staker address as a param to find the folder.
pub async fn read_key_share_from_disk_from_test_harness<T>(
    peer: &SimplePeer,
    pubkey: &str,
    epoch: u64,
    curve_type: CurveType,
    realm_id: u64,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let staker_address = bytes_to_hex(peer.staker_address.as_bytes());
    let key_cache = KeyCache::default();
    read_key_share_from_disk(
        curve_type,
        pubkey,
        &staker_address,
        &peer.peer_id,
        epoch,
        realm_id,
        &key_cache,
    )
    .await
}

pub async fn write_key_share_to_disk_from_test_harness<G>(
    peer: &SimplePeer,
    peers: &SimplePeerCollection,
    pubkey: &str,
    epoch: u64,
    threshold: usize,
    total_shares: usize,
    dkg_id: &str,
    curve_type: CurveType,
    key_share: G::Scalar,
    realm_id: u64,
) -> Result<()>
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: CompressedBytes,
{
    let staker_address = bytes_to_hex(peer.staker_address.as_bytes());
    let key_cache = KeyCache::default();
    let persistence = KeyPersistence::<G>::new(curve_type);
    let public_key = persistence.pk_from_hex(&pubkey)?;
    let local_key = KeyShare::new(
        key_share,
        public_key,
        curve_type,
        &peer.peer_id,
        peers,
        threshold,
        total_shares,
        dkg_id.to_string(),
        realm_id,
    )?;
    write_key_share_to_disk(
        curve_type,
        pubkey,
        &staker_address,
        &peer.peer_id,
        epoch,
        realm_id,
        &key_cache,
        &local_key,
    )
    .await
}

pub async fn interpolate_secret_for_key<G>(
    peers: &SimplePeerCollection,
    pubkey: &str,
    epoch: u64,
    curve_type: CurveType,
    realm_id: u64,
) -> <G as Group>::Scalar
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let mut shares = Vec::with_capacity(peers.0.len());
    let mut threshold = 0;
    for peer in &peers.0 {
        let (_identifier, private_share, _public_key, share_threshold) =
            load_key_share::<G>(peer, pubkey, epoch, curve_type, realm_id).await;
        if threshold == 0 {
            threshold = share_threshold as usize;
        }
        shares.push(private_share);
    }

    interpolate_secret_from_shares::<G>(threshold, &shares)
}

pub async fn load_key_share<G>(
    peer: &SimplePeer,
    pubkey: &str,
    epoch: u64,
    curve_type: CurveType,
    realm_id: u64,
) -> (
    PeerId,
    DefaultShare<IdentifierPrimeField<G::Scalar>, ValuePrimeField<G::Scalar>>,
    G,
    usize,
)
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let key_share = read_key_share_from_disk_from_test_harness::<KeyShare>(
        peer, pubkey, epoch, curve_type, realm_id,
    )
    .await
    .expect("Failed to load key share");

    let private_share = key_share.secret::<G>().unwrap();
    let public_key = key_share.public_key::<G>().unwrap();
    let share = DefaultShare {
        identifier: IdentifierPrimeField(G::Scalar::from(key_share.peer_id)),
        value: IdentifierPrimeField(private_share),
    };
    (key_share.peer_id, share, public_key, key_share.threshold)
}

pub fn interpolate_secret_from_shares<G>(
    threshold: usize,
    shares: &[DefaultShare<IdentifierPrimeField<G::Scalar>, ValuePrimeField<G::Scalar>>],
) -> <G as Group>::Scalar
where
    G: Group + GroupEncoding + Default,
{
    tracing::info!("Interpolating secret from shares: {:?} ", shares);
    let secret = (&shares[..threshold]).combine();

    assert!(secret.is_ok());

    secret.unwrap().0
}
