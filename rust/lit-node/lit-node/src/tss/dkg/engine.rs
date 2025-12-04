use crate::common::key_helper::KeyCache;
use crate::config::chain::CachedRootKey;
use crate::error::{Result, unexpected_err};
use crate::metrics;
use crate::p2p_comms::CommsManager;
use crate::peers::peer_state::models::{SimplePeer, SimplePeerCollection};
use crate::tasks::fsm::epoch_change::ShadowOptions;
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::key_persistence::{KeyPersistence, RECOVERY_DKG_EPOCH};
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::key_share_commitment::KeyShareCommitments;
use crate::tss::common::storage::{
    delete_key_share_commitments_older_than_epoch, delete_keyshares_older_than_epoch,
    read_key_share_commitments_from_disk, read_key_share_from_disk,
    write_key_share_commitments_to_disk,
};
use crate::tss::common::tss_state::TssState;
use crate::tss::dkg::models::{DkgOutput, Mode};
use crate::version::DataVersionReader;
use elliptic_curve::group::GroupEncoding;
use frost_dkg::elliptic_curve_tools::SumOfProducts;
use frost_dkg::*;
use lit_blockchain::contracts::backup_recovery::RecoveredPeerId;
use lit_core::error::Unexpected;
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use lit_node_core::{CompressedBytes, CompressedHex};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Values;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::num::NonZeroUsize;
use std::sync::Arc;
use tracing::instrument;
use vsss_rs::{DefaultShare, IdentifierPrimeField, ParticipantIdGeneratorType};

const MIN_EPOCH_FOR_COMMITMENT_DELETION: u64 = 1;
#[derive(Clone, Debug)]
pub struct DkgEngine {
    tss_state: Arc<TssState>,
    dkgs: BTreeMap<String, DkgData>,
    dkg_type: DkgType,
    epoch: u64,
    threshold: usize,
    shadow_key_opts: ShadowOptions,
    current_peers: SimplePeerCollection,
    next_peers: SimplePeerCollection,
    next_dkg_after_restore: DkgAfterRestore,
}

#[derive(Clone, Debug)]
pub enum DkgAfterRestore {
    True(DkgAfterRestoreData),
    False,
}

impl DkgAfterRestore {
    pub fn value(&self) -> bool {
        match self {
            DkgAfterRestore::True(_) => true,
            DkgAfterRestore::False => false,
        }
    }
    pub fn take(&mut self) -> Option<DkgAfterRestoreData> {
        match std::mem::replace(self, DkgAfterRestore::False) {
            DkgAfterRestore::False => None,
            DkgAfterRestore::True(data) => Some(data),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DkgAfterRestoreData {
    pub peers: Vec<RecoveredPeerId>,
    pub key_cache: KeyCache,
}

impl DkgEngine {
    /// Create a new DkgManager instance
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tss_state: Arc<TssState>,
        dkg_type: DkgType,
        epoch: u64,
        threshold: usize,
        shadow_key_opts: &ShadowOptions,
        current_peers: &SimplePeerCollection,
        next_peers: &SimplePeerCollection,
        next_dkg_after_restore: DkgAfterRestore,
    ) -> Self {
        Self {
            tss_state,
            dkg_type,
            epoch,
            threshold,
            shadow_key_opts: shadow_key_opts.clone(),
            current_peers: current_peers.clone(),
            next_peers: next_peers.clone(),
            dkgs: BTreeMap::new(),
            next_dkg_after_restore,
        }
    }

    /// Add a DKG to be computed
    pub fn add_dkg(
        &mut self,
        dkg_id: &str,
        key_set_id: &str,
        curve_type: CurveType,
        pubkey: Option<String>,
    ) {
        let dkg_data = DkgData {
            dkg_id: dkg_id.to_string(),
            curve_type,
            key_set_id: key_set_id.to_string(),
            pubkey: pubkey.clone(),
            result: None,
        };
        self.dkgs
            .entry(dkg_id.to_string())
            .and_modify(|d| *d = dkg_data.clone())
            .or_insert(dkg_data);
    }

    /// Get the DKG data
    pub fn get_dkgs(&self) -> Values<'_, String, DkgData> {
        self.dkgs.values()
    }

    /// Execute the all DKGs
    #[instrument(level = "debug", skip_all)]
    pub async fn execute(&mut self, root_dkg_id: &str, realm_id: u64) -> Result<Option<Mode>> {
        if self.next_peers.is_empty() {
            return Err(unexpected_err(
                "No new peers to launch DKG with!".to_string(),
                None,
            ));
        }
        if !self
            .next_peers
            .contains_address(&self.tss_state.peer_state.addr)
        {
            info!("Node is not in the new peer set - skipping DKG.");
            return Ok(None);
        }
        info!(
            "Doing Epoch change for realm_id {} {} with {} DKGs.  Going from  {} to {}, after_restore: {:?}",
            realm_id,
            self.dkg_type,
            self.dkgs.len(),
            self.current_peers.debug_addresses(),
            self.next_peers.debug_addresses(),
            self.next_dkg_after_restore,
        );

        let self_peer = self.next_peers.peer_at_address(&self.tss_state.addr)?;
        let self_ordinal = self
            .next_peers
            .0
            .iter()
            .position(|p| p.socket_address == self.tss_state.addr)
            .expect_or_err("ordinal not found")?;
        let usize_limit = self.next_peers.0.len();
        let threshold =
            NonZeroUsize::new(self.threshold).expect_or_err("Empty non-zero usize threshold")?;
        let old_threshold = self.tss_state.get_threshold().await;
        let limit = NonZeroUsize::new(usize_limit).expect_or_err("Empty non-zero usize limit")?;
        let old_ids = self.current_peers.peer_ids();
        let next_ids = self.next_peers.peer_ids();
        let old_peer_id = match self.current_peers.peer_at_address(&self.tss_state.addr) {
            Ok(peer) => peer.peer_id,
            Err(_) => PeerId::NOT_ASSIGNED,
        };

        // Need a reshare and not refresh when the DKG takes place after a restore
        // because the KeyShareCommitments will not exist yet, and a refresh won't be valid
        // for the key shares, resulting in invalid key share proofs
        let start = std::time::Instant::now();
        let mode = if self.next_dkg_after_restore.value() {
            Mode::ExistingPeer
        } else if self.current_peers.is_empty() {
            Mode::Initial
        } else if self.current_peers == self.next_peers {
            Mode::RefreshPeer
        } else if old_peer_id != PeerId::NOT_ASSIGNED {
            Mode::ExistingPeer
        } else {
            Mode::NewPeer
        };

        debug!(
            "Realm {} Node {} | Peer ID {:?} -> {:?} Threshold : {} -> {} | Mode: {}, DKG IDs: [{}]",
            realm_id,
            self_peer.socket_address,
            old_peer_id,
            self_peer.peer_id,
            if self.current_peers.is_empty() {
                0
            } else {
                old_threshold
            },
            self.threshold,
            mode,
            self.dkgs.keys().cloned().collect::<Vec<_>>().join(", ")
        );

        metrics::counter::add_one(metrics::dkg::DkgMetrics::DkgInit, &[]);
        let mut valid_dkgs = vec![true; self.dkgs.len()];
        let mut dkg_participants = Vec::with_capacity(self.dkgs.len());
        let mut dkg_id_to_index = HashMap::with_capacity(self.dkgs.len());
        let mut index_to_dkg_id = Vec::with_capacity(self.dkgs.len());
        let staker_address = &self.tss_state.peer_state.hex_staker_address();

        let t = DkgData::default();
        let mut create_participant_args = CreateParticipantArgs {
            mode,
            peer_id: self_peer.peer_id,
            old_ids: &old_ids,
            next_ids: &next_ids,
            dkg_data: &t,
            threshold,
            limit,
            realm_id,
        };

        for (i, (dkg_id, dkg_data)) in self.dkgs.iter().enumerate() {
            dkg_id_to_index.insert(dkg_id.clone(), i);
            index_to_dkg_id.push(dkg_id.clone());
            create_participant_args.dkg_data = dkg_data;
            match dkg_data.curve_type {
                CurveType::BLS => {
                    let participant = self
                        .create_participant::<blsful::inner_types::G1Projective>(
                            &create_participant_args,
                            None,
                        )
                        .await?;
                    dkg_participants.push(DkgCurve::Bls(participant));
                }
                CurveType::K256 => {
                    let participant = self
                        .create_participant::<k256::ProjectivePoint>(&create_participant_args, None)
                        .await?;
                    dkg_participants.push(DkgCurve::K256(participant));
                }
                CurveType::P256 => {
                    let participant = self
                        .create_participant::<p256::ProjectivePoint>(&create_participant_args, None)
                        .await?;
                    dkg_participants.push(DkgCurve::P256(participant));
                }
                CurveType::P384 => {
                    let participant = self
                        .create_participant::<p384::ProjectivePoint>(&create_participant_args, None)
                        .await?;
                    dkg_participants.push(DkgCurve::P384(participant));
                }
                CurveType::Ed25519 => {
                    let participant = self
                        .create_participant::<vsss_rs::curve25519::WrappedEdwards>(
                            &create_participant_args,
                            None,
                        )
                        .await?;
                    dkg_participants.push(DkgCurve::Ed25519(participant));
                }
                CurveType::Ristretto25519 => {
                    let participant = self
                        .create_participant::<vsss_rs::curve25519::WrappedRistretto>(
                            &create_participant_args,
                            None,
                        )
                        .await?;
                    dkg_participants.push(DkgCurve::Ristretto25519(participant));
                }
                CurveType::Ed448 => {
                    let participant = self
                        .create_participant::<ed448_goldilocks::EdwardsPoint>(
                            &create_participant_args,
                            None,
                        )
                        .await?;
                    dkg_participants.push(DkgCurve::Ed448(participant));
                }
                CurveType::RedJubjub => {
                    let participant = self
                        .create_participant::<jubjub::SubgroupPoint>(
                            &create_participant_args,
                            Some(lit_frost::red_jubjub_generator()),
                        )
                        .await?;
                    dkg_participants.push(DkgCurve::JubJub(participant));
                }
                CurveType::RedDecaf377 => {
                    let participant = self
                        .create_participant::<decaf377::Element>(&create_participant_args, None)
                        .await?;
                    dkg_participants.push(DkgCurve::Decaf377(participant));
                }
                CurveType::BLS12381G1 => {
                    let participant = self
                        .create_participant::<blsful::inner_types::G1Projective>(
                            &create_participant_args,
                            None,
                        )
                        .await?;
                    dkg_participants.push(DkgCurve::Bls12381G1ProofOfPossession(participant));
                }
            }
        }

        let txn_prefix = format!("{}.{}_keyset", root_dkg_id, self.dkg_type);
        trace!(
            "Node {} - Using DKG txn prefix: {}, realm_id: {}",
            self_peer.peer_id, txn_prefix, realm_id
        );
        let mut send_participant_data = Vec::with_capacity(self.next_peers.0.len());
        for _ in 0..self.next_peers.0.len() {
            send_participant_data.push(Vec::with_capacity(self.next_peers.0.len()));
        }
        loop {
            if valid_dkgs.iter().all(|b| !b) {
                return Err(unexpected_err(
                    "All participants are in an invalid state",
                    None,
                ));
            }
            for d in send_participant_data.iter_mut() {
                d.clear();
            }
            let mut round = Round::One.to_string();
            let mut count = 0;
            for (i, dkg_data) in dkg_participants.iter_mut().enumerate() {
                if !valid_dkgs[i] {
                    continue;
                }
                round = dkg_data.round().to_string();
                let output_generator = match dkg_data.run() {
                    Ok(generator) => generator,
                    Err(e) => {
                        error!("Dkg round failed: {:?}, realm_id: {}", e, realm_id);
                        break;
                    }
                };
                for output in output_generator.iter() {
                    let added_inner_data = DkgMessage {
                        dkg_id: index_to_dkg_id[i].clone(),
                        output: output.clone(),
                    };
                    let entry = send_participant_data.get_mut(output.dst_ordinal()).ok_or(
                        unexpected_err(
                            format!("Missing participant at index: {}", output.dst_ordinal()),
                            None,
                        ),
                    )?;
                    entry.push(added_inner_data);
                }
                if send_participant_data.iter().all(|v| v.is_empty()) {
                    continue;
                }
                count += 1;
            }
            if count > 0 {
                debug!(
                    "Node {} - Sending DKG messages for round: {} , prefix: {}",
                    self_peer.peer_id, round, txn_prefix
                );
                let cm = CommsManager::new_with_peers(
                    &self.tss_state,
                    &txn_prefix,
                    &self.next_peers,
                    &round,
                )
                .await?;

                let mut sent_any = false;
                for (dest_peer, data) in self.next_peers.0.iter().zip(send_participant_data.iter())
                {
                    if data.is_empty() {
                        continue;
                    }
                    if let Err(e) = cm.send_direct(dest_peer, data).await {
                        error!("Error while sending DKG data: {:?}", e);
                        return Err(e);
                    }
                    sent_any = true;
                }

                if sent_any {
                    trace!(
                        "Node {} - Receiving DKG messages for round: {}, txn_prefix: {}",
                        self_peer.peer_id, round, txn_prefix
                    );
                    let received = cm.collect::<Vec<DkgMessage>>().await?;
                    debug!(
                        "Node {} - {} DKG messages received for round: {}, realm_id: {}",
                        self_peer.peer_id,
                        received.len(),
                        round,
                        realm_id
                    );

                    for (_, messages) in received {
                        for d in &messages {
                            let participant_index = *dkg_id_to_index
                                .get(&d.dkg_id)
                                .expect_or_err("Invalid dkg id")?;
                            let participant: &mut DkgCurve = dkg_participants
                                .get_mut(participant_index)
                                .expect_or_err("dkg participant not found")?;
                            if !valid_dkgs[participant_index] {
                                error!(
                                    "Participant is not in a valid state. dkg_id: {}, curve_type: {}, key_index: {}, realm_id: {}",
                                    self.dkgs[&d.dkg_id].dkg_id,
                                    self.dkgs[&d.dkg_id].curve_type,
                                    participant_index,
                                    realm_id
                                );
                                continue;
                            }
                            if let Err(e) = participant.receive(&d.output) {
                                error!(
                                    "Error while receiving participant data: {:?}, realm_id: {}",
                                    e, realm_id
                                );
                                valid_dkgs[participant_index] = false;
                            }
                        }
                    }
                }
            }

            let finished = dkg_participants.iter().all(|p| p.completed());

            if finished {
                debug!(
                    "Node: {}, txn_prefix: {} - Saving key shares to disk",
                    self_peer.peer_id, txn_prefix
                );
                let dkg_ids = self.dkgs.keys().cloned().collect::<Vec<_>>();
                let mut args = CreateDkgResultArgs {
                    mode,
                    dkg_id: "",
                    peer_id: self_peer.peer_id,
                    pub_key: None,
                    curve_type: CurveType::BLS,
                    realm_id,
                };
                for dkg_id in &dkg_ids {
                    let (curve_type, pubkey) = {
                        let dkg = self.dkgs.get(dkg_id).expect_or_err("Invalid dkg id")?;
                        (dkg.curve_type, dkg.pubkey.clone())
                    };
                    let participant_index = *dkg_id_to_index
                        .get(dkg_id)
                        .expect_or_err("Invalid dkg id")?;
                    let participant: &mut DkgCurve = dkg_participants
                        .get_mut(participant_index)
                        .expect_or_err("dkg participant not found")?;

                    args.dkg_id = dkg_id;
                    args.curve_type = curve_type;
                    args.pub_key = pubkey;

                    let result = match participant {
                        DkgCurve::Bls(p) => DkgResult::Bls(
                            self.create_dkg_result::<blsful::inner_types::G1Projective>(
                                &args,
                                p.as_ref(),
                            )
                            .await?,
                        ),
                        DkgCurve::K256(p) => DkgResult::K256(
                            self.create_dkg_result::<k256::ProjectivePoint>(&args, p.as_ref())
                                .await?,
                        ),
                        DkgCurve::P256(p) => DkgResult::P256(
                            self.create_dkg_result::<p256::ProjectivePoint>(&args, p.as_ref())
                                .await?,
                        ),
                        DkgCurve::P384(p) => DkgResult::P384(
                            self.create_dkg_result::<p384::ProjectivePoint>(&args, p.as_ref())
                                .await?,
                        ),
                        DkgCurve::Ed25519(p) => DkgResult::Ed25519(
                            self.create_dkg_result::<vsss_rs::curve25519::WrappedEdwards>(
                                &args,
                                p.as_ref(),
                            )
                            .await?,
                        ),
                        DkgCurve::Ristretto25519(p) => DkgResult::Ristretto256(
                            self.create_dkg_result::<vsss_rs::curve25519::WrappedRistretto>(
                                &args,
                                p.as_ref(),
                            )
                            .await?,
                        ),
                        DkgCurve::Ed448(p) => DkgResult::Ed448(
                            self.create_dkg_result::<ed448_goldilocks::EdwardsPoint>(
                                &args,
                                p.as_ref(),
                            )
                            .await?,
                        ),
                        DkgCurve::JubJub(p) => DkgResult::JubJub(
                            self.create_dkg_result::<jubjub::SubgroupPoint>(&args, p.as_ref())
                                .await?,
                        ),
                        DkgCurve::Decaf377(p) => DkgResult::Decaf377(
                            self.create_dkg_result::<decaf377::Element>(&args, p.as_ref())
                                .await?,
                        ),
                        DkgCurve::Bls12381G1ProofOfPossession(p) => {
                            DkgResult::Bls12381G1ProofOfPossession(
                                self.create_dkg_result::<blsful::inner_types::G1Projective>(
                                    &args,
                                    p.as_ref(),
                                )
                                .await?,
                            )
                        }
                    };

                    self.dkgs
                        .get_mut(dkg_id)
                        .expect_or_err("Invalid dkg id")?
                        .result = Some(result);
                }
                break;
            }
        }
        self.tss_state.set_threshold(self.threshold);
        metrics::counter::add_one(metrics::dkg::DkgMetrics::DkgComplete, &[]);

        Ok(Some(mode))
    }

    #[instrument(level = "debug", skip_all)]
    async fn create_dkg_result<G>(
        &self,
        args: &CreateDkgResultArgs<'_>,
        participant: &dyn AnyParticipant<G>,
    ) -> Result<DkgOutput<G>>
    where
        G: GroupEncoding + SumOfProducts + Default + CompressedBytes,
        G::Scalar: ScalarHash + From<PeerId> + CompressedBytes,
    {
        debug!(
            "Creating DKG result for peer: {:?} in realm: {}",
            args.peer_id, args.realm_id
        );
        let staker_address = &self.tss_state.peer_state.hex_staker_address();
        let mut pk = participant
            .get_public_key()
            .expect_or_err("Unable to get public key")?;
        let mut share = participant
            .get_secret_share()
            .expect_or_err("Unable to get secret share")?
            .value
            .0;
        // The key share commitments are all the feldman verifiers from all participants.
        let feldman_commitments = participant
            .get_received_round1_data()
            .values()
            .map(|v| v.feldman_commitments().iter().map(|g| g.0).collect())
            .collect::<Vec<Vec<G>>>();
        let mut key_share_commitments = feldman_commitments[0].clone();
        for commitments in feldman_commitments.iter().skip(1) {
            for (c, o) in key_share_commitments.iter_mut().zip(commitments.iter()) {
                *c += o;
            }
        }
        let key_state = KeyPersistence::<G>::new(args.curve_type);
        let next_epoch = match self.dkg_type {
            DkgType::Standard => self.epoch + 1,
            DkgType::RecoveryParty => RECOVERY_DKG_EPOCH,
        };

        let active_peers = self.next_peers.active_peers();
        let pubkey = args.pub_key.clone();
        let (write_key_pubkey, save_commitments, delete_epoch) = match args.mode {
            Mode::Initial | Mode::NewPeer => {
                // No checks needed, just save the result
                debug!("Saving new key share");
                let save_commitments = KeyShareCommitments {
                    dkg_id: args.dkg_id.to_string(),
                    commitments: key_share_commitments.clone(),
                };
                (None, save_commitments, 0)
            }
            Mode::RefreshPeer => {
                if !bool::from(pk.is_identity()) {
                    return Err(unexpected_err(
                        "Public key is not identity when a refresh occurred".to_string(),
                        None,
                    ));
                }
                // Refresh the existing share
                share = participant.get_original_secret() + share;
                let pubkey = pubkey.expect_or_err("Unable to get public key")?;

                let mut saved_commitments =
                    read_key_share_commitments_from_disk::<KeyShareCommitments<G>>(
                        args.curve_type,
                        &pubkey,
                        staker_address,
                        &args.peer_id,
                        self.epoch,
                        args.realm_id,
                        &self.tss_state.key_cache,
                    )
                    .await?;
                for (c, o) in saved_commitments
                    .commitments
                    .iter_mut()
                    .zip(key_share_commitments.iter())
                {
                    *c += o;
                }
                debug!("Saving refreshed key share");
                pk = key_state.pk_from_hex(&pubkey)?;
                (
                    Some(pubkey),
                    saved_commitments,
                    self.epoch.saturating_sub(2),
                )
            }
            Mode::ExistingPeer => {
                let pubkey = pubkey.expect_or_err("Unable to get public key")?;
                let old_pk = key_state.pk_from_hex(&pubkey)?;
                if pk != old_pk {
                    return Err(unexpected_err(
                        format!(
                            "Public key mismatch. Expected: {}, computed: {}",
                            pubkey,
                            pk.to_compressed_hex()
                        ),
                        None,
                    ));
                }
                let save_commitments = KeyShareCommitments {
                    dkg_id: args.dkg_id.to_string(),
                    commitments: key_share_commitments.clone(),
                };
                debug!("Saving reshared key share");
                (Some(pubkey), save_commitments, self.epoch.saturating_sub(2))
            }
        };

        let pubkey = key_state
            .write_key(
                write_key_pubkey,
                pk,
                share,
                &args.peer_id,
                args.dkg_id,
                next_epoch,
                &active_peers,
                staker_address,
                args.realm_id,
                self.threshold,
                &self.tss_state.key_cache,
            )
            .await?;
        debug!(
            "Saved key share to disk for public key {}, epoch {}, realm {}",
            pubkey, next_epoch, args.realm_id
        );
        write_key_share_commitments_to_disk(
            args.curve_type,
            &pubkey,
            staker_address,
            &args.peer_id,
            next_epoch,
            args.realm_id,
            &self.tss_state.key_cache,
            &save_commitments,
        )
        .await?;

        if delete_epoch > MIN_EPOCH_FOR_COMMITMENT_DELETION {
            let shadow_realm_id = DataVersionReader::read_field_unchecked(
                &self.tss_state.chain_data_config_manager.shadow_realm_id,
                |realm| realm.as_u64(),
            );

            if shadow_realm_id > 0 {
                debug!(
                    "Holding on to key shares and commitments while processing shadow realm {} / epoch {}",
                    args.realm_id, delete_epoch
                );
            } else {
                debug!(
                    "Removing old key share commitments for epochs less than {}",
                    delete_epoch
                );
                let _ = delete_key_share_commitments_older_than_epoch(
                    args.curve_type,
                    &pubkey,
                    staker_address,
                    &args.peer_id,
                    delete_epoch,
                    args.realm_id,
                    &self.tss_state.key_cache,
                )
                .await;
                debug!(
                    "Removing old key shares for epochs less than {}",
                    delete_epoch
                );
                let _ = delete_keyshares_older_than_epoch(
                    args.curve_type,
                    &pubkey,
                    staker_address,
                    &args.peer_id,
                    delete_epoch,
                    args.realm_id,
                    &self.tss_state.key_cache,
                )
                .await;
            }
        }

        Ok(DkgOutput {
            pk,
            share,
            key_share_commitments,
            peer_id: args.peer_id,
        })
    }

    #[instrument(level = "debug", skip_all)]
    #[allow(clippy::too_many_arguments)]
    async fn create_participant<G>(
        &self,
        args: &CreateParticipantArgs<'_>,
        generator: Option<G>,
    ) -> Result<Box<dyn AnyParticipant<G>>>
    where
        G: GroupEncoding + SumOfProducts + Default + CompressedBytes,
        G::Scalar: ScalarHash + From<PeerId> + CompressedBytes,
    {
        let id = IdentifierPrimeField(G::Scalar::from(args.peer_id));
        let old_ids = args.old_ids.iter().collect::<HashSet<_>>();
        let new_ids_set = args.next_ids.iter().collect::<HashSet<_>>();
        let realm_id = args.realm_id;
        let old_ids = old_ids
            .intersection(&new_ids_set)
            .map(|id| IdentifierPrimeField(G::Scalar::from(**id)))
            .collect::<Vec<_>>();

        let new_id_scalars = args
            .next_ids
            .iter()
            .map(|id| IdentifierPrimeField(G::Scalar::from(*id)))
            .collect::<Vec<_>>();

        trace!(
            "Creating participant in realm {} dkg_id {} curve {} scalar id {} with other scalar ids: {}",
            args.realm_id,
            args.dkg_data.dkg_id,
            args.dkg_data.curve_type,
            format!(
                "{}...",
                id.0.to_compressed_hex().chars().take(6).collect::<String>()
            ),
            new_id_scalars
                .iter()
                .map(|x| format!(
                    "{}...",
                    x.0.to_compressed_hex().chars().take(6).collect::<String>()
                ))
                .collect::<Vec<_>>()
                .join(", "),
        );

        let dummy_key_cache = KeyCache::default();
        let seq = vec![ParticipantIdGeneratorType::list(new_id_scalars.as_slice())];
        let parameters = Parameters::<G>::new(args.threshold, args.limit, generator, Some(seq));
        let key_state = KeyPersistence::<G>::new(args.dkg_data.curve_type);
        match args.mode {
            Mode::Initial => Ok(Box::new(
                SecretParticipant::<G>::new_secret(id, &parameters).map_err(|e| {
                    unexpected_err(e, Some("Unable to create new dkg participant".to_string()))
                })?,
            )),
            Mode::NewPeer => Ok(Box::new(
                RefreshParticipant::<G>::new_refresh(id, None, &parameters).map_err(|e| {
                    unexpected_err(e, Some("Unable to create new dkg participant".to_string()))
                })?,
            )),
            Mode::RefreshPeer => {
                let staker_address = &self.tss_state.peer_state.hex_staker_address();
                let pubkey = args
                    .dkg_data
                    .pubkey
                    .as_ref()
                    .expect_or_err("Unable to get public key")?;

                let (private_share, _) = match key_state
                    .read_key(
                        pubkey,
                        &args.peer_id,
                        self.epoch,
                        staker_address,
                        realm_id,
                        &dummy_key_cache,
                    )
                    .await
                {
                    Ok(Some((private_share, public_key))) => (private_share, public_key),
                    Ok(None) => {
                        let err_msg = format!(
                            "key share not found on disk for realm {} public key {}",
                            realm_id, pubkey
                        );
                        error!("{}", err_msg);
                        return Err(unexpected_err(err_msg, None));
                    }
                    Err(e) => {
                        let err_msg = format!(
                            "Error reading key share for realm {} public key {}",
                            realm_id, pubkey
                        );
                        error!("{}", err_msg);
                        return Err(unexpected_err(e, Some(err_msg)));
                    }
                };
                Ok(Box::new(
                    RefreshParticipant::<G>::new_refresh(id, Some(private_share), &parameters)
                        .map_err(|e| {
                            unexpected_err(
                                e,
                                Some("Unable to create new dkg participant".to_string()),
                            )
                        })?,
                ))
            }
            Mode::ExistingPeer => {
                let staker_address = &self.tss_state.peer_state.hex_staker_address();
                let pubkey = args
                    .dkg_data
                    .pubkey
                    .as_ref()
                    .expect_or_err("Unable to get public key")?;

                let key_cache = match &self.next_dkg_after_restore {
                    DkgAfterRestore::True(data) => &data.key_cache,
                    DkgAfterRestore::False => &dummy_key_cache,
                };

                // in the initial epoch when we're shadow splicing we actually use the same key share as regular DKG...
                // this is specific to the first nodes in the realm, and only for the first epoch.
                let (read_epoch, read_realm_id) = if self.shadow_key_opts.is_shadow
                    && self.shadow_key_opts.epoch_number > 1
                {
                    trace!("Using shadow key opts to read key share from disk.");
                    (
                        self.shadow_key_opts.epoch_number,
                        self.shadow_key_opts.realm_id,
                    )
                } else if self.shadow_key_opts.is_shadow {
                    trace!(
                        "Using normal key opts to read key share from disk while in shadow realm."
                    );
                    (
                        self.shadow_key_opts.non_shadow_epoch_number,
                        self.shadow_key_opts.non_shadow_realm_id,
                    )
                } else {
                    trace!("Using normal key opts to read key share from disk.");

                    (self.epoch, realm_id)
                };

                let key_share = match read_key_share_from_disk::<KeyShare>(
                    key_state.curve_type,
                    pubkey,
                    staker_address,
                    &args.peer_id,
                    read_epoch,
                    read_realm_id,
                    key_cache,
                )
                .await
                {
                    Ok(share) => share,
                    Err(e) => {
                        let err_msg = format!(
                            "Error reading key share in realm {}, epoch {}, for public key {}",
                            read_realm_id, read_epoch, pubkey
                        );
                        error!("{}", err_msg);
                        error!("Shadow key opts: {:?}", self.shadow_key_opts);
                        return Err(unexpected_err(e, Some(err_msg)));
                    }
                };

                if tracing::enabled!(tracing::Level::TRACE) {
                    let all_peers = self
                        .tss_state
                        .peer_state
                        .peers_in_next_epoch_current_union_including_shadow();

                    trace!(
                        "Current Peer  - {:?},  current peers {:?} - Key reshare peer: {:?},  next peers: {:?}",
                        all_peers
                            .peer_by_id(&args.peer_id)
                            .unwrap_or(SimplePeer::unknown_peer())
                            .debug_address(),
                        all_peers.peers_by_id(args.next_ids).debug_addresses(),
                        all_peers
                            .peer_by_id(&key_share.peer_id)
                            .unwrap_or(SimplePeer::unknown_peer())
                            .debug_address(),
                        all_peers.peers_by_id(&key_share.peers).debug_addresses()
                    );
                };
                let private_share = key_state.secret_from_hex(&key_share.hex_private_share)?;
                let old_share = DefaultShare {
                    identifier: IdentifierPrimeField(G::Scalar::from(key_share.peer_id)),
                    value: IdentifierPrimeField(private_share),
                };
                // The set of peer ids used to populate `old_ids` should exactly match the
                // peer ids used to create the `old_share` instances above. If a private
                // share is no longer used, the corresponding peer id should be dropped as well.
                let old_ids = match &self.next_dkg_after_restore {
                    DkgAfterRestore::True(data) => {
                        let mut old_ids = vec![];
                        for pair in data.peers.iter() {
                            let new_peer_id = PeerId::try_from(pair.new_peer_id)
                                .map_err(|e| unexpected_err(e, None))?;
                            let old_peer_id = PeerId::try_from(pair.old_peer_id)
                                .map_err(|e| unexpected_err(e, None))?;
                            if args.next_ids.contains(&new_peer_id) {
                                old_ids.push(IdentifierPrimeField(G::Scalar::from(old_peer_id)));
                            }
                        }
                        old_ids
                    }
                    DkgAfterRestore::False => {
                        let old_ids = key_share
                            .peers
                            .iter()
                            .map(|id| IdentifierPrimeField(G::Scalar::from(*id)))
                            .collect::<HashSet<_>>();
                        let new_ids_set = new_id_scalars.iter().copied().collect::<HashSet<_>>();
                        old_ids
                            .intersection(&new_ids_set)
                            .copied()
                            .collect::<Vec<_>>()
                    }
                };
                Ok(Box::new(
                    SecretParticipant::<G>::with_secret(id, &old_share, &parameters, &old_ids)
                        .map_err(|e| {
                            unexpected_err(
                                e,
                                Some("Unable to create new dkg participant".to_string()),
                            )
                        })?,
                ))
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DkgData {
    pub(crate) dkg_id: String,
    pub(crate) key_set_id: String,
    pub(crate) curve_type: CurveType,
    pub(crate) pubkey: Option<String>,
    pub(crate) result: Option<DkgResult>,
}

impl DkgData {
    pub fn dkg_id(&self) -> &str {
        &self.dkg_id
    }

    pub fn key_set_id(&self) -> &str {
        &self.key_set_id
    }

    pub fn curve_type(&self) -> CurveType {
        self.curve_type
    }

    pub fn pubkey(&self) -> Option<&String> {
        self.pubkey.as_ref()
    }

    pub fn result(&self) -> Option<&DkgResult> {
        self.result.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DkgScalar {
    Bls(blsful::inner_types::Scalar),
    K256(k256::Scalar),
    P256(p256::Scalar),
    P384(p384::Scalar),
    Ed25519(vsss_rs::curve25519::WrappedScalar),
    Ristretto25519(vsss_rs::curve25519::WrappedScalar),
    Ed448(ed448_goldilocks::Scalar),
    JubJub(jubjub::Scalar),
    Decaf377(decaf377::Fr),
    Bls12381G1ProofOfPossession(blsful::inner_types::Scalar),
}

impl std::fmt::Display for DkgScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex = match self {
            Self::Bls(scalar) => scalar.to_compressed_hex(),
            Self::K256(scalar) => scalar.to_compressed_hex(),
            Self::P256(scalar) => scalar.to_compressed_hex(),
            Self::P384(scalar) => scalar.to_compressed_hex(),
            Self::Ed25519(scalar) => scalar.to_compressed_hex(),
            Self::Ristretto25519(scalar) => scalar.to_compressed_hex(),
            Self::Ed448(scalar) => scalar.to_compressed_hex(),
            Self::JubJub(scalar) => scalar.to_compressed_hex(),
            Self::Decaf377(scalar) => scalar.to_compressed_hex(),
            Self::Bls12381G1ProofOfPossession(scalar) => scalar.to_compressed_hex(),
        };
        write!(f, "{}", hex)
    }
}

#[derive(Clone, Debug)]
pub enum DkgResult {
    Bls(DkgOutput<blsful::inner_types::G1Projective>),
    K256(DkgOutput<k256::ProjectivePoint>),
    P256(DkgOutput<p256::ProjectivePoint>),
    P384(DkgOutput<p384::ProjectivePoint>),
    Ed25519(DkgOutput<vsss_rs::curve25519::WrappedEdwards>),
    Ristretto256(DkgOutput<vsss_rs::curve25519::WrappedRistretto>),
    Ed448(DkgOutput<ed448_goldilocks::EdwardsPoint>),
    JubJub(DkgOutput<jubjub::SubgroupPoint>),
    Decaf377(DkgOutput<decaf377::Element>),
    Bls12381G1ProofOfPossession(DkgOutput<blsful::inner_types::G1Projective>),
}

impl DkgResult {
    pub fn public_key(&self) -> String {
        match self {
            Self::Bls(output) => {
                let helper =
                    KeyPersistence::<blsful::inner_types::G1Projective>::new(CurveType::BLS);
                helper.pk_to_hex(&output.pk)
            }
            Self::K256(output) => {
                let helper = KeyPersistence::<k256::ProjectivePoint>::new(CurveType::K256);
                helper.pk_to_hex(&output.pk)
            }
            Self::P256(output) => {
                let helper = KeyPersistence::<p256::ProjectivePoint>::new(CurveType::P256);
                helper.pk_to_hex(&output.pk)
            }
            Self::P384(output) => {
                let helper = KeyPersistence::<p384::ProjectivePoint>::new(CurveType::P384);
                helper.pk_to_hex(&output.pk)
            }
            Self::Ed25519(output) => {
                let helper =
                    KeyPersistence::<vsss_rs::curve25519::WrappedEdwards>::new(CurveType::Ed25519);
                helper.pk_to_hex(&output.pk)
            }
            Self::Ristretto256(output) => {
                let helper = KeyPersistence::<vsss_rs::curve25519::WrappedRistretto>::new(
                    CurveType::Ristretto25519,
                );
                helper.pk_to_hex(&output.pk)
            }
            Self::Ed448(output) => {
                let helper =
                    KeyPersistence::<ed448_goldilocks::EdwardsPoint>::new(CurveType::Ed448);
                helper.pk_to_hex(&output.pk)
            }
            Self::JubJub(output) => {
                let helper = KeyPersistence::<jubjub::SubgroupPoint>::new(CurveType::RedJubjub);
                helper.pk_to_hex(&output.pk)
            }
            Self::Decaf377(output) => {
                let helper = KeyPersistence::<decaf377::Element>::new(CurveType::RedDecaf377);
                helper.pk_to_hex(&output.pk)
            }
            Self::Bls12381G1ProofOfPossession(output) => {
                let helper =
                    KeyPersistence::<blsful::inner_types::G1Projective>::new(CurveType::BLS12381G1);
                helper.pk_to_hex(&output.pk)
            }
        }
    }

    pub fn dkg_root_key(&self) -> CachedRootKey {
        match self {
            Self::Bls(output) => {
                let helper =
                    KeyPersistence::<blsful::inner_types::G1Projective>::new(CurveType::BLS);
                CachedRootKey {
                    curve_type: CurveType::BLS,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::K256(output) => {
                let helper = KeyPersistence::<k256::ProjectivePoint>::new(CurveType::K256);
                CachedRootKey {
                    curve_type: CurveType::K256,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::P256(output) => {
                let helper = KeyPersistence::<p256::ProjectivePoint>::new(CurveType::P256);
                CachedRootKey {
                    curve_type: CurveType::P256,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::P384(output) => {
                let helper = KeyPersistence::<p384::ProjectivePoint>::new(CurveType::P384);
                CachedRootKey {
                    curve_type: CurveType::P384,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::Ed25519(output) => {
                let helper =
                    KeyPersistence::<vsss_rs::curve25519::WrappedEdwards>::new(CurveType::Ed25519);
                CachedRootKey {
                    curve_type: CurveType::Ed25519,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::Ristretto256(output) => {
                let helper = KeyPersistence::<vsss_rs::curve25519::WrappedRistretto>::new(
                    CurveType::Ristretto25519,
                );
                CachedRootKey {
                    curve_type: CurveType::Ristretto25519,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::Ed448(output) => {
                let helper =
                    KeyPersistence::<ed448_goldilocks::EdwardsPoint>::new(CurveType::Ed448);
                CachedRootKey {
                    curve_type: CurveType::Ed448,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::JubJub(output) => {
                let helper = KeyPersistence::<jubjub::SubgroupPoint>::new(CurveType::RedJubjub);
                CachedRootKey {
                    curve_type: CurveType::RedJubjub,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::Decaf377(output) => {
                let helper = KeyPersistence::<decaf377::Element>::new(CurveType::RedDecaf377);
                CachedRootKey {
                    curve_type: CurveType::RedDecaf377,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
            Self::Bls12381G1ProofOfPossession(output) => {
                let helper =
                    KeyPersistence::<blsful::inner_types::G1Projective>::new(CurveType::BLS12381G1);
                CachedRootKey {
                    curve_type: CurveType::BLS12381G1,
                    public_key: helper.pk_to_hex(&output.pk),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum DkgCurve {
    Bls(Box<dyn AnyParticipant<blsful::inner_types::G1Projective>>),
    K256(Box<dyn AnyParticipant<k256::ProjectivePoint>>),
    P256(Box<dyn AnyParticipant<p256::ProjectivePoint>>),
    P384(Box<dyn AnyParticipant<p384::ProjectivePoint>>),
    Ed25519(Box<dyn AnyParticipant<vsss_rs::curve25519::WrappedEdwards>>),
    Ristretto25519(Box<dyn AnyParticipant<vsss_rs::curve25519::WrappedRistretto>>),
    Ed448(Box<dyn AnyParticipant<ed448_goldilocks::EdwardsPoint>>),
    JubJub(Box<dyn AnyParticipant<jubjub::SubgroupPoint>>),
    Decaf377(Box<dyn AnyParticipant<decaf377::Element>>),
    Bls12381G1ProofOfPossession(Box<dyn AnyParticipant<blsful::inner_types::G1Projective>>),
}

impl DkgCurve {
    pub fn round(&self) -> Round {
        match self {
            Self::Bls(participant) => participant.get_round(),
            Self::K256(participant) => participant.get_round(),
            Self::P256(participant) => participant.get_round(),
            Self::P384(participant) => participant.get_round(),
            Self::Ed25519(participant) => participant.get_round(),
            Self::Ristretto25519(participant) => participant.get_round(),
            Self::Ed448(participant) => participant.get_round(),
            Self::JubJub(participant) => participant.get_round(),
            Self::Decaf377(participant) => participant.get_round(),
            Self::Bls12381G1ProofOfPossession(participant) => participant.get_round(),
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub fn run(&mut self) -> Result<DkgRoundOutputGenerator> {
        match self {
            DkgCurve::Bls(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::Bls(output))
            }
            DkgCurve::K256(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::K256(output))
            }
            DkgCurve::P256(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::P256(output))
            }
            DkgCurve::P384(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::P384(output))
            }
            DkgCurve::Ed25519(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::Ed25519(output))
            }
            DkgCurve::Ristretto25519(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::Ristretto25519(output))
            }
            DkgCurve::Ed448(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::Ed448(output))
            }
            DkgCurve::JubJub(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::JubJub(output))
            }
            DkgCurve::Decaf377(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::Decaf377(output))
            }
            DkgCurve::Bls12381G1ProofOfPossession(participant) => {
                let output = participant.run().map_err(|e| {
                    unexpected_err(
                        e,
                        Some("an error occurred while computing next round".to_string()),
                    )
                })?;
                Ok(DkgRoundOutputGenerator::Bls12381G1ProofOfPossession(output))
            }
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub fn completed(&self) -> bool {
        match self {
            DkgCurve::Bls(participant) => participant.completed(),
            DkgCurve::K256(participant) => participant.completed(),
            DkgCurve::P256(participant) => participant.completed(),
            DkgCurve::P384(participant) => participant.completed(),
            DkgCurve::Ed25519(participant) => participant.completed(),
            DkgCurve::Ristretto25519(participant) => participant.completed(),
            DkgCurve::Ed448(participant) => participant.completed(),
            DkgCurve::JubJub(participant) => participant.completed(),
            DkgCurve::Decaf377(participant) => participant.completed(),
            DkgCurve::Bls12381G1ProofOfPossession(participant) => participant.completed(),
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub fn receive(&mut self, participant_data: &DkgParticipantRoundOutput) -> Result<()> {
        match (self, participant_data) {
            (DkgCurve::Bls(participant), DkgParticipantRoundOutput::Bls(participant_data)) => {
                participant.receive(&participant_data.data).map_err(|e| {
                    unexpected_err(e, Some("an error occurred while receiving".to_string()))
                })
            }
            (DkgCurve::K256(participant), DkgParticipantRoundOutput::K256(participant_data)) => {
                participant.receive(&participant_data.data).map_err(|e| {
                    unexpected_err(e, Some("an error occurred while receiving".to_string()))
                })
            }
            (DkgCurve::P256(participant), DkgParticipantRoundOutput::P256(participant_data)) => {
                participant.receive(&participant_data.data).map_err(|e| {
                    unexpected_err(e, Some("an error occurred while receiving".to_string()))
                })
            }
            (DkgCurve::P384(participant), DkgParticipantRoundOutput::P384(participant_data)) => {
                participant.receive(&participant_data.data).map_err(|e| {
                    unexpected_err(e, Some("an error occurred while receiving".to_string()))
                })
            }
            (
                DkgCurve::Ed25519(participant),
                DkgParticipantRoundOutput::Ed25519(participant_data),
            ) => participant.receive(&participant_data.data).map_err(|e| {
                unexpected_err(e, Some("an error occurred while receiving".to_string()))
            }),
            (
                DkgCurve::Ristretto25519(participant),
                DkgParticipantRoundOutput::Ristretto25519(participant_data),
            ) => participant.receive(&participant_data.data).map_err(|e| {
                unexpected_err(e, Some("an error occurred while receiving".to_string()))
            }),
            (DkgCurve::Ed448(participant), DkgParticipantRoundOutput::Ed448(participant_data)) => {
                participant.receive(&participant_data.data).map_err(|e| {
                    unexpected_err(e, Some("an error occurred while receiving".to_string()))
                })
            }
            (
                DkgCurve::JubJub(participant),
                DkgParticipantRoundOutput::JubJub(participant_data),
            ) => participant.receive(&participant_data.data).map_err(|e| {
                unexpected_err(e, Some("an error occurred while receiving".to_string()))
            }),
            (
                DkgCurve::Decaf377(participant),
                DkgParticipantRoundOutput::Decaf377(participant_data),
            ) => participant.receive(&participant_data.data).map_err(|e| {
                unexpected_err(e, Some("an error occurred while receiving".to_string()))
            }),
            (
                DkgCurve::Bls12381G1ProofOfPossession(participant),
                DkgParticipantRoundOutput::Bls12381G1ProofOfPossession(participant_data),
            ) => participant.receive(&participant_data.data).map_err(|e| {
                unexpected_err(e, Some("an error occurred while receiving".to_string()))
            }),
            _ => Err(unexpected_err(
                "Invalid participant data for dkg curve",
                Some("an error occurred while receiving".to_string()),
            )),
        }
    }

    pub fn get_public_key(&self) -> Option<Vec<u8>> {
        match self {
            DkgCurve::Bls(participant) => participant.get_public_key().map(|pk| {
                <blsful::inner_types::G1Projective as CompressedBytes>::to_compressed(&pk)
            }),
            DkgCurve::K256(participant) => participant
                .get_public_key()
                .map(|pk| <k256::ProjectivePoint as CompressedBytes>::to_compressed(&pk)),
            DkgCurve::P256(participant) => participant
                .get_public_key()
                .map(|pk| <p256::ProjectivePoint as CompressedBytes>::to_compressed(&pk)),
            DkgCurve::P384(participant) => participant
                .get_public_key()
                .map(|pk| <p384::ProjectivePoint as CompressedBytes>::to_compressed(&pk)),
            DkgCurve::Ed25519(participant) => participant.get_public_key().map(|pk| {
                <vsss_rs::curve25519::WrappedEdwards as CompressedBytes>::to_compressed(&pk)
            }),
            DkgCurve::Ristretto25519(participant) => participant.get_public_key().map(|pk| {
                <vsss_rs::curve25519::WrappedRistretto as CompressedBytes>::to_compressed(&pk)
            }),
            DkgCurve::Ed448(participant) => participant
                .get_public_key()
                .map(|pk| <ed448_goldilocks::EdwardsPoint as CompressedBytes>::to_compressed(&pk)),
            DkgCurve::JubJub(participant) => participant
                .get_public_key()
                .map(|pk| <jubjub::SubgroupPoint as CompressedBytes>::to_compressed(&pk)),
            DkgCurve::Decaf377(participant) => participant
                .get_public_key()
                .map(|pk| <decaf377::Element as CompressedBytes>::to_compressed(&pk)),
            DkgCurve::Bls12381G1ProofOfPossession(participant) => {
                participant.get_public_key().map(|pk| {
                    <blsful::inner_types::G1Projective as CompressedBytes>::to_compressed(&pk)
                })
            }
        }
    }

    pub fn get_secret_share(&self) -> Option<Vec<u8>> {
        match self {
            DkgCurve::Bls(participant) => participant.get_secret_share().map(|share| {
                <blsful::inner_types::Scalar as CompressedBytes>::to_compressed(&share.value.0)
            }),
            DkgCurve::K256(participant) => participant
                .get_secret_share()
                .map(|share| <k256::Scalar as CompressedBytes>::to_compressed(&share.value.0)),
            DkgCurve::P256(participant) => participant
                .get_secret_share()
                .map(|share| <p256::Scalar as CompressedBytes>::to_compressed(&share.value.0)),
            DkgCurve::P384(participant) => participant
                .get_secret_share()
                .map(|share| <p384::Scalar as CompressedBytes>::to_compressed(&share.value.0)),
            DkgCurve::Ed25519(participant) => participant.get_secret_share().map(|share| {
                <vsss_rs::curve25519::WrappedScalar as CompressedBytes>::to_compressed(
                    &share.value.0,
                )
            }),
            DkgCurve::Ristretto25519(participant) => participant.get_secret_share().map(|share| {
                <vsss_rs::curve25519::WrappedScalar as CompressedBytes>::to_compressed(
                    &share.value.0,
                )
            }),
            DkgCurve::Ed448(participant) => participant.get_secret_share().map(|share| {
                <ed448_goldilocks::Scalar as CompressedBytes>::to_compressed(&share.value.0)
            }),
            DkgCurve::JubJub(participant) => participant
                .get_secret_share()
                .map(|share| <jubjub::Scalar as CompressedBytes>::to_compressed(&share.value.0)),
            DkgCurve::Decaf377(participant) => participant
                .get_secret_share()
                .map(|share| <decaf377::Fr as CompressedBytes>::to_compressed(&share.value.0)),
            DkgCurve::Bls12381G1ProofOfPossession(participant) => {
                participant.get_secret_share().map(|share| {
                    <blsful::inner_types::Scalar as CompressedBytes>::to_compressed(&share.value.0)
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum DkgRoundOutputGenerator {
    Bls(RoundOutputGenerator<blsful::inner_types::G1Projective>),
    K256(RoundOutputGenerator<k256::ProjectivePoint>),
    P256(RoundOutputGenerator<p256::ProjectivePoint>),
    P384(RoundOutputGenerator<p384::ProjectivePoint>),
    Ed25519(RoundOutputGenerator<vsss_rs::curve25519::WrappedEdwards>),
    Ristretto25519(RoundOutputGenerator<vsss_rs::curve25519::WrappedRistretto>),
    Ed448(RoundOutputGenerator<ed448_goldilocks::EdwardsPoint>),
    JubJub(RoundOutputGenerator<jubjub::SubgroupPoint>),
    Decaf377(RoundOutputGenerator<decaf377::Element>),
    Bls12381G1ProofOfPossession(RoundOutputGenerator<blsful::inner_types::G1Projective>),
}

impl DkgRoundOutputGenerator {
    pub fn iter(&self) -> Box<dyn Iterator<Item = DkgParticipantRoundOutput> + '_> {
        match self {
            DkgRoundOutputGenerator::Bls(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::Bls))
            }
            DkgRoundOutputGenerator::K256(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::K256))
            }
            DkgRoundOutputGenerator::P256(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::P256))
            }
            DkgRoundOutputGenerator::P384(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::P384))
            }
            DkgRoundOutputGenerator::Ed25519(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::Ed25519))
            }
            DkgRoundOutputGenerator::Ristretto25519(generator) => Box::new(
                generator
                    .iter()
                    .map(DkgParticipantRoundOutput::Ristretto25519),
            ),
            DkgRoundOutputGenerator::Ed448(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::Ed448))
            }
            DkgRoundOutputGenerator::JubJub(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::JubJub))
            }
            DkgRoundOutputGenerator::Decaf377(generator) => {
                Box::new(generator.iter().map(DkgParticipantRoundOutput::Decaf377))
            }
            DkgRoundOutputGenerator::Bls12381G1ProofOfPossession(generator) => Box::new(
                generator
                    .iter()
                    .map(DkgParticipantRoundOutput::Bls12381G1ProofOfPossession),
            ),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DkgMessage {
    pub dkg_id: String,
    pub output: DkgParticipantRoundOutput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DkgParticipantRoundOutput {
    Bls(ParticipantRoundOutput<blsful::inner_types::Scalar>),
    K256(ParticipantRoundOutput<k256::Scalar>),
    P256(ParticipantRoundOutput<p256::Scalar>),
    P384(ParticipantRoundOutput<p384::Scalar>),
    Ed25519(ParticipantRoundOutput<vsss_rs::curve25519::WrappedScalar>),
    Ristretto25519(ParticipantRoundOutput<vsss_rs::curve25519::WrappedScalar>),
    Ed448(ParticipantRoundOutput<ed448_goldilocks::Scalar>),
    JubJub(ParticipantRoundOutput<jubjub::Scalar>),
    Decaf377(ParticipantRoundOutput<decaf377::Fr>),
    Bls12381G1ProofOfPossession(ParticipantRoundOutput<blsful::inner_types::Scalar>),
}

impl DkgParticipantRoundOutput {
    pub fn dst_id(&self) -> DkgScalar {
        match self {
            Self::Bls(data) => DkgScalar::Bls(data.dst_id.0),
            Self::K256(data) => DkgScalar::K256(data.dst_id.0),
            Self::P256(data) => DkgScalar::P256(data.dst_id.0),
            Self::P384(data) => DkgScalar::P384(data.dst_id.0),
            Self::Ed25519(data) => DkgScalar::Ed25519(data.dst_id.0),
            Self::Ristretto25519(data) => DkgScalar::Ristretto25519(data.dst_id.0),
            Self::Ed448(data) => DkgScalar::Ed448(data.dst_id.0),
            Self::JubJub(data) => DkgScalar::JubJub(data.dst_id.0),
            Self::Decaf377(data) => DkgScalar::Decaf377(data.dst_id.0),
            Self::Bls12381G1ProofOfPossession(data) => {
                DkgScalar::Bls12381G1ProofOfPossession(data.dst_id.0)
            }
        }
    }

    pub fn dst_ordinal(&self) -> usize {
        match self {
            Self::Bls(data) => data.dst_ordinal,
            Self::K256(data) => data.dst_ordinal,
            Self::P256(data) => data.dst_ordinal,
            Self::P384(data) => data.dst_ordinal,
            Self::Ed25519(data) => data.dst_ordinal,
            Self::Ristretto25519(data) => data.dst_ordinal,
            Self::Ed448(data) => data.dst_ordinal,
            Self::JubJub(data) => data.dst_ordinal,
            Self::Decaf377(data) => data.dst_ordinal,
            Self::Bls12381G1ProofOfPossession(data) => data.dst_ordinal,
        }
    }

    pub fn data(&self) -> Vec<u8> {
        match self {
            Self::Bls(data) => data.data.clone(),
            Self::K256(data) => data.data.clone(),
            Self::P256(data) => data.data.clone(),
            Self::P384(data) => data.data.clone(),
            Self::Ed25519(data) => data.data.clone(),
            Self::Ristretto25519(data) => data.data.clone(),
            Self::Ed448(data) => data.data.clone(),
            Self::JubJub(data) => data.data.clone(),
            Self::Decaf377(data) => data.data.clone(),
            Self::Bls12381G1ProofOfPossession(data) => data.data.clone(),
        }
    }
}

struct CreateParticipantArgs<'a> {
    pub mode: Mode,
    pub peer_id: PeerId,
    pub old_ids: &'a [PeerId],
    pub next_ids: &'a [PeerId],
    pub dkg_data: &'a DkgData,
    pub threshold: NonZeroUsize,
    pub limit: NonZeroUsize,
    pub realm_id: u64,
}

struct CreateDkgResultArgs<'a> {
    pub mode: Mode,
    pub dkg_id: &'a str,
    pub peer_id: PeerId,
    pub pub_key: Option<String>,
    pub curve_type: CurveType,
    pub realm_id: u64,
}
