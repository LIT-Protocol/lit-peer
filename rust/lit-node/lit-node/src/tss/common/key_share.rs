use crate::error::{Result, parser_err};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::key_persistence::KeyPersistence;
use lit_node_core::{CompressedBytes, CurveType, PeerId};
use lit_rust_crypto::{
    group::{Group, GroupEncoding},
    vsss_rs::{DefaultShare, IdentifierPrimeField},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyShare {
    pub hex_private_share: String,
    pub hex_public_key: String,
    pub curve_type: CurveType,
    pub peer_id: PeerId,
    pub threshold: usize,
    pub total_shares: usize,
    pub txn_prefix: String,
    #[serde(default = "default_realm_id")]
    pub realm_id: u64,
    /// All other peers that participated in the DKG of this share
    pub peers: Vec<PeerId>,
}

// This allows backwards compatibility with old shares that did not have a realm_id
fn default_realm_id() -> u64 {
    1
}

impl KeyShare {
    #[allow(clippy::too_many_arguments)]
    pub fn new<G>(
        secret: <G as Group>::Scalar,
        public_key: G,
        curve_type: CurveType,
        peer_id: &PeerId,
        peers: &SimplePeerCollection,
        threshold: usize,
        total_shares: usize,
        txn_prefix: String,
        realm_id: u64,
    ) -> Result<Self>
    where
        G: Group + GroupEncoding + Default + CompressedBytes,
        G::Scalar: CompressedBytes,
    {
        let persistent = KeyPersistence::<G>::new(curve_type);
        let hex_private_share = persistent.secret_to_hex(&secret);
        let hex_public_key = persistent.pk_to_hex(&public_key);

        Ok(KeyShare {
            hex_private_share,
            hex_public_key,
            curve_type,
            peer_id: *peer_id,
            threshold,
            total_shares,
            txn_prefix,
            realm_id,
            peers: peers.peer_ids(),
        })
    }

    pub fn secret<G>(&self) -> Result<G::Scalar>
    where
        G: Group + GroupEncoding + Default + CompressedBytes,
        G::Scalar: CompressedBytes,
    {
        KeyPersistence::<G>::new(self.curve_type).secret_from_hex(&self.hex_private_share)
    }

    pub fn public_key<G>(&self) -> Result<G>
    where
        G: Group + GroupEncoding + Default + CompressedBytes,
        G::Scalar: CompressedBytes,
    {
        KeyPersistence::<G>::new(self.curve_type).pk_from_hex(&self.hex_public_key)
    }

    pub fn secret_as_bytes(&self) -> Result<Vec<u8>> {
        hex::decode(&self.hex_private_share).map_err(|e| parser_err("Failed to decode hex", None))
    }

    pub fn public_key_as_bytes(&self) -> Result<Vec<u8>> {
        hex::decode(&self.hex_public_key).map_err(|e| parser_err("Failed to decode hex", None))
    }

    #[allow(clippy::type_complexity)]
    pub fn default_share<G>(
        &self,
    ) -> Result<DefaultShare<IdentifierPrimeField<G::Scalar>, IdentifierPrimeField<G::Scalar>>>
    where
        G: Group + GroupEncoding + Default + CompressedBytes,
        G::Scalar: CompressedBytes + From<PeerId>,
    {
        Ok(DefaultShare {
            identifier: IdentifierPrimeField(G::Scalar::from(self.peer_id)),
            value: IdentifierPrimeField(self.secret::<G>()?),
        })
    }
}
