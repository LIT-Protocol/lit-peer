use crate::error::{EC, parser_err, unexpected_err, unexpected_err_code};
use crate::p2p_comms::CommsManager;
use crate::peers::peer_state::models::SimplePeer;
use crate::tss::common::hd_keys::get_derived_keyshare;
use crate::tss::common::traits::signable::Signable;
use crate::{
    error::Result,
    metrics,
    peers::peer_state::models::SimplePeerCollection,
    tss::common::{dkg_type::DkgType, tss_state::TssState},
};
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_frost::{
    Identifier, KeyPackage, Scheme, SignatureShare, SigningCommitments, SigningShare, VerifyingKey,
    VerifyingShare,
};
use lit_node_core::{
    CompressedBytes, CurveType, FrostSignedMessageShare, NodeSet, PeerId, SignableOutput,
    SigningAlgorithm, SigningScheme,
    hd_keys_curves_wasm::{HDDerivable, HDDeriver},
};
use lit_rust_crypto::{
    curve25519_dalek, decaf377, ed448_goldilocks, group::GroupEncoding, jubjub, k256, p256, p384,
    pallas, vsss_rs,
};
use lit_sdk::signature::signing_scheme_to_frost_scheme;
use std::{num::NonZeroU16, sync::Arc};
use verifiable_share_encryption::legacy_vsss_rs::ShareIdentifier;

#[derive(Debug, Clone)]
pub struct FrostState {
    pub state: Arc<TssState>,
    pub dkg_type: DkgType,
    pub signing_scheme: SigningScheme,
}

impl FrostState {
    pub fn new(state: Arc<TssState>, signing_scheme: SigningScheme) -> Self {
        Self::new_with_dkg_type(state, signing_scheme, DkgType::Standard)
    }

    pub fn new_with_dkg_type(
        state: Arc<TssState>,
        signing_scheme: SigningScheme,
        dkg_type: DkgType,
    ) -> Self {
        FrostState {
            state,
            dkg_type,
            signing_scheme,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn sign_internal(
        &self,
        txn_prefix: &str,
        peers: &SimplePeerCollection,
        message: &[u8],
        signature_scheme: SigningScheme,
        group_key: &VerifyingKey,
        secret_share: &SigningShare,
        threshold: usize,
    ) -> Result<(
        Identifier,
        SignatureShare,
        SigningCommitments,
        VerifyingShare,
    )> {
        if !signature_scheme.supports_algorithm(SigningAlgorithm::Schnorr) {
            let msg = format!(
                "Requested signature scheme {:?} does not support Schnorr",
                signature_scheme
            );
            return Err(unexpected_err_code(
                "Unsupported signature curve for Schnorr signature",
                EC::NodeSignatureNotSupported,
                Some(msg),
            ));
        }

        // setup communications
        let round = "frost1";
        let cm = CommsManager::new_with_peers(&self.state, txn_prefix, peers, round).await?;

        // setup signing protocol
        let mut rng = rand::rngs::OsRng;
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let scheme: Scheme = signing_scheme_to_frost_scheme(signature_scheme)
            .map_err(|e| unexpected_err(e, None))?;
        let identifier = self.peer_id_to_frost_identifier(self_peer.peer_id)?;

        let verifying_share = scheme.verifying_share(secret_share).map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("VerifyingShare::try_from".to_string()),
            )
        })?;

        // round1
        let (nonces, commitments) = scheme.signing_round1(secret_share, &mut rng).map_err(|e| {
            unexpected_err_code(e, EC::NodeUnknownError, Some("Signing Round 1".to_string()))
        })?;

        // exchange commitments
        let r_commitments = cm
            .broadcast_and_collect::<SigningCommitments, SigningCommitments>(commitments.clone())
            .await?;

        // store commitments & starting with ours!
        let mut signing_commitments = vec![(identifier.clone(), commitments.clone())];

        for (remote_peer_id, peer_commitments) in r_commitments {
            let remote_identifier = self.peer_id_to_frost_identifier(remote_peer_id)?;
            signing_commitments.push((remote_identifier, peer_commitments));
        }

        let threshold = match NonZeroU16::new(
            threshold
                .try_into()
                .map_err(|_| parser_err("Unable to convert threshold to 16-bit integer", None))?,
        ) {
            Some(threshold) => threshold,
            None => {
                return Err(unexpected_err_code(
                    "threshold must be greater than 0",
                    EC::NodeUnknownError,
                    Some("Signing Round 1".to_string()),
                ));
            }
        };
        // round 2
        let key_package = KeyPackage {
            identifier: identifier.clone(),
            secret_share: secret_share.clone(),
            verifying_key: group_key.clone(),
            threshold,
        };

        let signature_share = scheme
            .signing_round2(message, &signing_commitments, &nonces, &key_package)
            .map_err(|e| {
                unexpected_err_code(e, EC::NodeUnknownError, Some("Signing Round 2".to_string()))
            })?;

        Ok((identifier, signature_share, commitments, verifying_share))
    }

    async fn derive_frost_signing_components<G>(
        &self,
        deriver: G::Scalar,
        root_pubkeys: Option<Vec<String>>,
        self_peer: &SimplePeer,
        epoch: u64,
    ) -> Result<(VerifyingKey, SigningShare)>
    where
        G: HDDerivable + GroupEncoding + Default + CompressedBytes,
        G::Scalar: HDDeriver + CompressedBytes,
    {
        let root_pubkeys = root_pubkeys.expect_or_err("No root pubkeys provided!")?;
        let staker_address = &bytes_to_hex(self_peer.staker_address.as_bytes());
        let realm_id = self.state.peer_state.realm_id();

        let (sk, pk) = get_derived_keyshare::<G>(
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

        let scheme = signing_scheme_to_frost_scheme(self.signing_scheme)
            .map_err(|e| unexpected_err(e, None))?;
        let vk = VerifyingKey {
            scheme,
            value: pk.to_compressed(),
        };
        let signing_share = SigningShare {
            scheme,
            value: sk.to_compressed(),
        };
        Ok((vk, signing_share))
    }

    fn peer_id_to_frost_identifier(&self, peer_id: PeerId) -> Result<Identifier> {
        let bytes = match self.signing_scheme.curve_type() {
            CurveType::K256 => k256::Scalar::from(peer_id).to_bytes().to_vec(),
            CurveType::P256 => p256::Scalar::from(peer_id).to_bytes().to_vec(),
            CurveType::P384 => p384::Scalar::from(peer_id).to_bytes().to_vec(),
            CurveType::Ed25519 | CurveType::Ristretto25519 => {
                curve25519_dalek::Scalar::from(peer_id).to_bytes().to_vec()
            }
            CurveType::Ed448 => ed448_goldilocks::Scalar::from(peer_id)
                .to_bytes_rfc_8032()
                .to_vec(),
            CurveType::RedJubjub => jubjub::Scalar::from(peer_id).to_bytes().to_vec(),
            CurveType::RedDecaf377 => decaf377::Fr::from(peer_id).to_bytes().to_vec(),
            CurveType::RedPallas => pallas::Scalar::from(peer_id).to_le_bytes().to_vec(),
            _ => {
                // Shouldn't happen but just in case
                return Err(unexpected_err(
                    "Unsupported curve type for frost_identifier",
                    None,
                ));
            }
        };
        let scheme = signing_scheme_to_frost_scheme(self.signing_scheme)
            .map_err(|e| unexpected_err(e, None))?;
        Ok(Identifier { scheme, id: bytes })
    }
}

#[async_trait::async_trait]
impl Signable for FrostState {
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
        let peers = self.state.peer_state.peers();
        let signing_peers = peers.peers_for_nodeset(nodeset);
        let self_peer = peers.peer_at_address(&self.state.addr)?;
        let threshold = nodeset.len();
        let key_id = tweak_preimage.expect_or_err("No hd_key_id provided!")?;
        let realm_id = self.state.peer_state.realm_id();
        let epoch = epoch.unwrap_or(self.state.peer_state.epoch());
        let (vk, signing_share) = match self.signing_scheme {
            SigningScheme::SchnorrK256Sha256 | SigningScheme::SchnorrK256Taproot => {
                let deriver = k256::Scalar::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<k256::ProjectivePoint>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrP256Sha256 => {
                let deriver = p256::Scalar::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<p256::ProjectivePoint>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrP384Sha384 => {
                let deriver = p384::Scalar::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<p384::ProjectivePoint>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrEd25519Sha512 => {
                let deriver = vsss_rs::curve25519::WrappedScalar::create(
                    &key_id,
                    self.signing_scheme.id_sign_ctx(),
                );
                self.derive_frost_signing_components::<vsss_rs::curve25519::WrappedEdwards>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrRistretto25519Sha512 | SigningScheme::SchnorrkelSubstrate => {
                let deriver = vsss_rs::curve25519::WrappedScalar::create(
                    &key_id,
                    self.signing_scheme.id_sign_ctx(),
                );
                self.derive_frost_signing_components::<vsss_rs::curve25519::WrappedRistretto>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrEd448Shake256 => {
                let deriver =
                    ed448_goldilocks::Scalar::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<ed448_goldilocks::EdwardsPoint>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrRedJubjubBlake2b512 => {
                let deriver = jubjub::Scalar::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<jubjub::SubgroupPoint>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrRedDecaf377Blake2b512 => {
                let deriver = decaf377::Fr::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<decaf377::Element>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            SigningScheme::SchnorrRedPallasBlake2b512 => {
                let deriver = pallas::Scalar::create(&key_id, self.signing_scheme.id_sign_ctx());
                self.derive_frost_signing_components::<pallas::Point>(
                    deriver,
                    root_pubkeys,
                    &self_peer,
                    epoch,
                )
                .await?
            }
            _ => {
                return Err(unexpected_err(
                    format!("Unsupported schnorr type: {}", self.signing_scheme),
                    None,
                ));
            }
        };

        let df_sig_share = self
            .sign_internal(
                &txn_prefix,
                &signing_peers,
                message_bytes,
                self.signing_scheme,
                &vk,
                &signing_share,
                threshold,
            )
            .await;

        let (id, sig_share, commitments, vk_share) = match df_sig_share {
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

        Ok(FrostSignedMessageShare {
            message: hex::encode(message_bytes),
            result: "success".to_string(),
            peer_id: self_peer.peer_id.to_string(),
            share_id: serde_json::to_string(&id).expect_or_err("Error serializing share_id")?,
            signature_share: serde_json::to_string(&sig_share)
                .expect_or_err("Error serializing signature_share")?,
            signing_commitments: serde_json::to_string(&commitments)
                .expect_or_err("Error serializing signing_commitments")?,
            verifying_share: serde_json::to_string(&vk_share)
                .expect_or_err("Error serializing verifying_share")?,
            public_key: serde_json::to_string(&vk).expect_or_err("Error serializing public_key")?,
            sig_type: self.signing_scheme.to_string(),
        }
        .into())
    }
}
