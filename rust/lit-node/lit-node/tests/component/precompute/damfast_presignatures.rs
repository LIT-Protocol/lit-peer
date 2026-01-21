use super::super::utils::virtual_node_collection::VirtualNodeCollection;
use futures::future::join_all;
use lit_fast_ecdsa::SignatureShare;
use lit_node::peers::peer_state::models::SimplePeerCollection;
use lit_node::tasks::presign_manager::models::PreSignatureValue;
use lit_node_core::{CurveType, NodeSet, SigningScheme};
use lit_rust_crypto::{k256, p256, p384};
use tokio::task::JoinHandle;

#[tokio::test]
async fn test_generate_damfast_presignature() {
    crate::common::setup_logging();
    let num_nodes = 5;
    let vnc = VirtualNodeCollection::new(num_nodes).await;

    let txn_prefix = "test_dam_fast".to_string();
    generate_damfast_presignature(&vnc, txn_prefix, SigningScheme::EcdsaK256Sha256).await;
}

async fn generate_damfast_presignature(
    vnc: &VirtualNodeCollection,
    txn_prefix: String,
    signing_scheme: SigningScheme,
) -> bool {
    let mut v = Vec::new();

    let peers = vnc.peers();

    let threshold = peers.threshold_for_set_testing_only();
    for node in vnc.nodes.iter() {
        let damfast_state = node.damfast_state(signing_scheme).clone();
        let mut peers = peers.clone();
        let txn_prefix = txn_prefix.clone();
        let jh: JoinHandle<_> = tokio::spawn(async move {
            match signing_scheme {
                SigningScheme::EcdsaK256Sha256 => {
                    let r = damfast_state
                        .create_presignature_for_peers::<k256::Secp256k1>(
                            &txn_prefix,
                            &mut peers,
                            threshold,
                        )
                        .await
                        .map(|r| PreSignatureValue::K256(r));
                    r.expect("error from create presignature")
                }
                SigningScheme::EcdsaP256Sha256 => {
                    let r = damfast_state
                        .create_presignature_for_peers::<p256::NistP256>(
                            &txn_prefix,
                            &mut peers,
                            threshold,
                        )
                        .await
                        .map(|r| PreSignatureValue::P256(r));
                    r.expect("error from create presignature")
                }
                SigningScheme::EcdsaP384Sha384 => {
                    let r = damfast_state
                        .create_presignature_for_peers::<p384::NistP384>(
                            &txn_prefix,
                            &mut peers,
                            threshold,
                        )
                        .await
                        .map(|r| PreSignatureValue::P384(r));
                    r.expect("error from create presignature")
                }
                _ => panic!("Unsupported signing scheme"),
            }
        });
        v.push(jh);
    }

    let start = std::time::Instant::now();
    let r = join_all(v).await;

    let _sig_shares = r
        .iter()
        .map(|result| {
            let sig = result.as_ref().unwrap();
            (*sig).clone()
        })
        .collect::<Vec<_>>();

    info!("Time to generate presignatures: {:?}", start.elapsed());
    true
}

#[tokio::test]
#[ignore] // this doesn't work right now, because we assume there to be HD keys available.
async fn test_damfast_signature() {
    crate::common::setup_logging();
    let num_nodes = 5;
    let vnc = VirtualNodeCollection::new(num_nodes).await;

    damfast_signature(&vnc).await;
}

async fn damfast_signature(vnc: &VirtualNodeCollection) -> bool {
    let mut v = Vec::new();

    let current_peers = SimplePeerCollection::default();
    let _pubkey = super::super::dkg::dkg(&vnc, CurveType::K256, 0, None, &current_peers).await;

    let message_bytes = b"DamFast Test!";
    let root_pubkeys = None;
    let tweak_preimage = None;
    let request_id = b"damfasttxn";
    let epoch = Some(1);

    let node_set = vnc
        .peers()
        .0
        .iter()
        .map(|p| NodeSet {
            socket_address: p.socket_address.clone(),
            value: 1,
        })
        .collect::<Vec<_>>();

    for node in vnc.nodes.iter() {
        let mut damfast_state = node.damfast_state(SigningScheme::EcdsaK256Sha256).clone();
        let root_pubkeys = root_pubkeys.clone();
        let tweak_preimage = tweak_preimage.clone();
        let epoch = epoch.clone();
        let request_id = request_id.clone();
        let node_set = node_set.clone();
        let jh: JoinHandle<_> = tokio::spawn(async move {
            let r = damfast_state
                .sign_with_pubkey_internal::<k256::Secp256k1>(
                    message_bytes,
                    root_pubkeys,
                    tweak_preimage,
                    request_id.to_vec(),
                    epoch,
                    &node_set,
                )
                .await;
            r.expect("error from sign with pubkey internal")
        });
        v.push(jh);
    }

    let start = std::time::Instant::now();
    let r = join_all(v).await;

    let sig_shares = r
        .iter()
        .map(|result| {
            let sig = result.as_ref().unwrap();
            SignatureShare {
                s: serde_json::from_str(&sig.signature_share).unwrap(),
                r: serde_json::from_str::<k256::AffinePoint>(&sig.big_r)
                    .map(k256::ProjectivePoint::from)
                    .unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let _signature = SignatureShare::<k256::Secp256k1>::combine_into_signature(&sig_shares)
        .expect("Error combining signature shares");

    info!("Time to generate signature: {:?}", start.elapsed());
    true
}
