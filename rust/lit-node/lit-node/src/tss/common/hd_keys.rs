use crate::common::key_helper::KeyCache;
use crate::tss::common::key_share::KeyShare;
use crate::{
    error::{Result, unexpected_err},
    tss::common::storage::read_key_share_from_disk,
};
use elliptic_curve::group::GroupEncoding;
use hd_keys_curves::{HDDerivable, HDDeriver};
use lit_node_core::CompressedBytes;
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use tracing::instrument;

pub const ID_SIGN_CTX: &[u8] = b"LIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_";

#[allow(clippy::too_many_arguments)]
#[instrument(level = "debug", skip_all)]
pub async fn get_derived_keyshare<G>(
    deriver: G::Scalar,
    hd_root_keys: &[String],
    curve_type: CurveType,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<(G::Scalar, G)>
where
    G: HDDerivable + GroupEncoding + Default + CompressedBytes,
    G::Scalar: HDDeriver + CompressedBytes,
{
    let mut hd_sk_shares: Vec<G::Scalar> = Vec::new();
    let mut hd_pk_shares: Vec<G> = Vec::new();

    for (i, hd_root_key) in hd_root_keys.iter().enumerate() {
        let keyshare = read_key_share_from_disk::<KeyShare>(
            curve_type,
            hd_root_key,
            staker_address,
            peer_id,
            epoch,
            realm_id,
            key_cache,
        )
        .await
        .map_err(|e| {
            unexpected_err(
                e,
                Some(format!(
                    "Could not read key share (index/epoch) {}/{} from disk",
                    peer_id, epoch,
                )),
            )
        })?;

        let sk = keyshare.secret::<G>()?;
        let pk = keyshare.public_key::<G>()?;

        hd_sk_shares.push(sk);
        hd_pk_shares.push(pk);
    }

    trace!("HD Key Shares length: {}", hd_pk_shares.len());

    let derived_pubkey = deriver.hd_derive_public_key(&hd_pk_shares);

    let derived_secret = deriver.hd_derive_secret_key(&hd_sk_shares);

    trace!("Computed secret share.");

    Ok((derived_secret, derived_pubkey))
}
