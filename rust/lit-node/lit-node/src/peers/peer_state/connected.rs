use super::super::{
    PeerState,
    peer_item::{PeerData, PeerItem},
    peer_reviewer::{Issue, PeerComplaint},
};
use std::{sync::Arc, time::Duration};

use ethers::types::{Address, U256};
use libsecp256k1::PublicKey;
use lit_attestation::{AttestationType, attestation::FromSystem};
use lit_node_common::config::{CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT, LitNodeConfig};
use rand::{Rng, rngs::OsRng};
use tonic::{Code, Request};
use tracing::{Instrument, info_span, warn};
use url::Url;

use crate::p2p_comms::web::chatter_server::chatter::ConnectRequest;
use crate::utils::networking::get_web_addr_from_chain_info;
use crate::version;
use crate::{
    error::{EC, Result, blockchain_err_code, unexpected_err},
    networking::grpc::client::ChatterClientFactory,
};
use crate::{models::PeerValidator, peers::peer_state::models::NetworkState};
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;

use super::models::PeerValidatorStatus;
use crate::utils::key_share_proof::KeyShareProofs;
use lit_attestation::verification::Policy;
use lit_node_core::CompressedBytes;
use lit_node_core::PeerId;
use tokio::task::JoinSet;

#[allow(dead_code)]
impl PeerState {
    // ############# Functions to read and alter the connected peers (and struct)
    pub async fn find_peers(self: &Arc<Self>, peers: Vec<PeerValidator>) -> Result<()> {
        self.find_peers_ext(peers, false).await
    }

    pub async fn find_peers_ext(
        self: &Arc<Self>,
        peers: Vec<PeerValidator>,
        is_union: bool,
    ) -> Result<()> {
        let mut futures = JoinSet::new();
        for peer in peers.into_iter() {
            futures.spawn(self.clone().connect_to_node(peer));
        }

        let mut data = PeerData::clone(&self.data.load());
        while let Some(node_info) = futures.join_next().await {
            let node_info = match node_info {
                Ok(node_info) => match node_info {
                    Ok(node) => node,
                    Err(e) => {
                        warn!("Error connecting to peer: {:?}", e.msg());
                        trace!("Details of error connecting to peer: {:?}", e);
                        continue;
                    }
                },
                Err(e) => {
                    warn!("Error running task to connect to peer: {:?}", e);
                    continue;
                }
            };

            // to believe the node (public key is not on contract, but addr and staker addr is)
            let pi = PeerItem::new(
                &node_info.addr,
                node_info.public_key,
                node_info.node_address,
                node_info.sender_public_key,
                node_info.receiver_public_key,
                node_info.staker_address,
                PeerValidatorStatus::Unknown,
                None,
                node_info.version,
                KeyShareProofs::default(),
            );
            data.insert(pi)
                .expect_or_err("failed to insert into PeerItem")?;
        }

        data.table
            .sort_by(|a, b| a.staker_address.cmp(&b.staker_address)); // keep it sorted

        if is_union {
            self.union_data.store(Arc::new(data.clone()));
        }
        self.data.store(Arc::new(data));

        Ok(())
    }

    pub async fn connect_to_node(self: Arc<Self>, peer: PeerValidator) -> Result<PeerItem> {
        // hang on, are we trying to connect to ourselves?
        // if so, let's just not do that.  let's just load up our own nodeinfo.

        let addr_string = get_web_addr_from_chain_info(peer.ip, peer.port);
        let addr = addr_string.as_str();
        let peer_item;
        if addr == self.addr {
            let key_as_bytes = self.wallet_keys.verifying_key().to_uncompressed();
            let key_as_bytes: &[u8; 65] = (&key_as_bytes[..]).try_into().map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!(
                        "Could not convert key_as_bytes into length 65.  It's length is {}",
                        key_as_bytes.len()
                    )),
                )
            })?;
            let public_key = PublicKey::parse(key_as_bytes).map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!(
                        "Failed to convert VerifyingKey to PublicKey for node {}",
                        self.addr
                    )),
                )
            })?;
            peer_item = PeerItem {
                id: self.peer_id,
                public_key,
                node_address: self.node_address,
                sender_public_key: self.comskeys.sender_public_key().to_bytes(),
                receiver_public_key: self.comskeys.receiver_public_key().to_bytes(),
                staker_address: self.staker_address,
                addr: self.addr.clone(),
                status: PeerValidatorStatus::Unknown,
                attestation: None,
                version: version::get_version().to_string(),
            }
        } else {
            let cfg = self.lit_config.load_full();
            let noonce_bytes = OsRng.r#gen::<[u8; 32]>();
            let noonce = hex::encode(noonce_bytes);
            let timeout = match cfg.chatter_client_timeout() {
                Ok(t) => Duration::from_secs(t),
                Err(e) => {
                    warn!("Failed to get chatter client timeout: {:?}", e);
                    Duration::from_secs(CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT as u64)
                }
            };
            let prefix = "http://";
            trace!(
                "Attempting to use grpc client connection to node from peer state w/timeout: {:?}",
                timeout
            );
            let mut request = Request::new(ConnectRequest {
                noonce: noonce.clone(),
            });
            request.set_timeout(timeout);

            let body;

            loop {
                match self.client_grpc_channels.get_connection(addr).await {
                    Some(mut client) => {
                        trace!("Reusing existing grpc client connection at {}", addr);

                        body = match client
                            .peer_connect(request)
                            .instrument(info_span!("grpc_connect"))
                            .await
                        {
                            Ok(resp) => resp,
                            Err(e) => {
                                error!(
                                    "Sending connect request to peer {:?} has failed, {:?}",
                                    &addr, e
                                );
                                match e.code() {
                                    Code::Cancelled
                                    | Code::DeadlineExceeded
                                    | Code::Unavailable => {
                                        self.client_grpc_channels.remove_connection(addr).await;
                                        warn!("Peer {:?} is unresponsive. Complaining.", addr);
                                        let complainer = self.addr.clone();
                                        let complaint_channel = self.complaint_channel.clone();
                                        if let Err(e) = complaint_channel
                                            .send_async(PeerComplaint {
                                                complainer,
                                                issue: Issue::Unresponsive,
                                                peer_node_staker_address: peer.staker_address,
                                                peer_node_socket_address: peer.socket_addr.clone(),
                                            })
                                            .await
                                        {
                                            error!("Failed to send complaint to channel: {:?}", e);
                                        }
                                    }
                                    _ => {}
                                }

                                return Err(unexpected_err(
                                    e,
                                    Some(format!(
                                        "Failed to send connect request to peer:{}",
                                        addr
                                    )),
                                ));
                            }
                        };
                        break;
                    }
                    None => {
                        let dest_url = Url::parse(format!("{}{}/", prefix, addr).as_str())
                            .expect("Failed to parse URL");
                        trace!("Creating a new grpc client connection at {}", addr);
                        match ChatterClientFactory::new_client(dest_url, cfg.clone()).await {
                            Ok(c) => self.client_grpc_channels.add_connection(addr, c).await,
                            Err(e) => {
                                let state = self.network_state(peer.realm_id.as_u64()).await?;
                                if state == NetworkState::Paused {
                                    return Err(unexpected_err(
                                        e,
                                        Some(format!(
                                            "Failed to connect to peer while network is paused ( no complaining ) : {}",
                                            addr
                                        )),
                                    ));
                                }

                                trace!("Connecting to peer {:?} has failed, {:?}", &addr, e);
                                warn!("Peer {:?} is unresponsive. Complaining.", addr);
                                let complainer = self.addr.clone();
                                let complaint_channel = self.complaint_channel.clone();
                                if let Err(e) = complaint_channel
                                    .send_async(PeerComplaint {
                                        complainer,
                                        issue: Issue::Unresponsive,
                                        peer_node_staker_address: peer.staker_address,
                                        peer_node_socket_address: peer.socket_addr.clone(),
                                    })
                                    .await
                                {
                                    error!("Failed to send complaint to channel: {:?}", e);
                                }
                                return Err(unexpected_err(
                                    e,
                                    Some(format!("Failed to connect to peer: {}", addr)),
                                ));
                            }
                        };
                    }
                }
            }

            let peer_item_str = body.into_inner().peer_item;
            peer_item = serde_json::from_str::<PeerItem>(&peer_item_str)
                .map_err(|e| unexpected_err(e, Some("Failed to deserialize PeerItem".into())))?;

            // verify NodeInfo against node information that is registered in the Staking contract
            // upon registration.
            let verify_res = self
                .verify_peer_item(
                    peer_item.node_address,
                    &peer,
                    peer_item.clone(),
                    noonce_bytes.as_slice(),
                    addr,
                )
                .await;
            if let Err(verify_err) = verify_res {
                // If the error is EC::NodeRpcError, log error and rethrow Error without complaining Peer.
                // Rethrowing Error will cause this code path to be run again at some later time by the caller.
                return if verify_err.is_code(EC::NodeRpcError, true)
                    || verify_err
                        .is_code(lit_attestation::error::EC::AttestationRequestFailed, true)
                {
                    error!("Error verifying node info: {:?}", verify_err);
                    Err(verify_err)
                } else {
                    let err_msg: &str = "Node provided incorrect info.";
                    warn!(
                        "{:?}: {:?}. Err: {:?}. Complaining.",
                        err_msg, addr, verify_err
                    );
                    let complainer = self.addr.clone();
                    let complaint_channel = self.complaint_channel.clone();
                    if let Err(e) = complaint_channel
                        .send_async(PeerComplaint {
                            complainer,
                            issue: Issue::IncorrectInfo,
                            peer_node_staker_address: peer.staker_address,
                            peer_node_socket_address: peer.socket_addr.clone(),
                        })
                        .await
                    {
                        error!("Failed to send complaint to channel: {:?}", e);
                    }
                    Err(unexpected_err(err_msg, None))
                };
            }
        }

        Ok(peer_item)
    }

    async fn verify_peer_item(
        &self,
        registered_node_address: Address,
        validator: &PeerValidator,
        peer_item_to_verify: PeerItem,
        noonce: &[u8],
        address_we_talked_to: &str,
    ) -> Result<()> {
        // first, get relevant info from chain
        let registered_staker_address = self
            .staking_contract
            .node_address_to_staker_address_across_realms(registered_node_address)
            .call()
            .await
            .map_err(|e| blockchain_err_code(e, EC::NodeRpcError, None))?;

        // verify node address
        if peer_item_to_verify.node_address != registered_node_address {
            return Err(unexpected_err(
                format!(
                    "node_address different from chain.  Peer item node_address: {:?}, chain node_address: {:?}",
                    peer_item_to_verify.node_address, registered_node_address
                ),
                None,
            ));
        }

        // verify web address
        if peer_item_to_verify.addr != get_web_addr_from_chain_info(validator.ip, validator.port) {
            return Err(unexpected_err(
                format!(
                    "addr different from chain.  Peer item addr: {:?}, chain addr: {:?}",
                    peer_item_to_verify.addr,
                    get_web_addr_from_chain_info(validator.ip, validator.port)
                ),
                None,
            ));
        }

        // verify communication keys
        if U256::from_big_endian(&peer_item_to_verify.sender_public_key)
            != validator.coms_sender_pub_key
        {
            return Err(unexpected_err(
                format!(
                    "sender_pubkey different from chain.  Peer item sender_pubkey little endian: {:?}, Peer item sender_pubkey big endian: {:?}, chain sender_pubkey: {:?}",
                    U256::from_little_endian(&peer_item_to_verify.sender_public_key),
                    U256::from_big_endian(&peer_item_to_verify.sender_public_key),
                    validator.coms_sender_pub_key
                ),
                None,
            ));
        }

        if U256::from_big_endian(&peer_item_to_verify.receiver_public_key)
            != validator.coms_receiver_pub_key
        {
            return Err(unexpected_err(
                format!(
                    "receiver_pubkey different from chain.  Peer item receiver_pubkey little endian: {:?}, Local receiver_pubkey big endian: {:?}, chain receiver_pubkey: {:?}",
                    U256::from_little_endian(&peer_item_to_verify.receiver_public_key),
                    U256::from_big_endian(&peer_item_to_verify.receiver_public_key),
                    validator.coms_receiver_pub_key
                ),
                None,
            ));
        }

        // verify staker address
        if peer_item_to_verify.staker_address != registered_staker_address {
            tracing::debug!("Peer item: {:?}", peer_item_to_verify);
            tracing::debug!("Validator from chain: {:?}", validator);

            return Err(unexpected_err(
                format!(
                    "staker_address different from chain.  Peer item staker_address: {:?}, chain staker_address: {:?}",
                    peer_item_to_verify.staker_address, registered_staker_address
                ),
                None,
            ));
        }

        let peer_id = PeerId::from_slice(&validator.wallet_public_key)
            .map_err(|e| unexpected_err(e, None))?;
        if peer_id.is_not_assigned() {
            return Err(unexpected_err(
                format!(
                    "Peer ID not assigned for peer id: {:?}, peer staker_address: {:?}",
                    peer_item_to_verify.id, peer_item_to_verify.staker_address
                ),
                None,
            ));
        }

        async fn check_attestation<N: AsRef<[u8]>>(
            lit_config: Arc<LitConfig>,
            peer_item_to_verify: &PeerItem,
            noonce: N,
            address_we_talked_to: &str,
        ) -> Result<()> {
            let attestation = peer_item_to_verify.attestation.as_ref().ok_or_else(|| {
                unexpected_err(
                    format!(
                        "empty attestation from peer. peer's node_address: {:?}",
                        peer_item_to_verify.node_address,
                    ),
                    None,
                )
            })?;

            let cfg = lit_config.as_ref();
            attestation
                .verify_full(cfg, None, Some(Policy::NodeConnect))
                .await
                .map_err(|e| {
                    unexpected_err(
                        e,
                        Some(format!(
                            "invalid attestation from peer. peer's node_address: {:?}",
                            peer_item_to_verify.node_address,
                        )),
                    )
                })?;

            if attestation.noonce().as_slice() != noonce.as_ref() {
                return Err(unexpected_err(
                    format!(
                        "invalid attestation from peer; incorrect noonce. peer's node_address: {:?}",
                        peer_item_to_verify.node_address,
                    ),
                    None,
                ));
            }

            // also check ip address and port from attestation report
            let split_addr: Vec<&str> = address_we_talked_to.split(':').collect();
            let ip_address_we_talked_to = split_addr[0];
            let port_we_talked_to = split_addr[1];

            let external_addr_from_report = attestation
                .external_addr()
                .expect("Could not get external_addr from attestation report");
            let split_addr_from_report: Vec<&str> = external_addr_from_report.split(':').collect();
            let ip_address_from_report = split_addr_from_report[0];
            let port_from_report = split_addr_from_report[1];

            if ip_address_from_report != ip_address_we_talked_to {
                return Err(unexpected_err(
                    format!(
                        "invalid attestation from peer; incorrect ip address. peer's node_address: {:?}, ip address we talked to: {}, ip address from report: {}",
                        peer_item_to_verify.node_address,
                        ip_address_we_talked_to,
                        ip_address_from_report,
                    ),
                    None,
                ));
            }

            if port_from_report != port_we_talked_to {
                return Err(unexpected_err(
                    format!(
                        "invalid attestation from peer; incorrect port. peer's node_address: {:?}, port we talked to: {}, port from report: {}",
                        peer_item_to_verify.node_address, port_we_talked_to, port_from_report,
                    ),
                    None,
                ));
            }

            Ok(())
        }

        // verify attestation (optional, except in prod)
        let lcfg = self.lit_config.load_full();
        if *lcfg.env() == lit_core::config::envs::LitEnv::Prod
            || AttestationType::from_system().is_some()
        {
            check_attestation(lcfg, &peer_item_to_verify, noonce, address_we_talked_to).await?
        } else {
            #[cfg(not(feature = "testing"))]
            warn!("skipping attestation check because attestation type is not known or is none");
        }

        Ok(())
    }

    pub fn get_peer_item_from_addr(&self, peer_addr: &str) -> Result<PeerItem> {
        let peer_state_data = self.union_data.load();
        peer_state_data
            .get_peer_by_addr(peer_addr)
            .expect_or_err(format!("PeerItem not found for peer_addr: {}", peer_addr))
    }

    pub fn get_peer_item_from_staker_addr(&self, staker_address: Address) -> Result<PeerItem> {
        let peer_state_data = self.union_data.load();
        peer_state_data
            .get_peer_by_staker_addr(staker_address)
            .expect_or_err(format!(
                "PeerItem not found for staker address: {}",
                staker_address
            ))
    }

    pub fn connected_nodes(&self) -> Result<Vec<PeerItem>> {
        let peer_data = self.data.load();
        let peer_items = &peer_data.table;

        Ok(peer_items.clone())
    }

    pub fn curr_connected_nodes(&self) -> Result<Vec<PeerItem>> {
        let peer_data = self.curr_data.load();
        let peer_items = &peer_data.table;

        Ok(peer_items.clone())
    }

    // Replaces existing curr_data with PeerData of target_peers
    pub fn set_curr_data_peers(&self, target_peers: Vec<PeerValidator>) -> Result<()> {
        let mut curr_data = PeerData::clone(&self.curr_data.load());
        curr_data.clear_table();
        self.append_curr_data_peers(target_peers)?;

        Ok(())
    }

    // Appends to existing curr_data with PeerData of peer_addresses
    pub fn append_curr_data_peers(&self, peer_addresses: Vec<PeerValidator>) -> Result<()> {
        let mut curr_data = PeerData::clone(&self.curr_data.load());
        let data = PeerData::clone(&self.data.load());
        for peer in &self.data.load().table {
            for validator in &peer_addresses {
                if validator.address == peer.node_address {
                    curr_data
                        .insert(peer.clone())
                        .expect_or_err("failed to insert into PeerItem")?;
                }
            }
        }
        curr_data
            .table
            .sort_by(|a, b| a.staker_address.cmp(&b.staker_address));
        self.curr_data.store(Arc::new(curr_data));

        Ok(())
    }

    pub fn connected_node_addrs(&self) -> Result<Vec<String>> {
        let peer_data = self.data.load();
        // collect all addr
        let addrs = peer_data.table.iter().map(|pi| pi.addr.clone()).collect();
        Ok(addrs)
    }

    pub async fn get_peer_staker_address_for_complain(&self, addr: &str) -> Result<Address> {
        let peer_item = self.get_peer_item_from_addr(addr);
        if let Ok(peer_item) = peer_item {
            return Ok(peer_item.staker_address);
        }

        debug!(
            "Failed to get peer item from addr: {:?}, trying from chain data",
            addr
        );

        self.get_staker_address_from_socket_addr(addr).map_err(|e| {
            unexpected_err(
                e,
                Some("Failed to get peer staker address from chain".into()),
            )
        })
    }
}
