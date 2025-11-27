use crate::*;
use blake2::Blake2b512;
use bulletproofs::Decaf377;

use lit_rust_crypto::{
    decaf377::{Element, Fq, Fr},
    elliptic_curve::generic_array::{GenericArray, typenum::U64},
    group::{Group, GroupEncoding},
    hash2curve::{ExpandMsg, ExpandMsgXmd, Expander},
};

use sha2::Digest;

const DECAF377_SUITE_STRING: u8 = 0x09;

impl Handler for Decaf377 {
    type Group = Element;
}

impl HashToCurve for Decaf377 {
    fn hash_to_curve(msg: &Fr) -> Element {
        const DST: &[u8] = b"ECVRF-DECAF377-BLAKE2B512-ELL2_RO_\x09";

        let bytes = msg.to_bytes();
        let mut expander = ExpandMsgXmd::<Blake2b512>::expand_message(&[&bytes], &[DST], 96)
            .expect("expander creation to succeed");
        let mut uniform_bytes = [0u8; 48];
        expander.fill_bytes(&mut uniform_bytes);
        let one = Fq::from_le_bytes_mod_order(&uniform_bytes);
        expander.fill_bytes(&mut uniform_bytes);
        let two = Fq::from_le_bytes_mod_order(&uniform_bytes);

        Element::hash_to_curve(&one, &two)
    }
}

impl NonceGeneration for Decaf377 {
    fn generate_nonce(sk: &Fr, alpha: &Fr) -> Fr {
        let mut hasher = Blake2b512::default();
        hasher.update(sk.to_bytes());
        let output = hasher.finalize_reset();
        hasher.update(&output[32..]);
        hasher.update(alpha.to_bytes());
        let bytes = hasher.finalize();
        Fr::from_le_bytes_mod_order(&bytes)
    }
}

impl ChallengeGeneration for Decaf377 {
    fn generate_challenge(points: &[Element]) -> Fr {
        const DST: &[u8] = b"ECVRF-DECAF377-BLAKE2B512-RO_CHALLENGE_";
        let mut hasher = Blake2b512::default();
        hasher.update(DST);
        // Suite string
        hasher.update([DECAF377_SUITE_STRING]);
        // challenge_generation_domain_separator_front
        hasher.update([0x02]);
        for point in points {
            hasher.update(point.to_bytes().as_ref());
        }
        // challenge_generation_domain_separator_back
        hasher.update([0x00]);

        let bytes = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        Fr::from_le_bytes_mod_order(ref_bytes)
    }
}

impl ProofToHash for Decaf377 {
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar {
        const DST: &[u8] = b"ECVRF-DECAF377-BLAKE2B512-RO_PROOF_TO_HASH_";
        let mut hasher = blake2::Blake2b::default();
        hasher.update(DST);
        // Suite string
        hasher.update([DECAF377_SUITE_STRING]);
        // proof_to_hash_domain_separator_front
        hasher.update([0x03]);
        hasher.update(gamma.to_bytes());
        // proof_to_hash_domain_separator_back
        hasher.update([0x00]);

        let bytes: GenericArray<u8, U64> = hasher.finalize();
        let ref_bytes = <&[u8; 64]>::try_from(bytes.as_slice()).unwrap();
        Fr::from_le_bytes_mod_order(ref_bytes)
    }
}

impl Coordinate for Decaf377 {
    fn point_to_scalar(pt: Element) -> Fr {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&pt.to_bytes());
        Fr::from_le_bytes_mod_order(&bytes)
    }
}

impl VrfProver for Decaf377 {}
impl VrfVerifier for Decaf377 {}

#[cfg(test)]
mod tests {
    use super::*;
    use lit_rust_crypto::elliptic_curve::{Field, Group};
    use rand::SeedableRng;

    #[test]
    fn decaf377_vrf() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);

        let sk = Fr::random(&mut rng);
        let message = Fr::random(&mut rng);
        let pk = Element::generator() * sk;

        let res = Decaf377::vrf_prove(&sk, &message, None);
        assert!(res.is_ok());
        let proof = res.unwrap();
        let res = Decaf377::vrf_verify(pk, message, &proof, None);
        assert!(res.is_ok());
    }

    #[test]
    fn decaf377_serde() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let sk = Fr::random(&mut rng);
        let message = Fr::random(&mut rng);

        let proof = Decaf377::vrf_prove(&sk, &message, None).unwrap();
        let proof_bytes = serde_bare::to_vec(&proof).expect("failed to serialize proof");
        let proof2: Proof<Element> =
            serde_bare::from_slice(&proof_bytes).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);

        let proof_json = serde_json::to_string(&proof).expect("failed to serialize proof");
        let proof2: Proof<Element> =
            serde_json::from_str(&proof_json).expect("failed to deserialize proof");
        assert_eq!(proof, proof2);
    }
}
