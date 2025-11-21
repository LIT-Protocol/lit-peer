use crate::*;
use blake2::{Blake2b512, Digest};
use lit_rust_crypto::{
    group::{Group, GroupEncoding, cofactor::CofactorGroup},
    pallas::{Pallas, Point, Scalar},
    pasta::arithmetic::CurveExt,
};

const PALLAS_SUITE_STRING: u8 = 0x0B;

impl Handler for Pallas {
    type Group = Point;

    fn is_weak(pt: Self::Group) -> bool {
        (!pt.is_torsion_free() | pt.is_identity() | pt.is_small_order()).into()
    }
}

impl HashToCurve for Pallas {
    fn hash_to_curve(msg: &Scalar) -> Point {
        const DST: &str = "ECVRF-PALLAS-BLAKE2B512-SSWU_RO_\x0B";
        let bytes = msg.to_le_bytes();
        let hasher = Point::hash_to_curve(DST);
        hasher(&bytes)
    }
}

impl NonceGeneration for Pallas {
    fn generate_nonce(sk: &Scalar, alpha: &Scalar) -> Scalar {
        let mut hasher = Blake2b512::default();
        hasher.update(&sk.to_le_bytes());
        let output = hasher.finalize_reset();
        hasher.update(&output[32..]);
        hasher.update(&alpha.to_le_bytes());
        let bytes = hasher.finalize();
        Scalar::from_bytes_wide(&(bytes.into()))
    }
}

impl ChallengeGeneration for Pallas {
    fn generate_challenge(points: &[Point]) -> Scalar {
        const DST: &[u8] = b"ECVRF-PALLAS-BLAKE2B512-RO_CHALLENGE_GENERATION_";
        let mut hasher = Blake2b512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([PALLAS_SUITE_STRING]);
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

impl ProofToHash for Pallas {
    fn proof_to_hash(gamma: Point) -> Scalar {
        const DST: &[u8] = b"ECVRF-PALLAS-BLAKE2B512-RO_PROOF_TO_HASH_";
        let mut hasher = Blake2b512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([PALLAS_SUITE_STRING]);
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

impl Coordinate for Pallas {
    fn point_to_scalar(pt: Point) -> Scalar {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(pt.to_bytes().as_ref());
        Scalar::from_bytes_wide(&bytes)
    }
}

impl VrfProver for Pallas {}
impl VrfVerifier for Pallas {}

#[cfg(test)]
mod tests {
    use super::*;
    use lit_rust_crypto::ff::Field;
    use rand::SeedableRng;

    #[test]
    fn pallas_vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);
        let pk = Point::generator() * sk;

        let res = Pallas::vrf_prove(&sk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = Pallas::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn pallas_serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);

        let proof = Pallas::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<Point> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<Point> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }
}
