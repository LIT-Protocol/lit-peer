use crate::*;
use lit_rust_crypto::{
    ed448_goldilocks::{Ed448, EdwardsPoint, Scalar, WideScalarBytes},
    group::{Group, GroupEncoding, cofactor::CofactorGroup},
    hash2curve::ExpandMsgXof,
};
use sha3::digest::{ExtendableOutput, ExtendableOutputReset, Update, XofReader};

/// Use 0x06 as the suite string
const SUITE_STRING: u8 = 0x06;

impl Handler for Ed448 {
    type Group = EdwardsPoint;

    fn is_weak(point: Self::Group) -> bool {
        let pt = point;
        (!pt.is_torsion_free() | pt.is_identity() | pt.is_small_order()).into()
    }

    fn clear_cofactor(point: Self::Group) -> Self::Group {
        if point.is_torsion_free().into() { point } else { point.clear_cofactor() }
    }
}

impl HashToCurve for Ed448 {
    fn hash_to_curve(msg: &Scalar) -> EdwardsPoint {
        const DST: &[u8] = b"ECVRF-EDWARDS448-SHAKE256-ELL2_RO_\x06";
        let bytes = msg.to_bytes();
        EdwardsPoint::hash::<ExpandMsgXof<sha3::Shake256>>(&bytes, DST)
    }
}

impl NonceGeneration for Ed448 {
    fn generate_nonce(sk: &Scalar, alpha: &Scalar) -> Scalar {
        // RFC 8032, section 5.1.6

        let mut hasher = sha3::Shake256::default();
        hasher.update(&sk.to_bytes_rfc_8032());
        let output = hasher.finalize_boxed_reset(64);
        hasher.update(&output[32..]);
        hasher.update(&alpha.to_bytes_rfc_8032());
        let mut bytes = WideScalarBytes::default();
        hasher.finalize_xof().read(&mut bytes);
        Scalar::from_bytes_mod_order_wide(&bytes)
    }
}

impl ChallengeGeneration for Ed448 {
    fn generate_challenge(points: &[EdwardsPoint]) -> Scalar {
        const DST: &[u8] = b"ECVRF-EDWARDS448-SHAKE256-RO_GENERATE_CHALLENGE_";
        let mut hasher = sha3::Shake256::default();
        hasher.update(DST);
        // Suite string
        hasher.update(&[SUITE_STRING]);
        // challenge_generation_domain_separator_front
        hasher.update(&[0x02]);
        for point in points {
            hasher.update(&point.to_bytes());
        }
        // challenge_generation_domain_separator_back
        hasher.update(&[0x00]);
        let mut bytes = WideScalarBytes::default();
        hasher.finalize_xof().read(&mut bytes);
        Scalar::from_bytes_mod_order_wide(&bytes)
    }
}

impl ProofToHash for Ed448 {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-EDWARDS448-SHAKE256-RO_PROOF_TO_HASH_";
        let mut hasher = sha3::Shake256::default();
        hasher.update(DST);
        // Suite string
        hasher.update(&[SUITE_STRING]);
        // proof_to_hash_domain_separator_front
        hasher.update(&[0x03]);
        hasher.update(&gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        hasher.update(&[0x00]);

        let mut bytes = WideScalarBytes::default();
        hasher.finalize_xof().read(&mut bytes);
        Scalar::from_bytes_mod_order_wide(&bytes)
    }
}

impl Coordinate for Ed448 {
    fn point_to_scalar(pt: EdwardsPoint) -> Scalar {
        let pt = pt.compress();
        let mut wide = WideScalarBytes::default();
        wide[..57].copy_from_slice(pt.as_bytes());
        Scalar::from_bytes_mod_order_wide(&wide)
    }
}

impl VrfProver for Ed448 {}

impl VrfVerifier for Ed448 {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);
        let pk = EdwardsPoint::GENERATOR * sk;

        let res = Ed448::vrf_prove(&sk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = Ed448::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);

        let proof = Ed448::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<EdwardsPoint> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<EdwardsPoint> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }
}
