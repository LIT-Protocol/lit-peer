use crate::*;
use bulletproofs::JubJub;
use elliptic_curve::{
    Group,
    group::{GroupEncoding, cofactor::CofactorGroup},
    hash2curve::ExpandMsgXmd,
};
use jubjub::{ExtendedPoint, Scalar, SubgroupPoint};
use sha2::Digest;

const JUBJUB_SUITE_STRING: u8 = 0x08;

impl Handler for JubJub {
    type Group = SubgroupPoint;

    fn is_weak(point: Self::Group) -> bool {
        let pt = ExtendedPoint::from(point);
        (!pt.is_torsion_free() | pt.is_identity() | pt.is_small_order()).into()
    }

    fn clear_cofactor(point: Self::Group) -> Self::Group {
        let pt = ExtendedPoint::from(point);
        if pt.is_torsion_free().into() { point } else { pt.clear_cofactor() }
    }
}

impl HashToCurve for JubJub {
    fn hash_to_curve(msg: &Scalar) -> SubgroupPoint {
        const DST: &[u8] = b"ECVRF-JUBJUB-BLAKE2B512-HP_RO_\x08";
        let bytes = msg.to_bytes();
        SubgroupPoint::from(ExtendedPoint::hash::<ExpandMsgXmd<blake2::Blake2b512>>(&bytes, DST))
    }
}

impl NonceGeneration for JubJub {
    fn generate_nonce(
        sk: &<Self::Group as Group>::Scalar, alpha: &<Self::Group as Group>::Scalar,
    ) -> <Self::Group as Group>::Scalar {
        let mut hasher = blake2::Blake2b512::default();
        hasher.update(sk.to_bytes());
        let output = hasher.finalize_reset();
        hasher.update(&output[32..]);
        hasher.update(alpha.to_bytes());
        let bytes = hasher.finalize();
        Scalar::from_bytes_wide(&(bytes.into()))
    }
}

impl ChallengeGeneration for JubJub {
    fn generate_challenge(points: &[Self::Group]) -> Scalar {
        const DST: &[u8] = b"ECVRF-JUBJUB-BLAKE2B512-RO_CHALLENGE_GENERATION_";
        let mut hasher = blake2::Blake2b512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([JUBJUB_SUITE_STRING]);
        // challenge_generation_domain_separator_front
        hasher.update([0x02]);

        for point in points {
            hasher.update(point.to_bytes());
        }
        // challenge_generation_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        Scalar::from_bytes_wide(ref_bytes)
    }
}

impl ProofToHash for JubJub {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-JUBJUB-BLAKE2B512-RO_PROOF_TO_HASH_";
        let mut hasher = blake2::Blake2b512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([JUBJUB_SUITE_STRING]);
        // proof_to_hash_domain_separator_front
        hasher.update([0x03]);
        hasher.update(gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        Scalar::from_bytes_wide(ref_bytes)
    }
}

impl Coordinate for JubJub {
    fn point_to_scalar(pt: Self::Group) -> <Self::Group as Group>::Scalar {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(pt.to_bytes().as_ref());
        Scalar::from_bytes_wide(&bytes)
    }
}

impl VrfProver for JubJub {}
impl VrfVerifier for JubJub {}

#[cfg(test)]
mod tests {
    use super::*;
    use elliptic_curve::Field;
    use rand::SeedableRng;

    #[test]
    fn jubjub_vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);
        let pk = SubgroupPoint::generator() * sk;

        let res = JubJub::vrf_prove(&sk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = JubJub::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn jubjub_serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);

        let proof = JubJub::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<SubgroupPoint> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<SubgroupPoint> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }
}
