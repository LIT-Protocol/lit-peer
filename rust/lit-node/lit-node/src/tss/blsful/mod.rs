pub mod models;
use crate::error::{Result, unexpected_err};
use crate::tss::blsful::models::BlsState;
use crate::tss::common::hd_keys::get_derived_keyshare;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::traits::signable::Signable;
use crate::tss::common::{storage::read_key_share_from_disk, traits::cipherable::Cipherable};
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_core::{
    BlsSignedMessageShare, CurveType, NodeSet, PeerId, SignableOutput, SigningScheme,
    hd_keys_curves_wasm::HDDeriver,
};
use lit_rust_crypto::{
    blsful::{
        Bls12381G1Impl, Bls12381G2Impl, Pairing, SecretKeyShare, SignatureSchemes, SignatureShare,
        inner_types::{G1Projective, Scalar},
    },
    group::Group,
    vsss_rs::{IdentifierPrimeField, Share},
};
use tracing::instrument;

#[async_trait::async_trait]
impl Cipherable for BlsState {
    #[instrument(level = "debug", skip(self))]
    async fn sign(
        &self,
        message_bytes: &[u8],
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, PeerId)> {
        let dkg_state = self.state.get_dkg_state(CurveType::BLS)?;
        let root_keys = dkg_state.root_keys().await;
        if root_keys.is_empty() {
            return Err(unexpected_err(
                "No primary BLS key found!".to_string(),
                None,
            ));
        }

        self.sign_with_pubkey(message_bytes, &root_keys[0], epoch)
            .await
    }

    #[instrument(level = "debug", skip(self))]
    async fn sign_with_pubkey(
        &self,
        message_bytes: &[u8],
        pub_key: &str,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, PeerId)> {
        trace!(
            "Encryption signing with pubkey: {:?} for epoch: {:?}",
            pub_key, epoch
        );
        let (secret_key_share, share_peer_id) = self.get_keyshare(pub_key, epoch).await?;

        let sks = secret_key_share
            .sign(SignatureSchemes::ProofOfPossession, &message_bytes)
            .map_err(|e| unexpected_err(format!("Failed to sign message: {:?}", e), None))?;

        Ok((sks, share_peer_id))
    }
}
#[async_trait::async_trait]
impl Signable for BlsState {
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
        let txn_prefix = bytes_to_hex(&request_id);
        let epoch = epoch.unwrap_or(self.state.peer_state.epoch());
        let peers = self.state.peer_state.peers();

        let threshold = if epoch == self.state.peer_state.epoch() {
            self.state.get_threshold().await
        } else {
            // Could be the previous epoch, so we need to get the threshold for that epoch
            // tss_state.threshold is only for the current epoch
            self.state
                .get_threshold_using_current_epoch_realm_peers_for_curve(
                    &peers,
                    CurveType::BLS,
                    Some(epoch),
                )
                .await?
        };

        let signing_peers = peers.peers_for_nodeset(nodeset);

        if signing_peers.0.len() < threshold {
            return Err(unexpected_err(
                format!(
                    "Threshold mismatch: signing_peers {} < expected threshold {}",
                    signing_peers.0.len(),
                    threshold
                ),
                None,
            ));
        }
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let key_id = tweak_preimage.expect_or_err("No hd_key_id provided!")?;
        let realm_id = self.state.peer_state.realm_id();

        let dkg_state = self.state.get_dkg_state(CurveType::BLS12381G1)?;
        let root_keys = dkg_state.root_keys().await;
        if root_keys.is_empty() {
            return Err(unexpected_err(
                "No primary BLS key found!".to_string(),
                None,
            ));
        }
        let staker_address = &bytes_to_hex(self_peer.staker_address.as_bytes());
        let deriver = <Scalar as HDDeriver>::create(&key_id, self.signing_scheme.id_sign_ctx());
        match self.signing_scheme {
            SigningScheme::Bls12381G1ProofOfPossession => {
                let (sk, vk) = get_derived_keyshare::<G1Projective>(
                    deriver,
                    &root_keys,
                    CurveType::BLS12381G1,
                    staker_address,
                    &self_peer.peer_id,
                    epoch,
                    realm_id,
                    &self.state.key_cache,
                )
                .await?;

                let identifier = <<Bls12381G1Impl as Pairing>::PublicKey as Group>::Scalar::from(
                    self_peer.peer_id,
                );
                let secret_key_share = SecretKeyShare(
                    <Bls12381G2Impl as Pairing>::SecretKeyShare::with_identifier_and_value(
                        IdentifierPrimeField(identifier),
                        IdentifierPrimeField(sk),
                    ),
                );
                let signature_share: SignatureShare<Bls12381G2Impl> = secret_key_share
                    .sign(SignatureSchemes::ProofOfPossession, message_bytes)
                    .map_err(|e| {
                        unexpected_err(e, Some("unable to generate signature".to_string()))
                    })?;
                let verifying_share = secret_key_share.public_key().map_err(|e| {
                    unexpected_err(e, Some("unable to generate verifying share".to_string()))
                })?;
                Ok(BlsSignedMessageShare {
                    message: hex::encode(message_bytes),
                    result: "success".to_string(),
                    peer_id: self_peer.peer_id.to_string(),
                    share_id: serde_json::to_string(&Scalar::from(self_peer.peer_id))
                        .expect_or_err("Error serializing share_id")?,
                    signature_share: serde_json::to_string(&signature_share)
                        .expect_or_err("Error serializing signature_share")?,
                    verifying_share: serde_json::to_string(&verifying_share)
                        .expect_or_err("Error serializing verifying_share")?,
                    public_key: serde_json::to_string(&vk)
                        .expect_or_err("Error serializing public_key")?,
                    sig_type: self.signing_scheme.to_string(),
                }
                .into())
            }
            _ => Err(unexpected_err("Unsupported bls signature type.", None)),
        }
    }
}

impl BlsState {
    pub(crate) async fn get_keyshare(
        &self,
        pubkey: &str,
        epoch: Option<u64>,
    ) -> Result<(SecretKeyShare<Bls12381G2Impl>, PeerId)> {
        let realm_id = self.state.peer_state.realm_id();
        let self_epoch = self.state.peer_state.epoch();

        let epoch = match epoch {
            Some(e) => match e > self_epoch {
                true => {
                    warn!(
                        "Requested epoch is in the future. Using current epoch: {}",
                        self_epoch
                    );
                    self_epoch
                }
                false => e,
            },
            None => self_epoch,
        };

        let (epoch, peers) = match self_epoch - epoch {
            0 => (epoch, self.state.peer_state.peers()),
            1 => (epoch, self.state.peer_state.peers_in_prior_epoch()),
            _ => (self_epoch, self.state.peer_state.peers()),
        };

        let peer_id = peers.peer_id_by_address(&self.state.addr)?;

        let staker_address = &self.state.peer_state.hex_staker_address();
        let realm_id = self.state.peer_state.realm_id();
        let bls_key_share = read_key_share_from_disk::<KeyShare>(
            CurveType::BLS,
            pubkey,
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &self.state.key_cache,
        )
        .await?;

        let identifier =
            <<Bls12381G2Impl as Pairing>::PublicKey as Group>::Scalar::from(bls_key_share.peer_id);
        let value = bls_key_share.secret::<<Bls12381G2Impl as Pairing>::PublicKey>()?;

        let secret_key_share = SecretKeyShare(
            <Bls12381G2Impl as Pairing>::SecretKeyShare::with_identifier_and_value(
                IdentifierPrimeField(identifier),
                IdentifierPrimeField(value),
            ),
        );

        Ok((secret_key_share, peer_id))
    }
}
