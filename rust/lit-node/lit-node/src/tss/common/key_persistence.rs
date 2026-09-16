use crate::common::key_helper::KeyCache;
use crate::error::{Result, unexpected_err};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::storage::{read_key_share_from_disk, write_key_share_to_disk};
use lit_node_core::{CompressedBytes, CompressedHex, CurveType, PeerId};
use lit_rust_crypto::group::{Group, GroupEncoding};
use std::fmt::Debug;

pub const RECOVERY_DKG_EPOCH: u64 = 0;

#[derive(Debug, Clone, Copy)]
pub struct KeyPersistence<G>
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: CompressedBytes,
{
    pub(crate) curve_type: CurveType,
    _group: std::marker::PhantomData<G>,
}

impl<G> KeyPersistence<G>
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: CompressedBytes,
{
    pub fn new(curve_type: CurveType) -> Self {
        Self {
            curve_type,
            _group: std::marker::PhantomData,
        }
    }

    pub fn secret_to_hex(&self, share: &G::Scalar) -> String {
        share.to_compressed_hex()
    }

    pub fn secret_to_bytes(&self, share: &G::Scalar) -> Vec<u8> {
        share.to_compressed()
    }

    pub fn secret_from_hex(&self, private_key: &str) -> Result<G::Scalar> {
        self.validate_scalar_len(&private_key.as_bytes()[..private_key.len() / 2])?;
        <G::Scalar as CompressedHex>::from_compressed_hex(private_key)
            .ok_or(unexpected_err("Failed to convert hex to private key", None))
    }

    pub fn secret_from_bytes(&self, private_key: &[u8]) -> Result<G::Scalar> {
        self.validate_scalar_len(private_key)?;
        <G::Scalar as CompressedBytes>::from_compressed(private_key).ok_or(unexpected_err(
            "Failed to convert bytes to private key",
            None,
        ))
    }

    pub fn pk_to_hex(&self, public_key: &G) -> String {
        public_key.to_compressed_hex()
    }

    pub fn pk_to_bytes(&self, public_key: &G) -> Vec<u8> {
        public_key.to_compressed()
    }

    pub fn pk_from_hex(&self, public_key: &str) -> Result<G> {
        self.validate_pk_len(&public_key.as_bytes()[..public_key.len() / 2])?;
        <G as CompressedHex>::from_compressed_hex(public_key)
            .ok_or(unexpected_err("Failed to convert hex to public key", None))
    }

    pub fn pk_from_uncompressed_hex(&self, public_key: &str) -> Result<G> {
        self.validate_pk_len(public_key.as_bytes())?;
        <G as CompressedHex>::from_uncompressed_hex(public_key).ok_or(unexpected_err(
            "Failed to convert uncompressed hex to public key",
            None,
        ))
    }

    pub fn pk_to_uncompressed_hex(&self, public_key: &G) -> String {
        public_key.to_uncompressed_hex()
    }

    pub fn pk_from_bytes(&self, public_key: &[u8]) -> Result<G> {
        self.validate_pk_len(public_key)?;
        <G as CompressedBytes>::from_compressed(public_key).ok_or(unexpected_err(
            "Failed to convert bytes to public key",
            None,
        ))
    }

    pub fn validate_pk_len(&self, public_key: &[u8]) -> Result<()> {
        if public_key.len() != self.curve_type.compressed_point_len() {
            return Err(unexpected_err(
                format!(
                    "Invalid compressed public key length. Expected {} length: {}, actual length: {}",
                    self.curve_type,
                    self.curve_type.compressed_point_len(),
                    public_key.len()
                ),
                None,
            ));
        }
        Ok(())
    }

    pub fn validate_scalar_len(&self, private_key: &[u8]) -> Result<()> {
        if private_key.len() != self.curve_type.scalar_len() {
            return Err(unexpected_err(
                "Invalid private key length",
                Some(format!(
                    "Expected {} length: {}, actual length: {}",
                    self.curve_type,
                    self.curve_type.scalar_len(),
                    private_key.len()
                )),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn write_key(
        &self,
        pubkey: Option<String>,
        pk: G,
        share: G::Scalar,
        peer_id: &PeerId,
        dkg_id: &str,
        epoch: u64,
        peers: &SimplePeerCollection,
        staker_address: &str,
        realm_id: u64,
        threshold: usize,
        key_cache: &KeyCache,
    ) -> Result<String> {
        let total_shares = peers.0.len();

        let key_share = KeyShare::new(
            share,
            pk,
            self.curve_type,
            peer_id,
            peers,
            threshold,
            total_shares,
            dkg_id.to_string(),
            realm_id,
        )?;

        // because refreshing return 0x0 as a result, we need to check for the existence of a passed public key
        // and use this value as the PK parameter and file name.
        let hex_pubkey = match pubkey {
            Some(pubkey) => pubkey,
            None => key_share.hex_public_key.clone(),
        };

        write_key_share_to_disk::<KeyShare>(
            self.curve_type,
            &key_share.hex_public_key,
            staker_address,
            peer_id,
            epoch,
            realm_id,
            key_cache,
            &key_share,
        )
        .await?;

        Ok(hex_pubkey)
    }

    pub async fn read_key(
        &self,
        pubkey: &str,
        peer_id: &PeerId,
        epoch: u64,
        staker_address: &str,
        realm_id: u64,
        key_cache: &KeyCache,
    ) -> Result<Option<(G::Scalar, G)>> {
        let key_share = read_key_share_from_disk::<KeyShare>(
            self.curve_type,
            pubkey,
            staker_address,
            peer_id,
            epoch,
            realm_id,
            key_cache,
        )
        .await?;

        let secret_share = self.secret_from_hex(&key_share.hex_private_share)?;
        let public_key = self.pk_from_hex(&key_share.hex_public_key)?;

        Ok(Some((secret_share, public_key)))
    }
}
