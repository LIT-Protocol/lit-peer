use std::collections::HashMap;
use std::num::NonZeroU64;

use super::models::{
    Presign, PresignListByGroup, PresignListByGroupTrait, PresignManager, XorFilterWithThreshold,
};
use crate::error::{Result, unexpected_err};
use crate::peers::peer_state::models::{SimplePeer, SimplePeerCollection};
use crate::tasks::presign_manager::listener::PreSignListCurveIndex;
use crate::tss::common::storage::read_presign_from_disk_direct;
use async_std::fs::{self, DirEntry};
use async_std::io::Error;
use async_std::path::PathBuf;
use futures::StreamExt;
use lit_node_common::config::presign_path;
use lit_node_core::CurveType;
use lit_rust_crypto::elliptic_curve::bigint::{self, U256};
use xorf::Filter;

impl PresignManager {
    pub async fn load_from_disk(
        &mut self,
        curve_type: CurveType,
        initial_load: bool,
    ) -> PresignListByGroup {
        match self.do_load_from_disk(curve_type).await {
            Ok(presign_list) => presign_list,
            Err(e) => {
                match initial_load {
                    true => trace!(
                        "Initial presign loading returned (no path is normal): {}",
                        e
                    ),
                    false => error!("Error loading presign from disk: {}", e),
                }
                PresignListByGroup::new()
            }
        }
    }

    async fn do_load_from_disk(&mut self, curve_type: CurveType) -> Result<PresignListByGroup> {
        let mut presign_list = PresignListByGroup::new();
        let mut peers = self.tss_state.peer_state.peers();
        let node_addr = self.tss_state.peer_state.addr.clone();
        // until we have peers, we can't really do anything
        while peers.0.is_empty() {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            peers = self.tss_state.peer_state.peers();
        }

        let staker_address = &self.tss_state.peer_state.hex_staker_address();
        let path = presign_path(curve_type.as_str(), staker_address);
        trace!("Loading presigns from disk.");
        self.recurse_dirs(path, &mut presign_list, &peers, &node_addr, curve_type)
            .await?;

        Ok(presign_list)
    }

    async fn recurse_dirs(
        &mut self,
        path: PathBuf,
        presign_list: &mut PresignListByGroup,
        peers: &SimplePeerCollection,
        node_addr: &String,
        curve_type: CurveType,
    ) -> Result<()> {
        let mut dirs = fs::read_dir(path)
            .await
            .map_err(|e| unexpected_err(e, Some("Presign path not found.".into())))?;

        while let Some(res) = dirs.next().await {
            let entry =
                res.map_err(|e| unexpected_err(e, Some("Presign folder read error.".into())))?;
            let filetype = entry
                .file_type()
                .await
                .map_err(|e| unexpected_err(e, Some("Presign file type read error.".into())))?;

            if filetype.is_dir() {
                let path = entry.path();
                Box::pin(self.recurse_dirs(path, presign_list, peers, node_addr, curve_type))
                    .await?;
            } else if filetype.is_file()
                && let Err(r) = self
                    .attempt_load_presign(entry.clone(), presign_list, peers, node_addr, curve_type)
                    .await
            {
                error!("Error loading presign {:?}: {:?}", entry, r);
            }
        }
        Ok(())
    }

    async fn attempt_load_presign(
        &mut self,
        entry: DirEntry,
        presign_list: &mut PresignListByGroup,
        peers: &SimplePeerCollection,
        node_addr: &str,
        curve_type: CurveType,
    ) -> Result<()> {
        let peer_id = peers.peer_id_by_address(node_addr)?;

        let filename = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => {
                error!("Error reading filename: {:?}", e);
                return Err(unexpected_err(
                    Error::other("file"),
                    Some("Presign filename read error.".into()),
                ));
            }
        };

        let entry_path = entry.path();
        let filename = match entry_path.to_str() {
            Some(s) => s,
            None => {
                error!("Error reading filename: {:?}", entry.path());
                return Err(unexpected_err(
                    Error::other("file"),
                    Some("Presign filename read error.".into()),
                ));
            }
        };

        let share_ending = format! {"{}-H.cbor", peer_id};
        if filename.ends_with(share_ending.as_str()) {
            let presign =
                match read_presign_from_disk_direct::<Presign>(filename, &self.tss_state.key_cache)
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Error reading presign file: {:?}", e);
                        return Err(e);
                    }
                };

            let peer_group_id = presign.peer_group_id;

            let tag = presign.share.tag();
            let xor_filter_with_threshold = XorFilterWithThreshold {
                filter: presign.xor_filter,
                threshold: presign.share.threshold(),
            };

            self.xor_filters
                .entry(peer_group_id)
                .or_insert(xor_filter_with_threshold);

            let pregen_creation_peers = self
                .node_socket_addresses_from_peer_group_id(peer_group_id, peers)
                .await;

            let peer_cnt = match NonZeroU64::new(pregen_creation_peers.0.len() as u64) {
                None => {
                    error!("No peers found in presign message store.");
                    return Ok(());
                }
                Some(c) => c,
            };
            let modulus = bigint::NonZero::<U256>::from_u64(peer_cnt);
            let idx = (U256::from_be_hex(&tag) % modulus).as_words()[0];
            let me = &peers.0[idx as usize];

            if me.socket_address == self.tss_state.addr {
                presign_list.add_storage_key(peer_group_id, tag);
            }

            self.current_generation_count[curve_type.index()] += 1; // technically this is the "loaded" amount right now.  But it will soon be reset by one of the leaders.
        }
        Ok(())
    }

    pub fn get_peer_group_id_from_xor_filter(
        &self,
        pregen_list: &PresignListByGroup,
        peers: &SimplePeerCollection,
        threshold: usize,
    ) -> u64 {
        let keys = peers.peer_keys();

        for (peer_group_id, xor_filter_with_threshold) in self.xor_filters.iter() {
            let mut found = true;
            if xor_filter_with_threshold.threshold == threshold {
                for key in &keys {
                    if !xor_filter_with_threshold.filter.contains(key) {
                        found = false;
                        break;
                    }
                }
                if found {
                    // even if there is a peer_group, we need to check if it's empty
                    if let Some(s) = pregen_list.get(peer_group_id)
                        && !s.is_empty()
                    {
                        info!(
                            "Found peer group_id {} with threshold {}.",
                            peer_group_id, xor_filter_with_threshold.threshold
                        );
                        return *peer_group_id;
                    }
                }
            }
        }
        0
    }

    pub async fn node_socket_addresses_from_peer_group_id(
        &self,
        peer_group_id: u64,
        all_peers: &SimplePeerCollection,
    ) -> SimplePeerCollection {
        let mut addresses = Vec::with_capacity(all_peers.0.len());
        let xor_filter_with_threshold = match self.xor_filters.get(&peer_group_id) {
            Some(s) => s,
            None => return addresses.into(),
        };

        for peer in &all_peers.0 {
            if xor_filter_with_threshold.filter.contains(&peer.key_hash) {
                addresses.push(peer.clone());
            }
        }
        addresses.into()
    }
}

pub async fn staker_hashes_from_peer_group_id(
    peer_group_id: u64,
    xor_filters: HashMap<super::models::PeerGroupId, XorFilterWithThreshold>,
    all_peers: &Vec<SimplePeer>,
) -> Vec<u64> {
    let mut peer_keys = Vec::new();
    let xor_filter_with_threshold = match xor_filters.get(&peer_group_id) {
        Some(s) => s,
        None => return peer_keys,
    };

    for peer in all_peers {
        if xor_filter_with_threshold.filter.contains(&peer.key_hash) {
            peer_keys.push(peer.key_hash);
        }
    }
    peer_keys
}
