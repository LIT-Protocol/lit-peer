use crate::error::Result; // EC , conversion_err_code
use blsful::{Bls12381G2Impl, SignatureShare};
use lit_node_core::PeerId;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Cipherable: Debug + Send + Sync {
    async fn sign(
        &self,
        message_bytes: &[u8],
        key_set_id: Option<&str>,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, PeerId)>;

    async fn sign_with_pubkey(
        &self,
        message_bytes: &[u8],
        public_key: &str,
        key_set_id: Option<&str>,
        epoch: Option<u64>,
    ) -> Result<(SignatureShare<Bls12381G2Impl>, PeerId)>;
}
