use crate::error::unexpected_err;
use crate::metrics;
use crate::p2p_comms::CommsManager;
use crate::tasks::presign_manager::models::{Presign, PresignMessage, PresignRequest};
use crate::tss::common::hd_keys::get_derived_keyshare;
use crate::{
    error::Result,
    peers::peer_state::models::SimplePeerCollection,
    tss::common::{dkg_type::DkgType, tss_state::TssState},
};
use elliptic_curve::{CurveArithmetic, FieldBytesSize, NonZeroScalar, PrimeCurve};
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_fast_ecdsa::{
    ParticipantList, PreSignature, PreSignatureParams, PreSignatureParticipant, RoundPayload,
    SignatureShare,
};
use lit_node_core::{EcdsaSignedMessageShare, NodeSet, SignableOutput};
use std::ops::Add;
use tracing::trace;

use super::common::traits::signable::Signable;
use crate::tasks::utils::generate_hash;
use crate::utils::traits::SignatureCurve;
use elliptic_curve::generic_array::ArrayLength;
use elliptic_curve::group::{Curve, GroupEncoding};
use hd_keys_curves::{HDDerivable, HDDeriver};
use k256::ecdsa::hazmat::DigestPrimitive;
use lit_node_core::PeerId;
use lit_node_core::SigningScheme;
use lit_node_core::{CompressedBytes, CompressedHex};
use serde::Serialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct DamFastState {
    pub state: Arc<TssState>,
    pub dkg_type: DkgType,
    pub signing_scheme: SigningScheme,
}

impl DamFastState {
    pub fn new(state: Arc<TssState>, signing_scheme: SigningScheme) -> Self {
        Self::new_with_dkg_type(state, signing_scheme, DkgType::Standard)
    }

    pub fn new_with_dkg_type(
        state: Arc<TssState>,
        signing_scheme: SigningScheme,
        dkg_type: DkgType,
    ) -> Self {
        DamFastState {
            state,
            signing_scheme,
            dkg_type,
        }
    }

    pub async fn create_presignature_for_peers<C>(
        &self,
        txn_prefix: &str,
        peers: &mut SimplePeerCollection,
        threshold: usize,
    ) -> Result<PreSignature<C>>
    where
        C: PrimeCurve + CurveArithmetic + DigestPrimitive,
        C::ProjectivePoint: GroupEncoding + HDDerivable + CompressedBytes,
        C::Scalar: HDDeriver + From<PeerId> + CompressedBytes,
        <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
    {
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let mut participants = Vec::with_capacity(peers.0.len());
        let mut self_peer_ordinal = 0;
        for (i, peer) in peers.0.iter().enumerate() {
            if self_peer.peer_id == peer.peer_id {
                self_peer_ordinal = i;
            }
            let id = C::Scalar::from(peer.peer_id);
            participants.push(id);
        }

        trace!("Participants [{}], {:?}", participants.len(), participants);

        let participant_list = ParticipantList::new(participants.as_slice())
            .map_err(|e| unexpected_err(e, Some("Error creating participant list".to_owned())))?;
        let params = PreSignatureParams {
            threshold,
            participant_list: participant_list.clone(),
        };

        trace!("Params: {:?}", params);
        let self_participant_id =
            Option::from(NonZeroScalar::<C>::new(participants[self_peer_ordinal])).ok_or(
                unexpected_err("Could not get self participant id".to_string(), None),
            )?;

        let mut self_participant = PreSignatureParticipant::<C>::new(&self_participant_id, &params)
            .map_err(|e| unexpected_err(e, Some("Error creating participant".to_owned())))?;

        trace!("Self Participant: {:?}", self_participant);

        for round_no in 1..4 {
            let participant_list = participant_list.clone();
            trace!(
                "Starting round {} for {} {}. Internal round: {:?}",
                round_no,
                txn_prefix,
                self_participant.threshold(),
                self_participant.round()
            );

            let cm =
                CommsManager::new_with_peers(&self.state, txn_prefix, peers, &round_no.to_string())
                    .await?;
            let start = std::time::Instant::now();

            let round_generator = self_participant
                .run()
                .map_err(|e| unexpected_err(e, Some("Error running round".to_owned())))?;

            let payloads = round_generator.iter().collect::<Vec<_>>();

            for payload in &payloads {
                // shouldn't happen, but just in case
                if payload.id.as_ref() == self_participant_id.as_ref() {
                    continue;
                }
                trace!("Sending from {} to {}", self_participant_id, payload.id);
                let dest_peer = &peers.0[payload.ordinal];

                match cm
                    .send_direct::<RoundPayload<C>>(dest_peer, payload.round_payload.clone())
                    .await
                {
                    Ok(_) => trace!(
                        "Successfully sent payload to peer {} for round {}",
                        dest_peer.peer_id, round_no
                    ),
                    Err(e) => error!(
                        "Failed to send payload to peer {} for round {}: {}",
                        dest_peer.peer_id, round_no, e
                    ),
                }
            }

            let round_data = cm.collect::<RoundPayload<C>>().await?;

            trace!(
                "Received all {} rounds of data for round {} participant {} in {:?} for {}.",
                round_data.len(),
                round_no,
                self_participant_id,
                start.elapsed(),
                txn_prefix,
            );
            for (id, payload) in round_data.iter() {
                match self_participant.receive(payload.clone()) {
                    Ok(_) => debug!(
                        "Successfully received payload from peer {} for round {}",
                        id, round_no
                    ),
                    Err(e) => error!(
                        "Failed to receive payload from peer {} for round {}: {}",
                        id, round_no, e
                    ),
                }
            }
        }

        let result = self_participant
            .run()
            .map_err(|e| unexpected_err(e, Some("Error running final round".to_owned())))?;

        let pre_sig: PreSignature<C> = result.output().expect("Error getting final presig.");
        debug!("Successfully generated presignature for {}.", txn_prefix);

        Ok(pre_sig)
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn get_presign(
        &self,
        message_bytes: &[u8],
        request_id: Vec<u8>,
        txn_prefix: &str,
        threshold: u16,
        peer_subset: SimplePeerCollection,
    ) -> Result<Option<Presign>> {
        let req = PresignRequest {
            message_bytes: message_bytes.to_vec(),
            request_id: request_id.clone(),
            txn_prefix: txn_prefix.to_string(),
            peers: peer_subset,
            signing_scheme: self.signing_scheme,
            threshold,
        };

        let (tx, rx) = flume::bounded(1);
        let msg = PresignMessage::RequestPresign(req, tx);
        let ps = self.state.peer_state.as_ref();
        ps.ps_tx.send_async(msg).await.map_err(|e| {
            unexpected_err(e, Some("Could not send request to presign manager".into()))
        })?;
        debug!("Sent request to presign manager for txn {}.", txn_prefix);

        let presign = rx.recv_async().await.map_err(|e| {
            unexpected_err(e, Some("Could not receive response from presign manager when requesting presign - Not enough presigns.".into()))
        })?;
        debug!("Got presign shares for {}.", txn_prefix);

        presign
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn sign_with_pubkey_internal<C>(
        &mut self,
        message_bytes: &[u8],
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
        epoch: Option<u64>,
        node_set: &[NodeSet],
    ) -> Result<EcdsaSignedMessageShare>
    where
        C: PrimeCurve + CurveArithmetic + DigestPrimitive + SignatureCurve,
        C::ProjectivePoint: GroupEncoding + HDDerivable + CompressedBytes,
        C::AffinePoint: Serialize,
        C::Scalar: HDDeriver + From<PeerId> + Serialize + CompressedBytes,
        <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
    {
        let peers = self.state.peer_state.peers();
        let signing_peers = peers.peers_for_nodeset(node_set);
        let threshold = node_set.len() as u16;

        let key_id = tweak_preimage.expect_or_err("No hd_key_id provided!")?;
        let txn_prefix = &bytes_to_hex(&request_id);

        // generate a presignature
        let presig = self
            .get_presign(
                message_bytes,
                request_id.clone(),
                txn_prefix,
                threshold,
                signing_peers.clone(),
            )
            .await?
            .expect_or_err("No presignature found!")?;

        debug!("Got presign for during signing: {}", txn_prefix);

        let (signature_share, pk, msg_digest, peer_id) = self
            .generate_signature_share_from_key_id::<C>(
                message_bytes,
                root_pubkeys,
                presig,
                &request_id,
                &signing_peers,
                &key_id,
            )
            .await?;
        debug!(
            "Successfully signed message with public key: {}",
            pk.to_compressed_hex(),
        );
        let self_peer = peers.peer_at_address(&self.state.addr)?;

        let signature_share = EcdsaSignedMessageShare {
            digest: hex::encode(message_bytes),
            result: "success".to_string(),
            peer_id: self_peer.peer_id.to_string(),
            share_id: serde_json::to_string(peer_id.as_ref())
                .expect_or_err("Error serializing share_id")?,
            signature_share: serde_json::to_string(&signature_share.s)
                .expect_or_err("Error serializing signature share")?,
            big_r: serde_json::to_string(&signature_share.r.to_affine())
                .expect_or_err("Error serializing big_r")?,
            compressed_public_key: format!("\"{}\"", pk.to_compressed_hex()),
            public_key: format!("\"{}\"", pk.to_uncompressed_hex()),
            sig_type: self.signing_scheme.to_string(),
        };

        Ok(signature_share)
    }

    pub async fn generate_signature_share_from_key_id<C>(
        &mut self,
        message_bytes: &[u8],
        root_pubkeys: Option<Vec<String>>,
        presig: Presign,
        request_id: &[u8],
        peers: &SimplePeerCollection,
        key_id: &[u8],
    ) -> Result<(
        SignatureShare<C>,  // the scalar share
        C::ProjectivePoint, // the public key that was derived from the tweak
        C::Scalar,
        NonZeroScalar<C>,
    )>
    where
        C: PrimeCurve + CurveArithmetic + DigestPrimitive + SignatureCurve,
        C::ProjectivePoint: GroupEncoding + HDDerivable + CompressedBytes,
        C::Scalar: HDDeriver + From<PeerId> + CompressedBytes,
        <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
    {
        let nonce = generate_hash(request_id).to_be_bytes();
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let staker_address = &bytes_to_hex(self_peer.staker_address.as_bytes());
        let realm_id = self.state.peer_state.realm_id();
        let epoch = self.state.peer_state.epoch();
        let deriver = C::Scalar::create(key_id, self.signing_scheme.id_sign_ctx());
        // participant list -> pulled from working presignature code.
        let mut participants = Vec::with_capacity(peers.0.len());
        for peer in peers.0.iter() {
            let id = C::Scalar::from(peer.peer_id);
            trace!(
                "signing with peer id: {:?} which maps to scalar: {:?}",
                peer.peer_id, id
            );
            participants.push(id);
        }
        debug!("Participants: {:?}", participants);
        let participant_list = ParticipantList::new(participants.as_slice())
            .map_err(|e| unexpected_err(e, Some("Error creating participant list".to_owned())))?;
        let root_pubkeys = root_pubkeys.expect_or_err("No root pubkeys provided!")?;

        let (sk, pk) = get_derived_keyshare::<C::ProjectivePoint>(
            deriver,
            &root_pubkeys,
            self.signing_scheme.curve_type(),
            staker_address,
            &self_peer.peer_id,
            epoch,
            realm_id,
            &self.state.key_cache,
        )
        .await?;

        debug!("Derived public key: {:?}", pk.to_compressed_hex());

        if message_bytes.len() != self.signing_scheme.ecdsa_message_len() {
            return Err(unexpected_err(
                format!(
                    "Message length to be signed is not {} bytes.  Please hash it before sending it to the node to sign.  You can use SHA256, Keccak256, SHA384 for example",
                    self.signing_scheme.ecdsa_message_len()
                ),
                None,
            ));
        }

        let scalar_primitive = elliptic_curve::ScalarPrimitive::<C>::from_slice(message_bytes)
            .map_err(|e| {
                unexpected_err(
                    e,
                    Some("Could not convert message to sign into ScalarPrimitive".into()),
                )
            })?;
        let msg_digest = C::Scalar::from(scalar_primitive);

        let peer_id = Option::<NonZeroScalar<C>>::from(NonZeroScalar::<C>::new(C::Scalar::from(
            self_peer.peer_id,
        )))
        .ok_or(unexpected_err("Could not convert peer id", None))?;
        let sig_share = SignatureShare::<C>::new_scalar(
            presig.share.unwrap::<C>(),
            &participant_list,
            nonce,
            msg_digest,
            &sk,
            &peer_id,
            &participant_list,
        );

        debug!("Signature share result: {:?}", sig_share);

        let sig_share = sig_share
            .map_err(|e| unexpected_err(e, Some("Error creating signature share".into())))?;
        Ok((sig_share, pk, msg_digest, peer_id))
    }
}

#[async_trait::async_trait]
impl Signable for DamFastState {
    #[doc = "Sign using a specifically identified public key.  This pubkey is the result of PKP generation."]
    #[instrument(level = "debug", skip_all)]
    async fn sign_with_pubkey(
        &mut self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
        epoch: Option<u64>,
        nodeset: &[NodeSet],
    ) -> Result<SignableOutput> {
        let txn_id = generate_hash(request_id.clone());

        let df_sig_share = match self.signing_scheme {
            SigningScheme::EcdsaK256Sha256 => {
                self.sign_with_pubkey_internal::<k256::Secp256k1>(
                    message_bytes,
                    root_pubkeys,
                    tweak_preimage,
                    request_id,
                    epoch,
                    nodeset,
                )
                .await
            }
            SigningScheme::EcdsaP256Sha256 => {
                self.sign_with_pubkey_internal::<p256::NistP256>(
                    message_bytes,
                    root_pubkeys,
                    tweak_preimage,
                    request_id,
                    epoch,
                    nodeset,
                )
                .await
            }
            SigningScheme::EcdsaP384Sha384 => {
                self.sign_with_pubkey_internal::<p384::NistP384>(
                    message_bytes,
                    root_pubkeys,
                    tweak_preimage,
                    request_id,
                    epoch,
                    nodeset,
                )
                .await
            }
            _ => Err(unexpected_err(
                format!("Unsupported signing scheme: {}", self.signing_scheme),
                None,
            )),
        };

        let sig_share = match df_sig_share {
            Ok(share) => {
                metrics::counter::add_one(metrics::tss::TssMetrics::SignatureShare, &[]);
                share
            }
            Err(e) => {
                metrics::counter::add_one(metrics::tss::TssMetrics::SignatureShareFail, &[]);
                error!("Error signing message: {:?}", e);
                return Err(e);
            }
        };

        Ok(sig_share.into())
    }
}
