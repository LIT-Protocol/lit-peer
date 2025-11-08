use elliptic_curve::{
    Group, PrimeField,
    bigint::U384,
    group::GroupEncoding,
    hash2curve::{ExpandMsgXmd, GroupDigest},
    ops::Reduce,
    point::AffineCoordinates,
};
use p384::{NistP384, ProjectivePoint, Scalar};
use rfc6979::consts::U48;

use crate::*;

const SUITE_STRING: u8 = 0x07;

impl Handler for NistP384 {
    type Group = ProjectivePoint;
}

impl HashToCurve for NistP384 {
    fn hash_to_curve(msg: &Scalar) -> ProjectivePoint {
        const DST: &[u8] = b"ECVRF-P384-SHA384-SSWU_RO_\x07";

        let bytes = msg.to_bytes();
        <NistP384 as GroupDigest>::hash_from_bytes::<ExpandMsgXmd<sha2::Sha384>>(&[&bytes], &[DST])
            .expect("GroupDigest::hash_from_bytes failed")
    }
}

impl NonceGeneration for NistP384 {
    fn generate_nonce(sk: &Scalar, alpha: &Scalar) -> Scalar {
        const MODULUS: &[u8; 48] = &[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc7, 0x63, 0x4d, 0x81,
            0xf4, 0x37, 0x2d, 0xdf, 0x58, 0x1a, 0x0d, 0xb2, 0x48, 0xb0, 0xa7, 0x7a, 0xec, 0xec,
            0x19, 0x6a, 0xcc, 0xc5, 0x29, 0x73,
        ];

        let output = rfc6979::generate_k::<sha2::Sha384, U48>(
            &sk.to_bytes(),
            MODULUS.into(),
            &alpha.to_bytes(),
            &[],
        );
        Scalar::from_repr(output).expect("rfc6979::generate_k failed")
    }
}

impl ChallengeGeneration for NistP384 {
    fn generate_challenge(points: &[ProjectivePoint]) -> Scalar {
        const DST: &[u8] = b"ECVRF-P384-SHA384-RO_GENERATE_CHALLENGE_";

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

        <NistP384 as GroupDigest>::hash_to_scalar::<ExpandMsgXmd<sha2::Sha384>>(&[&bytes], &[DST])
            .expect("GroupDigest::hash_to_scalar failed")
    }
}

impl ProofToHash for NistP384 {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-P384-SHA384-RO_PROOF_TO_HASH_";

        let mut bytes = Vec::with_capacity(3 + 49 + 48 + 49);
        // Suite string
        bytes.push(SUITE_STRING);
        // proof_to_hash_domain_separator_front
        bytes.push(0x03);
        bytes.extend_from_slice(&gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        bytes.push(0x00);

        <NistP384 as GroupDigest>::hash_to_scalar::<ExpandMsgXmd<sha2::Sha384>>(&[&bytes], &[DST])
            .expect("GroupDigest::hash_to_scalar failed")
    }
}

impl Coordinate for NistP384 {
    fn point_to_scalar(pt: Self::Group) -> <Self::Group as Group>::Scalar {
        let pt = pt.to_affine();
        <Scalar as Reduce<U384>>::reduce_bytes(&pt.x())
    }
}

impl VrfProver for NistP384 {}

impl VrfVerifier for NistP384 {}

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

        let res = NistP384::vrf_prove(&sk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = NistP384::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = Scalar::random(&mut rng);
        let message = Scalar::random(&mut rng);

        let proof = NistP384::vrf_prove(&sk, &message, None).unwrap();
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
