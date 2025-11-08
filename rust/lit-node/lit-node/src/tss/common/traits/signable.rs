use crate::error::Result;
use lit_node_core::{NodeSet, SignableOutput};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Signable: Debug + Send + Sync {
    #[allow(clippy::too_many_arguments)]
    async fn sign_with_pubkey(
        &mut self,
        message_bytes: &[u8],
        public_key: Vec<u8>,
        root_pubkeys: Option<Vec<String>>,
        tweak_preimage: Option<Vec<u8>>,
        request_id: Vec<u8>,
        epoch: Option<u64>,
        nodeset: &[NodeSet],
    ) -> Result<SignableOutput>;

    fn failed_message_share(&self) -> SignableOutput {
        SignableOutput::ecdsa_failed_message_share()
    }
}
