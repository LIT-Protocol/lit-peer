use super::models::*;
use super::models::{PregenConfirmation, PregenSignal, PresignMessage, SimpleHash, TxnPrefix};
use crate::error::{self, Result, unexpected_err};
use crate::metrics;
use crate::p2p_comms::CommsManager;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tasks::presign_manager::models::Presign;
use crate::tss::common::storage::{delete_presign, read_presign_from_disk, write_presign_to_disk};
use crate::tss::ecdsa_damfast::DamFastState;
use crate::version::DataVersionReader;
use flume::Sender;
use lit_core::config::ReloadableLitConfig;
use lit_node_common::config::{CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT, LitNodeConfig};
use lit_node_core::{CurveType, PeerId, SigningScheme};
use lit_rust_crypto::{
    elliptic_curve::bigint::{self, U256},
    k256, p256, p384,
};
use std::num::NonZeroU64;
use std::time::Duration;
use tracing::instrument;

const PRESIGN_ROUND: &str = "0";

pub(crate) trait PreSignListCurveIndex {
    fn index(&self) -> usize;
}

impl PreSignListCurveIndex for CurveType {
    fn index(&self) -> usize {
        match self {
            CurveType::K256 => 0,
            CurveType::P256 => 1,
            CurveType::P384 => 2,
            _ => 0,
        }
    }
}

impl PresignManager {
    pub async fn listen(
        &mut self,
        mut quit_rx: tokio::sync::broadcast::Receiver<bool>,
        config: ReloadableLitConfig,
    ) {
        info!("Loading Presigs and istening for requests to create new presigs...");
        // either load the presigns from disk and/or generate new ones.
        let mut all_presign_list = Vec::with_capacity(3);
        for curve_type in [CurveType::K256, CurveType::P256, CurveType::P384].iter() {
            let presign_list = self.load_from_disk(*curve_type, true).await;
            trace!(
                "Loaded {} {} presigns from disk. Leader for {}. ",
                curve_type,
                self.current_generation_count[curve_type.index()],
                presign_list.total_shares_count(),
            );
            all_presign_list.push(presign_list);
        }

        self.set_chain_defaults();

        let cfg = config.load_full();
        let timeout = cfg
            .signing_round_timeout()
            .unwrap_or(CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT) as u64;

        loop {
            let mut heartbeat = tokio::time::interval(Duration::from_millis(2 * timeout));
            heartbeat.tick().await; // First tick is immediate
            tokio::select! {
                biased;
                _ = quit_rx.recv() => {
                    info!("Shutting down: tasks::presign_manager");
                    break;
                }
                presign_message = self.rx.recv_async() =>  {
                    let presign_message = match presign_message {
                        Ok(m) => m,
                        Err(e) => {
                            error!("Error receiving message: {}", e);
                            continue;
                        }
                    };
                    match presign_message {
                        PresignMessage::RequestPresign(req, tx) => {
                            let key_hash = req.get_presign_request_key().hash();
                            if req.peers.address_is_leader(key_hash , &self.tss_state.addr) {
                                // if we're the leader, we need to generate a key and return it.
                                let curve_type = req.signing_scheme.curve_type();
                                let presign_list = match get_presign_list_by_curve_type(&mut all_presign_list, curve_type) {
                                    Some(list) => list,
                                    None => {
                                        if tx
                                            .send_async(Err(unexpected_err(
                                                "Invalid curve type.".to_string(),
                                                Some("Invalid curve type.".to_string()),
                                            )))
                                            .await
                                            .is_err()
                                        {
                                            error!("Error returning presign error.");
                                        }
                                        continue;
                                    }
                                };
                                self.leader_node_presign_key_request(req, presign_list, tx).await;
                            } else {
                                // otherwise, we need to send the request to the leader node.
                                self.get_presign_key_from_remote_host(req, tx).await;
                            }
                        }
                        // this is a follow-up to the original RequestPresign function - because the leader may be a remote note, we send a message back the data in this channel once that async function has completed.
                        PresignMessage::FullfillPresignRequest(request_hash, leader_response, peers, tx, signing_scheme) => {
                            debug!("Fulfilling presign request for hash: {}", request_hash);
                            let curve_type = signing_scheme.curve_type();
                            self.check_to_start_regenerating_presigns(request_hash, &peers, leader_response.remaining_presigns, curve_type);
                            if leader_response.presign_storage_key.is_empty() {
                                self.generate_real_time_presign(request_hash, &peers, leader_response.remaining_presigns, tx, signing_scheme).await;
                                trace!("Presign Cache Miss: {}", request_hash);
                            } else {
                                self.return_presign_from_disk(curve_type, request_hash, &peers, leader_response, tx).await;
                                trace!("Presign Cache Hit: {}", request_hash);
                            }
                        }
                        PresignMessage::Generate(txn_hash, peers, curve_type) => {
                            match self.presign_message_generate(txn_hash, &peers, curve_type).await {
                                Ok(_) => {
                                    debug!("Kicked off presign generation for hash: {}", txn_hash);
                                }
                                Err(e) => {
                                    error!("Error kicking off presign generation: {}", e);
                                }
                            }
                        }
                        PresignMessage::Store(presign_box, request_hash) => {
                            let curve_type = presign_box.share.curve_type();
                            if let Some(presign_list) = get_presign_list_by_curve_type(&mut all_presign_list, curve_type) {
                                self.presign_message_store(curve_type, presign_list, presign_box, request_hash).await;
                            }
                        }
                        PresignMessage::InformNonParticipants(request_key_hash, peers) => {
                            debug!("Broadcasting to non-participants for hash: {}", request_key_hash);
                            self.broadcast_to_non_participants(request_key_hash, peers).await;
                        }
                        PresignMessage::Clear => {
                            for curve_type in [CurveType::K256, CurveType::P256, CurveType::P384] {
                                if let Some(presign_list) = get_presign_list_by_curve_type(&mut all_presign_list, curve_type) {
                                    self.presign_message_clear(curve_type, presign_list).await;
                                }
                            }
                        }
                        PresignMessage::Count => {
                            let mut total_presigns = 0;
                            for curve_type in [CurveType::K256, CurveType::P256, CurveType::P384] {
                                if let Some(presign_list) = get_presign_list_by_curve_type(&mut all_presign_list, curve_type) {
                                    total_presigns += presign_list.total_shares_count();
                                }
                            }
                            info!("Total presigns: {}", total_presigns);
                        }
                        PresignMessage::RemoveGenerationHash(txn_hash) => {
                            self.generating_txn_ids.retain(|&x| x != txn_hash);
                            debug!("Removed {}, remaining: {:?}", txn_hash, &self.generating_txn_ids);
                        }
                        PresignMessage::PregenVerified(request_hash, peers, remaining_presigns, curve_type) => {
                            self.check_to_start_regenerating_presigns(request_hash, &peers, remaining_presigns, curve_type);
                        }
                    }
                }
                result = self.collect_pregen_signal() => {
                    match result {
                        Ok(messages) => {
                            if let Some((_, signal)) = messages.into_iter().next() {
                                self.handle_pregen_signal(signal).await;
                            }
                        }
                        Err(e) => {
                                trace!("Presign pregen listener timed out without receiving a signal: {}", e);

                            // match e.msg() {


                            //     Some(msg) if msg.contains("Timeout") || msg.contains("Peer not found in peer list") => {
                            //         trace!("Presign pregen listener timed out without receiving a signal: {}", e);
                            //     }
                            //     _ => {
                            //         error!("Error collecting from pregen signal comms manager: {}", e);
                            //     }
                            // }
                        }
                    }
                }
                _ = heartbeat.tick() => {
                    self.set_chain_defaults();
                }
            }
        }
    }

    fn set_chain_defaults(&mut self) {
        DataVersionReader::reader_unchecked(
            &self.tss_state.chain_data_config_manager.generic_config,
            |generic_chain_config| {
                // TODO: create separate limits for this
                self.min_presigns = generic_chain_config.min_presign_count;
                self.max_presigns = generic_chain_config.max_presign_count;
                self.max_presign_concurrency = generic_chain_config.max_presign_concurrency;
            },
        );
    }

    #[instrument(level = "debug", skip_all, fields(txn_prefix = req.txn_prefix))]
    async fn leader_node_presign_key_request(
        &mut self,
        req: PresignRequest,
        presign_list: &mut PresignListByGroup,
        tx: Sender<Result<Option<Presign>>>,
    ) {
        let request_key = PresignRequestKey::from(req.clone());
        let request_hash = request_key.hash();
        let txn_prefix = TxnPrefix::GetPresign(request_hash).as_str();
        let round = PRESIGN_ROUND;
        let state = self.tss_state.clone();
        let signing_peers = req.peers.clone();

        trace!(
            "Is leader for txn prefix {}, request hash {}, and peer set {:?}",
            txn_prefix,
            request_hash,
            signing_peers.debug_addresses()
        );

        let cm_for_participants =
            match CommsManager::new_with_peers(&state, &txn_prefix, &signing_peers, round).await {
                Ok(c) => c,
                Err(e) => {
                    error!("Error setting up comms manager: {}", e);
                    if tx
                        .send_async(Err(unexpected_err(
                            e,
                            Some("Error setting up comms manager.".to_string()),
                        )))
                        .await
                        .is_err()
                    {
                        error!("Error returning presign error.");
                    }
                    return;
                }
            };

        let local_tx = self.tx.clone();
        let leader_response = self
            .get_storage_key_from_request_key(
                request_key.clone(),
                &req.peers,
                presign_list,
                false,
                req.threshold,
                tx.clone(),
            )
            .await;
        let remaining_presigns = leader_response.remaining_presigns;

        // Spawn task for broadcasting to participants
        tokio::spawn(async move {
            trace!(
                "Broadcasting presign key to peers: {:?} for txn_prefix: {}",
                req.peers.debug_addresses(),
                txn_prefix
            );
            if let Err(e) = cm_for_participants.broadcast(leader_response.clone()).await {
                error!("Error broadcasting presign key to peers: {}", e);
                if tx
                    .send_async(Err(unexpected_err(
                        e,
                        Some("Error sending presign key to peers.".to_string()),
                    )))
                    .await
                    .is_err()
                {
                    error!("Error returning presign error.");
                }
                return;
            }

            let presign_message = PresignMessage::FullfillPresignRequest(
                request_hash,
                leader_response,
                req.peers.clone(),
                tx.clone(),
                req.signing_scheme,
            );

            if local_tx.send_async(presign_message).await.is_err() {
                error!("Error returning presign key.");
            }
        });

        // Only create comms manager and broadcast to non-participants if remaining presigns is below threshold
        if remaining_presigns < self.min_presigns {
            let nonparticipants = match self.get_non_participants(&signing_peers) {
                Ok(p) => p,
                Err(e) => {
                    error!("Error getting non-participants: {}", e);
                    return;
                }
            };

            let state = self.tss_state.clone();
            let curve_type = req.signing_scheme.curve_type();

            // Spawn separate task for non-participants
            tokio::spawn(async move {
                let cm_for_nonparticipants = match CommsManager::new_with_peers(
                    &state,
                    &TxnPrefix::PregenSignal.as_str(),
                    &nonparticipants,
                    round,
                )
                .await
                {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Error setting up comms manager: {}", e);
                        return;
                    }
                };

                let presign_signal = PregenSignal {
                    request_hash,
                    curve_type,
                    remaining_presigns,
                };
                if let Err(e) = cm_for_nonparticipants.broadcast(presign_signal).await {
                    error!("Error broadcasting presign key to non-participants: {}", e);
                }
            });
        }
    }

    #[doc = "Generates new presigns"]
    #[instrument(level = "debug", skip_all)]
    async fn presign_message_generate(
        &mut self,
        txn_hash: u64,
        peers: &SimplePeerCollection,
        curve_type: CurveType,
    ) -> Result<()> {
        let threshold = self.tss_state.get_threshold().await;

        let signing_state = match curve_type {
            CurveType::K256 => {
                DamFastState::new(self.tss_state.clone(), SigningScheme::EcdsaK256Sha256)
            }
            CurveType::P256 => {
                DamFastState::new(self.tss_state.clone(), SigningScheme::EcdsaP256Sha256)
            }
            CurveType::P384 => {
                DamFastState::new(self.tss_state.clone(), SigningScheme::EcdsaP384Sha384)
            }
            _ => {
                return Err(unexpected_err("Unsupported curve type.", None));
            }
        };
        let tx = self.tx.clone();
        let txn_prefix = TxnPrefix::PregenPresign(txn_hash, curve_type).as_str();
        let start = std::time::Instant::now();

        if !peers.contains_address(&self.tss_state.addr) {
            let _r = tx
                .send_async(PresignMessage::RemoveGenerationHash(txn_hash))
                .await;
            warn!(
                "Non active peer was asked to generate presigns. Aborting request to generate; other peers will still succeed."
            );
            return Ok(());
        }

        let mut active_peers = peers.clone();
        tokio::spawn(async move {
            trace!(
                "Generating presign for txn prefix: {} and curve: {}",
                txn_prefix, curve_type
            );
            let result = match curve_type {
                CurveType::K256 => signing_state
                    .create_presignature_for_peers::<k256::Secp256k1>(
                        &txn_prefix.clone(),
                        &mut active_peers,
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::K256),
                CurveType::P256 => signing_state
                    .create_presignature_for_peers::<p256::NistP256>(
                        &txn_prefix.clone(),
                        &mut active_peers,
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::P256),
                CurveType::P384 => signing_state
                    .create_presignature_for_peers::<p384::NistP384>(
                        &txn_prefix.clone(),
                        &mut active_peers,
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::P384),
                _ => Err(unexpected_err("Unsupported curve type.", None)),
            };
            match result {
                Ok(result) => {
                    debug!("Generated presign in {} ms.", start.elapsed().as_millis());
                    metrics::counter::add_one(metrics::tss::PresignMetrics::Generate, &[]);
                    let peer = match active_peers.peer_at_address(&signing_state.state.addr) {
                        Ok(p) => p,
                        Err(e) => {
                            error!("Error getting peer at address: {}", e);
                            return;
                        }
                    };
                    let staker_hash = peer.key_hash;
                    let presign = match Presign::new(result, staker_hash, &active_peers) {
                        Ok(p) => p,
                        Err(e) => {
                            error!("Error creating presign: {}", e);
                            return;
                        }
                    };
                    if let Err(e) = tx
                        .send_async(PresignMessage::Store(Box::new(presign), txn_hash))
                        .await
                    {
                        error!("Error sending presign to store: {}", e);
                    }
                }
                Err(e) => {
                    let _r = tx
                        .send_async(PresignMessage::RemoveGenerationHash(txn_hash))
                        .await;

                    error!(
                        "Error generating presign: {} in {} ms",
                        e,
                        start.elapsed().as_millis()
                    );
                }
            }
        });
        Ok(())
    }

    #[doc = "Clears the presign from internal map and disk"]
    #[instrument(level = "debug", skip_all)]
    async fn presign_message_clear(
        &mut self,
        curve_type: CurveType,
        presign_list: &mut PresignListByGroup,
    ) {
        info!("Clearing and starting repopulation...");
        let old_map = presign_list.clone();
        presign_list.clear();
        let peer_id = match self.tss_state.peer_state.peer_id_in_current_epoch() {
            Ok(id) => id,
            Err(e) => {
                error!("Error getting node index: {}", e);
                return;
            }
        };

        let staker_address = self.tss_state.peer_state.hex_staker_address();
        let key_cache = &self.tss_state.key_cache;
        let epoch = self.tss_state.peer_state.epoch();
        let realm = self.tss_state.peer_state.realm_id();

        for key in old_map.presign_list_values() {
            let staker_address = staker_address.clone();
            let local_key_cache = key_cache.clone();
            tokio::spawn(async move {
                // delete the files
                if let Err(e) = delete_presign(
                    curve_type,
                    &key.to_string(),
                    &staker_address,
                    epoch,
                    realm,
                    &local_key_cache,
                )
                .await
                {
                    error!("Error deleting presign: {:?}", e);
                }
            });
        }

        info!("Cleared presign list; actual presigns on disk are being deleted.");
    }

    #[doc = "Stores a presign to disk and adds it to the presign list.  Will request another presign to be generated if the list is not full."]
    #[instrument(level = "debug", skip_all)]
    async fn presign_message_store(
        &mut self,
        curve_type: CurveType,
        presign_list: &mut PresignListByGroup,
        presign_box: Box<Presign>,
        request_hash: u64,
    ) {
        let presign = *presign_box;
        let tag = presign.share.tag();
        let staker_address = self.tss_state.peer_state.hex_staker_address();
        let key_cache = &self.tss_state.key_cache;
        let epoch = self.tss_state.peer_state.epoch();
        let realm = self.tss_state.peer_state.realm_id();

        debug!("Storing presign... key: {:?}", tag);
        if let Err(e) = write_presign_to_disk(
            curve_type,
            &tag,
            &staker_address,
            epoch,
            realm,
            key_cache,
            &presign,
        )
        .await
        {
            error!("Error writing presign to disk: {}", e);
        };
        metrics::counter::add_one(metrics::tss::PresignMetrics::Store, &[]);
        self.current_generation_count[curve_type.index()] += 1;

        // if we're to be the "leader" for this presign, add it to the list.
        let peers = self.get_active_peers();

        let peer_cnt = match NonZeroU64::new(peers.0.len() as u64) {
            None => {
                error!("No peers found in presign message store.");
                return;
            }
            Some(c) => c,
        };
        let modulus = bigint::NonZero::<U256>::from_u64(peer_cnt);
        let idx = (U256::from_be_hex(&tag) % modulus).as_words()[0];
        let me = &peers.0[idx as usize];

        if me.socket_address == self.tss_state.addr {
            presign_list.add_storage_key(presign.peer_group_id, tag);
        }

        let xor_filter_with_threshold = XorFilterWithThreshold {
            filter: presign.xor_filter,
            threshold: presign.share.threshold(),
        };

        self.xor_filters
            .entry(presign.peer_group_id)
            .or_insert(xor_filter_with_threshold);
        // end of adding presign to the list, if I'm the leader.

        // remove the range of presigns being generated that is greater than the max, as sorted by u64 (hash value).
        self.generating_txn_ids.sort();
        if self.generating_txn_ids.len() > self.max_presign_concurrency as usize {
            let range_to_remove =
                (self.max_presign_concurrency as usize)..self.generating_txn_ids.len();
            self.generating_txn_ids.drain(range_to_remove);
        }

        // if we're no longer in the list, we're done!
        if !self.generating_txn_ids.contains(&request_hash) {
            return;
        }

        self.generating_txn_ids.retain(|&x| x != request_hash);

        // check if we should keep generating
        // also, make it possible to disable presign by setting max presigns to 0
        if self.current_generation_count[curve_type.index()] < self.max_presigns
            && self.max_presigns != 0
        {
            // this blocks other concurrent requests from coming in.
            // it's possible that two or three happen right at the start and eventually some get aborted.
            self.last_generated[curve_type.index()] = std::time::Instant::now(); // not correct, but useful if all requests fail.
            debug!(
                "Requesting more presigns. Generation at {} of {}.",
                self.current_generation_count[curve_type.index()],
                self.max_presigns
            );

            let next_request_hash = request_hash + 1;
            self.generating_txn_ids.push(next_request_hash);
            debug!(
                "Requesting more presigns. Generation at {} of {}. \n Removed {} : current {:?}",
                self.current_generation_count[curve_type.index()],
                self.max_presigns,
                request_hash,
                &self.generating_txn_ids
            );
            if let Err(e) = self
                .tx
                .send_async(PresignMessage::Generate(
                    next_request_hash,
                    peers,
                    presign.share.curve_type(),
                ))
                .await
            {
                error!("Error sending generate message: {}", e);
            };
        } else {
            debug!(
                "Finished generating {} presigns.",
                self.current_generation_count[curve_type.index()]
            );
        }
    }

    #[doc = "Executes a presign generation process in a new thread and returns it directly to the calling function - used when a presign is needed in real time and the list is empty."]
    #[instrument(level = "debug", skip(self), fields(txn_prefix = format!("rt_{}", presign_hash)))]
    async fn generate_real_time_presign(
        &mut self,
        presign_hash: PresignRequestKeyHash,
        peers: &SimplePeerCollection,
        remaining_presign: u64,
        tx: Sender<Result<Option<Presign>>>,
        signing_scheme: SigningScheme,
    ) {
        let signing_state = DamFastState::new(self.tss_state.clone(), signing_scheme);
        let txn_prefix =
            TxnPrefix::RealTimePresign(presign_hash, signing_scheme.curve_type()).as_str();
        trace!(
            "Generating real time presign {} with peers {:?}.",
            txn_prefix,
            peers.debug_addresses()
        );
        let included = peers.contains_address(&self.tss_state.addr);
        let mut peers = peers.clone();

        let threshold = peers.0.len();
        match self
            .tss_state
            .get_threshold_using_current_epoch_realm_peers_for_curve(
                &peers,
                signing_scheme.curve_type(),
                None,
            )
            .await
        {
            Ok(t) => {
                debug!(
                    "generate real time presign: number of peers {} / key share threshold: {}",
                    threshold, t
                )
            }
            Err(_e) => {}
        };

        tokio::spawn(async move {
            // if we're not part of this generation, just return a blank presign.
            if !included {
                trace!("Not included in this generation.  Returning blank presign.");
                if tx.send_async(Ok(None)).await.is_err() {
                    error!("Error returning presign.");
                }
                return;
            }
            let result = match signing_scheme {
                SigningScheme::EcdsaK256Sha256 => signing_state
                    .create_presignature_for_peers::<k256::Secp256k1>(
                        &txn_prefix.clone(),
                        &mut peers,
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::K256),
                SigningScheme::EcdsaP256Sha256 => signing_state
                    .create_presignature_for_peers::<p256::NistP256>(
                        &txn_prefix.clone(),
                        &mut peers,
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::P256),
                SigningScheme::EcdsaP384Sha384 => signing_state
                    .create_presignature_for_peers::<p384::NistP384>(
                        &txn_prefix.clone(),
                        &mut peers,
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::P384),
                scheme => Err(unexpected_err(
                    format!("Unsupported scheme {}.", scheme),
                    None,
                )),
            };

            match result {
                Ok(result) => {
                    trace!(
                        "Generated presign in real time: {} scheme: {}",
                        txn_prefix, signing_scheme
                    );
                    metrics::counter::add_one(metrics::tss::PresignMetrics::GenerateRealTime, &[]);
                    let peer = match peers.peer_at_address(&signing_state.state.addr) {
                        Ok(p) => p,
                        Err(e) => {
                            error!("Error getting peer at address: {}", e);
                            return;
                        }
                    };
                    let staker_hash = peer.key_hash;
                    let presign = match Presign::new(result, staker_hash, &peers) {
                        Ok(p) => p,
                        Err(e) => {
                            error!("Error generating real time presign: {}", e);
                            if let Err(e) = tx.send_async(Err(unexpected_err(e, None))).await {
                                error!("Error returning generated real time presign error: {}", e);
                            }
                            return;
                        }
                    };
                    if let Err(e) = tx.send_async(Ok(Some(presign))).await {
                        error!("Error returning generated real time presign: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error generating real time presign: {}", e);
                    if let Err(e) = tx
                        .send_async(Err(unexpected_err(
                            "Could not generate real time presign.",
                            None,
                        )))
                        .await
                    {
                        error!("Error returning generated real time presign error: {}", e);
                    }
                }
            };
        });
    }

    #[doc = "Checks to see if we need to start generating presigns, and if so, sends a message to the channel to do so."]
    pub fn check_to_start_regenerating_presigns(
        &mut self,
        presign_hash: u64,
        peers: &SimplePeerCollection,
        remaining_presign: u64,
        curve_type: CurveType,
    ) {
        // if the incoming remaining presigns is above the low threshold mark, don't do anything.
        // also, make it possible to disable presign Presign by setting max presigns to 0
        if remaining_presign > self.min_presigns || self.max_presigns == 0 {
            return;
        }

        // set the "counter" to whatever the leader says needs to be generated.
        self.current_generation_count[curve_type.index()] = remaining_presign;
        self.generating_txn_ids.push(presign_hash);
        let local_tx = self.tx.clone();
        // this spawn probably doesn't add much - it's a pretty quick message to send.
        let peers = peers.clone();
        tokio::spawn(async move {
            trace!("Sending generate message for hash: {}", presign_hash);
            if let Err(e) = local_tx
                .send_async(PresignMessage::Generate(presign_hash, peers, curve_type))
                .await
            {
                error!("Error sending generate message: {}", e);
            };
        });
    }

    // error handling to return a message back to the request originator - ie cait-sith
    pub async fn return_error_to_requester(
        &self,
        tx: Sender<Result<Option<Presign>>>,
        error: String,
    ) {
        error!("Error getting presign: {}", error);
        if tx
            .send_async(Err(unexpected_err(error, None)))
            .await
            .is_err()
        {
            error!("Error returning presign through channel.");
        }
    }

    #[instrument(level = "debug", skip_all)]
    #[allow(clippy::too_many_arguments)]
    async fn get_storage_key_from_request_key(
        &mut self,
        request_key: PresignRequestKey,
        request_peers: &SimplePeerCollection,
        presign_list: &mut PresignListByGroup,
        is_local_request: bool,
        threshold: u16,
        tx: Sender<Result<Option<Presign>>>,
    ) -> PresignLeaderResponse {
        let request_hash = request_key.hash();
        trace!("Getting next presign for hash: {} ", request_hash);
        let leader_response = self
            .get_next_presign_storage_key_from_presign_list(
                presign_list,
                threshold,
                request_peers,
                request_key,
            )
            .await;
        debug!(
            "Got local key for {}, from presign list: {}.  Presign remaining: {}",
            request_hash, leader_response.presign_storage_key, leader_response.remaining_presigns
        );

        leader_response
    }

    #[instrument(level = "debug", skip_all)]
    async fn return_presign_from_disk(
        &mut self,
        curve_type: CurveType,
        request_hash: PresignRequestKeyHash,
        peers: &SimplePeerCollection,
        leader_response: PresignLeaderResponse,
        tx: Sender<Result<Option<Presign>>>,
    ) {
        let pubkey = leader_response.presign_storage_key.to_string();
        let staker_address = self.tss_state.peer_state.hex_staker_address();
        let included = peers.contains_address(&self.tss_state.addr);
        let epoch = self.tss_state.peer_state.epoch();
        let realm_id = self.tss_state.peer_state.realm_id();
        let key_cache = self.tss_state.key_cache.clone();

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            let presign = match read_presign_from_disk::<Presign>(
                curve_type,
                &pubkey,
                &staker_address,
                epoch,
                realm_id,
                &key_cache,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    if e.is_kind(lit_core::error::Kind::Io, true) {
                        // If there is an error reading the file, we assume that it is because the file doesn't exist.
                        // In that case, we first check whether this peer is part of the peer subset:
                        // - If yes, this is a serious error, and we should produce an error log.
                        // - If not, this is an expected scenario, and we should produce an info log and return Ok(None).
                        if included {
                            error!(
                                "Error reading presign from disk: {} and self is included in peers - this is an issue.",
                                e
                            );
                            match tx.send_async(Err(e)).await {
                                Ok(_) => {
                                    trace!("Returned error to presign requester");
                                }
                                Err(e) => {
                                    error!("Error returning error to presign requester: {}", e);
                                }
                            }
                            return;
                        } else {
                            trace!(
                                "Could not read presign from disk: {}, but self is not in peers, so this is not an issue.",
                                e
                            );
                            match tx.send_async(Ok(None)).await {
                                Ok(_) => {
                                    trace!("Returned response to presign requester");
                                }
                                Err(e) => {
                                    error!("Error returning response to presign requester: {}", e);
                                }
                            }
                            return;
                        }
                    } else {
                        error!("Error reading presign from disk: {}", e);
                        match tx.send_async(Err(e)).await {
                            Ok(_) => {
                                trace!("Returned error to presign requester");
                            }
                            Err(e) => {
                                error!("Error returning error to presign requester: {}", e);
                            }
                        }
                        return;
                    }
                }
            };

            // delete the file only if this is a local request.
            trace!("Deleting local presign file: {}", pubkey);
            //hardcode 1 for share index, since it doesn't matter,
            if let Err(e) = delete_presign(
                curve_type,
                &pubkey,
                &staker_address,
                epoch,
                realm_id,
                &key_cache,
            )
            .await
            {
                error!("Error deleting presign file: {:?}", e);
                match tx.send_async(Err(e)).await {
                    Ok(_) => {
                        info!("Returned error to presign requester");
                    }
                    Err(e) => {
                        error!("Error returning error to presign requester: {}", e);
                    }
                }
                return;
            }

            match tx.send_async(Ok(Some(presign))).await {
                Ok(_) => {
                    info!("Successfully returned presign response.");
                }
                Err(e) => {
                    error!("Error returning presign: {}", e);
                }
            }
            debug!("Raw read & delete from disk in {:?}.", start.elapsed());
        });
    }

    #[instrument(level = "debug", skip_all)]
    async fn get_next_presign_storage_key_from_presign_list(
        &mut self,
        presign_list: &mut PresignListByGroup,
        threshold: u16,
        peers: &SimplePeerCollection,
        request_key: PresignRequestKey,
    ) -> PresignLeaderResponse {
        let request_hash = request_key.hash();
        debug!("We are the leader node - getting key from local storage.");

        let peer_group_id =
            self.get_peer_group_id_from_xor_filter(presign_list, peers, threshold as usize);
        if peer_group_id == 0 {
            debug!("No presigns for peers: {:?}", peers.debug_addresses());
            return PresignLeaderResponse {
                presign_storage_key: "".to_string(),
                remaining_presigns: 0,
            }; // if we don't have any presigns for this group, return 0.
        }

        // technically this should always be something, even if it's empty, and we already did a len check on the quantity....
        let presign_list = match presign_list.get_mut(&peer_group_id) {
            Some(v) => v,
            None => {
                error!(
                    "Error getting presign list from hashmap for group id: {}",
                    peer_group_id
                );
                return PresignLeaderResponse {
                    presign_storage_key: "".to_string(),
                    remaining_presigns: 0,
                };
            }
        };

        // get the next key from our list
        let presign_key = presign_list.pop_front();

        // add it to the hashmap & return, or return 0 if something failed.
        let presign_key = presign_key.unwrap_or("".to_string());

        if presign_key.is_empty() {
            warn!("Leader has no active presigns.");
        };

        let request_hash = request_key.hash();
        info!(
            "Adding key to request map {:?} -> {:?}  ",
            request_hash, presign_key
        );

        // this may not be relevant for just storing in the map, but to match the leader request....
        let remaining_presigns = match self.generating_txn_ids.is_empty() {
            true => {
                if (presign_list.len() as u64) < self.min_presigns {
                    // this returns a fixed value since these results are sometimes returned in varying orders
                    // a fixed results ensures that the "last" value received by every node is the same, making
                    // the number of presigns to be generated equivalent across all nodes
                    self.min_presigns
                } else {
                    presign_list.len() as u64
                }
            }
            false => self.max_presigns,
        };
        PresignLeaderResponse {
            presign_storage_key: presign_key,
            remaining_presigns,
        }
    }

    #[instrument(level = "debug", skip_all)]
    async fn get_presign_key_from_remote_host(
        &self,
        req: PresignRequest,
        tx: Sender<error::Result<Option<Presign>>>,
    ) {
        debug!("Getting presign key from remote host.");

        let request_key = PresignRequestKey::from(req.clone());
        let request_key_hash = request_key.hash();

        let txn_prefix = TxnPrefix::GetPresign(request_key_hash).as_str();
        let round = PRESIGN_ROUND;
        let state = self.tss_state.clone();
        let signing_peers = req.peers.clone();

        trace!(
            "Waiting on leader selection for txn prefix {}, request hash {}, and peer set: {:?}",
            txn_prefix,
            request_key_hash,
            signing_peers.debug_addresses()
        );

        let cm =
            match CommsManager::new_with_peers(&state, &txn_prefix, &signing_peers, round).await {
                Ok(c) => c,
                Err(e) => {
                    error!("Error setting up comms manager: {}", e);
                    if tx
                        .send_async(Err(unexpected_err(
                            e,
                            Some("Error setting up comms manager.".to_string()),
                        )))
                        .await
                        .is_err()
                    {
                        error!("Error returning presign error.");
                    }
                    return;
                }
            };

        let signing_peers_cloned = signing_peers.clone();
        let leader_peer = match signing_peers_cloned.leader_for_active_peers(request_key_hash) {
            Ok(peer) => peer,
            Err(e) => {
                if tx.send_async(Err(e)).await.is_err() {
                    error!("Error returning presign error.");
                }
                return;
            }
        }
        .clone();

        trace!(
            "Leader peer/signing peers: {:?} / {:?}",
            leader_peer,
            signing_peers.debug_addresses()
        );

        let local_tx = self.tx.clone();
        let min_presigns = self.min_presigns;

        let nonparticipants = match self.get_non_participants(&signing_peers) {
            Ok(p) => p,
            Err(e) => {
                error!("Error getting non-participants: {}", e);
                return;
            }
        };

        tokio::spawn(async move {
            let leader_vec = SimplePeerCollection(vec![leader_peer.clone()]);

            let presign_leader_response =
                match cm.collect_from::<PresignLeaderResponse>(&leader_vec).await {
                    Ok(r) => r,
                    Err(e) => {
                        error!("Error getting leader response: {}", e);
                        if tx
                            .send_async(Err(unexpected_err(
                                e,
                                Some("Error getting leader response.".to_string()),
                            )))
                            .await
                            .is_err()
                        {
                            error!("Error returning presign error.");
                        }
                        return;
                    }
                };

            let presign_leader_response = match presign_leader_response.into_iter().next() {
                Some(r) => {
                    trace!("Got leader response: {:?}", r);
                    r.1
                }
                None => {
                    if tx
                        .send_async(Err(unexpected_err("Invalid leader response.", None)))
                        .await
                        .is_err()
                    {
                        error!("Error returning presign error.");
                    }
                    return;
                }
            };

            if presign_leader_response.remaining_presigns < min_presigns
                && let Err(e) = local_tx
                    .send_async(PresignMessage::InformNonParticipants(
                        request_key_hash,
                        nonparticipants,
                    ))
                    .await
            {
                error!("Error sending inform non participants message: {}", e);
            }

            let presign_message = PresignMessage::FullfillPresignRequest(
                request_key_hash,
                presign_leader_response,
                req.peers.clone(),
                tx,
                req.signing_scheme,
            );

            // send this remote request back for processing.
            if local_tx.send_async(presign_message).await.is_err() {
                error!("Error returning presign key.");
            }
        });
    }

    async fn broadcast_to_non_participants(
        &self,
        request_key_hash: u64,
        peers: SimplePeerCollection,
    ) {
        let round = PRESIGN_ROUND;
        let txn_prefix = TxnPrefix::ConfirmPregenPresign(request_key_hash).as_str();
        let cm =
            match CommsManager::new_with_peers(&self.tss_state.clone(), &txn_prefix, &peers, round)
                .await
            {
                Ok(c) => c,
                Err(e) => {
                    error!("Error setting up comms manager: {}", e);
                    return;
                }
            };
        let presign_confirmation = PregenConfirmation {};
        tokio::spawn(async move {
            match cm.broadcast(presign_confirmation).await {
                Ok(_) => {
                    info!("Broadcasted presign confirmation to non-participants.");
                }
                Err(e) => {
                    error!("Error broadcasting presign confirmation: {}", e);
                }
            }
        });
    }

    fn get_active_peers(&self) -> SimplePeerCollection {
        self.tss_state.peer_state.peers().active_peers()
    }

    fn get_non_participants(
        &self,
        signing_peers: &SimplePeerCollection,
    ) -> Result<SimplePeerCollection> {
        Ok(SimplePeerCollection(
            self.get_active_peers()
                .0
                .into_iter()
                .filter(|p| !signing_peers.contains_address(&p.socket_address))
                .collect(),
        ))
    }

    async fn handle_pregen_signal(&mut self, signal: PregenSignal) {
        info!(
            "Received new pregen signal for request hash: {}",
            signal.request_hash
        );
        let txn_prefix = TxnPrefix::PregenPresign(signal.request_hash, signal.curve_type).as_str();
        let state_clone = self.tss_state.clone();
        let peers_clone = self.get_active_peers();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            match CommsManager::new_with_peers(
                &state_clone,
                &txn_prefix,
                &peers_clone,
                PRESIGN_ROUND,
            )
            .await
            {
                Ok(new_cm) => {
                    let threshold = state_clone.get_threshold().await;
                    match new_cm
                        .collect_from_earliest::<()>(&peers_clone, threshold - 1)
                        .await
                    {
                        Ok(_) => {
                            info!(
                                "Collected threshold messages for request hash: {}",
                                signal.request_hash
                            );
                            if let Err(e) = tx
                                .send_async(PresignMessage::PregenVerified(
                                    signal.request_hash,
                                    peers_clone,
                                    signal.remaining_presigns,
                                    signal.curve_type,
                                ))
                                .await
                            {
                                error!("Error sending pregen verified message: {}", e);
                            }
                        }
                        Err(e) => {
                            // TODO complain about leader?
                            error!("Error collecting threshold messages: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error setting up new comms manager: {}", e);
                }
            }
        });
    }

    async fn collect_pregen_signal(&self) -> Result<Vec<(PeerId, PregenSignal)>> {
        let txn_prefix = TxnPrefix::PregenSignal.as_str();
        let round = PRESIGN_ROUND;
        let state = self.tss_state.clone();
        let peers = self.get_active_peers();
        if !peers.0.iter().any(|p| p.socket_address == state.addr) {
            // this is a temp fix to give some cycles back to the node.... this only happens when the node isn't active, but no use spinning the CPU needlessly.
            tokio::time::sleep(Duration::from_millis(5000)).await;
            return Ok(vec![]);
        }
        let mut cm = CommsManager::new_with_peers(&state, &txn_prefix, &peers, round).await?;
        cm.set_timeout(Duration::from_millis(
            (CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT / 2) as u64,
        ));
        cm.poll_from_earliest::<PregenSignal>(&peers, 1).await
    }
}

fn get_presign_list_by_curve_type(
    all_presign_list: &mut [PresignListByGroup],
    curve_type: CurveType,
) -> Option<&mut PresignListByGroup> {
    match all_presign_list.get_mut(curve_type.index()) {
        Some(list) => Some(list),
        None => {
            error!("Invalid curve type: {:?}", curve_type);
            None
        }
    }
}
