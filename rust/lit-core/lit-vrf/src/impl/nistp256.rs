use bulletproofs::group::Group;
use elliptic_curve::{
    PrimeField,
    bigint::U256,
    group::GroupEncoding,
    hash2curve::{ExpandMsgXmd, GroupDigest},
    ops::Reduce,
    point::AffineCoordinates,
};
use p256::{NistP256, ProjectivePoint, Scalar};
use rfc6979::consts::U32;

use crate::*;

const SUITE_STRING: u8 = 0x02;

impl Handler for NistP256 {
    type Group = ProjectivePoint;
}

impl HashToCurve for NistP256 {
    fn hash_to_curve(msg: &Scalar) -> ProjectivePoint {
        const DST: &[u8] = b"ECVRF-P256-SHA256-SSWU_RO_\x02";

        let bytes = msg.to_bytes();
        <NistP256 as GroupDigest>::hash_from_bytes::<ExpandMsgXmd<sha2::Sha256>>(&[&bytes], &[DST])
            .expect("GroupDigest::hash_from_bytes failed")
    }
}

impl NonceGeneration for NistP256 {
    fn generate_nonce(sk: &Scalar, alpha: &Scalar) -> Scalar {
        const MODULUS: &[u8; 32] = &[
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2,
            0xfc, 0x63, 0x25, 0x51,
        ];

        let output = rfc6979::generate_k::<sha2::Sha256, U32>(
            &sk.to_bytes(),
            MODULUS.into(),
            &alpha.to_bytes(),
            &[],
        );
        Scalar::from_repr(output).expect("rfc6979::generate_k failed")
    }
}

impl ChallengeGeneration for NistP256 {
    fn generate_challenge(points: &[ProjectivePoint]) -> Scalar {
        const DST: &[u8] = b"ECVRF-P256-SHA256-RO_GENERATE_CHALLENGE_";

        let mut bytes = Vec::with_capacity(3 + points.len() * 33);
        // Suite string
        bytes.push(SUITE_STRING);
        // challenge_generation_domain_separator_front
        bytes.push(0x02);
        for point in points {
            bytes.extend_from_slice(&point.to_bytes());
        }
        // challenge_generation_domain_separator_back
        bytes.push(0x00);

        <NistP256 as GroupDigest>::hash_to_scalar::<ExpandMsgXmd<sha2::Sha256>>(&[&bytes], &[DST])
            .expect("GroupDigest::hash_to_scalar failed")
    }
}

impl ProofToHash for NistP256 {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-P256-SHA256-RO_PROOF_TO_HASH_";

        let mut bytes = Vec::with_capacity(3 + 33 + 32 + 32);
        // Suite string
        bytes.push(SUITE_STRING);
        // proof_to_hash_domain_separator_front
        bytes.push(0x03);
        bytes.extend_from_slice(&gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        bytes.push(0x00);

        <NistP256 as GroupDigest>::hash_to_scalar::<ExpandMsgXmd<sha2::Sha256>>(&[&bytes], &[DST])
            .expect("GroupDigest::hash_to_scalar failed")
    }
}

impl Coordinate for NistP256 {
    fn point_to_scalar(pt: ProjectivePoint) -> Scalar {
        let pt = pt.to_affine();
        <Scalar as Reduce<U256>>::reduce_bytes(&pt.x())
    }
}

impl VrfProver for NistP256 {}

impl VrfVerifier for NistP256 {}

#[cfg(test)]
mod tests {
    use super::*;
    use elliptic_curve::Field;
    use rand::SeedableRng;

    #[test]
    fn vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);
        let pk = ProjectivePoint::GENERATOR * sk;

        let res = NistP256::vrf_prove(&sk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = NistP256::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);

        let proof = NistP256::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<ProjectivePoint> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<ProjectivePoint> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }
}
