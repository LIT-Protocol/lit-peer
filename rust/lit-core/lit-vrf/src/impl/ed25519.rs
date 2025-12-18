use crate::*;
use bulletproofs::{Ed25519, Ristretto25519};
use lit_rust_crypto::{
    curve25519_dalek::{EdwardsPoint, RistrettoPoint},
    group::{Group, GroupEncoding},
    hash2curve::ExpandMsgXmd,
    vsss_rs::{
        curve25519::{WrappedEdwards, WrappedRistretto, WrappedScalar},
        curve25519_dalek::{
            EdwardsPoint as AltEdwardsPoint, RistrettoPoint as AltRistrettoPoint, Scalar,
        },
    },
};
use sha2::Digest;

const EDWARDS_SUITE_STRING: u8 = 0x04;
const RISTRETTO_SUITE_STRING: u8 = 0x05;

impl Handler for Ed25519 {
    type Group = WrappedEdwards;

    fn is_weak(point: Self::Group) -> bool {
        let pt = point.0;
        !pt.is_torsion_free() || bool::from(pt.is_identity()) || pt.is_small_order()
    }

    fn clear_cofactor(point: Self::Group) -> Self::Group {
        if point.0.is_torsion_free() { point } else { WrappedEdwards(point.0.mul_by_cofactor()) }
    }
}

impl Handler for Ristretto25519 {
    type Group = WrappedRistretto;
}

impl HashToCurve for Ed25519 {
    fn hash_to_curve(msg: &WrappedScalar) -> WrappedEdwards {
        const DST: &[u8] = b"ECVRF-EDWARDS25519-SHA512-ELL2_RO_\x04";
        let bytes = msg.0.to_bytes();
        let pt = EdwardsPoint::hash_to_curve::<ExpandMsgXmd<sha2::Sha512>>(&bytes, DST);
        WrappedEdwards(unsafe { std::mem::transmute::<EdwardsPoint, AltEdwardsPoint>(pt) })
    }
}

impl HashToCurve for Ristretto25519 {
    fn hash_to_curve(msg: &WrappedScalar) -> WrappedRistretto {
        const DST: &[u8] = b"ECVRF-RISTRETTO25519-SHA512-R255MAP_RO_\x05";
        let bytes = msg.0.to_bytes();
        let pt = RistrettoPoint::hash_to_curve::<ExpandMsgXmd<sha2::Sha512>>(&bytes, DST);
        WrappedRistretto(unsafe { std::mem::transmute::<RistrettoPoint, AltRistrettoPoint>(pt) })
    }
}

impl NonceGeneration for Ed25519 {
    fn generate_nonce(
        sk: &<Self::Group as Group>::Scalar, alpha: &<Self::Group as Group>::Scalar,
    ) -> <Self::Group as Group>::Scalar {
        // RFC 8032, section 5.1.6

        let mut hasher = sha2::Sha512::default();
        hasher.update(sk.0.to_bytes());
        let output = hasher.finalize_reset();
        hasher.update(&output[32..]);
        hasher.update(alpha.0.to_bytes());
        let bytes = hasher.finalize();
        WrappedScalar(Scalar::from_bytes_mod_order_wide(&(bytes.into())))
    }
}

impl NonceGeneration for Ristretto25519 {
    fn generate_nonce(
        sk: &<Self::Group as Group>::Scalar, alpha: &<Self::Group as Group>::Scalar,
    ) -> <Self::Group as Group>::Scalar {
        // RFC 8032, section 5.1.6

        let mut hasher = sha2::Sha512::default();
        hasher.update(sk.0.to_bytes());
        let output = hasher.finalize_reset();
        hasher.update(&output[32..]);
        hasher.update(alpha.0.to_bytes());
        let bytes = hasher.finalize();
        WrappedScalar(Scalar::from_bytes_mod_order_wide(&(bytes.into())))
    }
}

impl ChallengeGeneration for Ed25519 {
    fn generate_challenge(points: &[Self::Group]) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-EDWARDS25519-SHA512-RO_GENERATE_CHALLENGE_";
        let mut hasher = sha2::Sha512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([EDWARDS_SUITE_STRING]);
        // challenge_generation_domain_separator_front
        hasher.update([0x02]);
        for point in points {
            hasher.update(point.to_bytes());
        }
        // challenge_generation_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        WrappedScalar(Scalar::from_bytes_mod_order_wide(ref_bytes))
    }
}

impl ChallengeGeneration for Ristretto25519 {
    fn generate_challenge(points: &[Self::Group]) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-RISTRETTO25519-SHA512-RO_GENERATE_CHALLENGE_";
        let mut hasher = sha2::Sha512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([RISTRETTO_SUITE_STRING]);
        // challenge_generation_domain_separator_front
        hasher.update([0x02]);
        for point in points {
            hasher.update(point.to_bytes());
        }
        // challenge_generation_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        WrappedScalar(Scalar::from_bytes_mod_order_wide(ref_bytes))
    }
}

impl ProofToHash for Ed25519 {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-EDWARDS25519-SHA512-RO_PROOF_TO_HASH_";
        let mut hasher = sha2::Sha512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([EDWARDS_SUITE_STRING]);
        // proof_to_hash_domain_separator_front
        hasher.update([0x03]);
        hasher.update(gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        WrappedScalar(Scalar::from_bytes_mod_order_wide(ref_bytes))
    }
}

impl ProofToHash for Ristretto25519 {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-RISTRETTO25519-SHA512-RO_PROOF_TO_HASH_";
        let mut hasher = sha2::Sha512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([RISTRETTO_SUITE_STRING]);
        // proof_to_hash_domain_separator_front
        hasher.update([0x03]);
        hasher.update(gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        WrappedScalar(Scalar::from_bytes_mod_order_wide(ref_bytes))
    }
}

impl Coordinate for Ed25519 {
    fn point_to_scalar(pt: Self::Group) -> <Self::Group as Group>::Scalar {
        WrappedScalar(Scalar::from_bytes_mod_order(pt.0.compress().0))
    }
}

impl Coordinate for Ristretto25519 {
    fn point_to_scalar(pt: Self::Group) -> <Self::Group as Group>::Scalar {
        WrappedScalar(Scalar::from_bytes_mod_order(pt.0.compress().0))
    }
}

impl VrfProver for Ed25519 {}
impl VrfProver for Ristretto25519 {}

impl VrfVerifier for Ed25519 {}
impl VrfVerifier for Ristretto25519 {}

#[cfg(test)]
mod tests {
    use super::*;
    use lit_rust_crypto::ff::Field;
    use rand::SeedableRng;

    #[test]
    fn ed25519_vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Scalar::random(&mut rng);
        let wsk = WrappedScalar(sk);
        let message = WrappedScalar::random(&mut rng);
        let pk = WrappedEdwards(AltEdwardsPoint::mul_base(&sk));

        let res = Ed25519::vrf_prove(&wsk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = Ed25519::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn ed25519_serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = WrappedScalar::random(&mut rng);
        let message = WrappedScalar::random(&mut rng);

        let proof = Ed25519::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<WrappedEdwards> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<WrappedEdwards> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }

    #[test]
    fn ristretto25519_vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Scalar::random(&mut rng);
        let wsk = WrappedScalar(sk);
        let message = WrappedScalar::random(&mut rng);
        let pk = WrappedRistretto(AltRistrettoPoint::mul_base(&sk));

        let res = Ristretto25519::vrf_prove(&wsk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = Ristretto25519::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn ristretto25519_serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = WrappedScalar::random(&mut rng);
        let message = WrappedScalar::random(&mut rng);

        let proof = Ristretto25519::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<WrappedRistretto> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<WrappedRistretto> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }
}
