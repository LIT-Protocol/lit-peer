use lit_rust_crypto::elliptic_curve::{Field, Group, subtle::ConstantTimeEq};

use crate::{
    ChallengeGeneration, Coordinate, HashToCurve, Proof, ProofToHash, VrfError, VrfResult,
};

/// Verifier trait for the Elliptic Curve VRF.
/// Implements [RFC9381](https://datatracker.ietf.org/doc/rfc9381/) section 5.3 VRF_verify
pub trait VrfVerifier: ChallengeGeneration + HashToCurve + ProofToHash + Coordinate {
    /// Verifies the VRF output for a given public key, alpha, and proof.
    fn vrf_verify(
        pk: Self::Group, alpha: <Self::Group as Group>::Scalar, proof: &Proof<Self::Group>,
        generator: Option<Self::Group>,
    ) -> VrfResult<()> {
        let pk = Self::clear_cofactor(pk);
        if Self::is_weak(pk) {
            return Err(VrfError::PublicKeyError);
        }

        if proof.is_invalid().into() {
            return Err(VrfError::InvalidProofInput);
        }

        let generator = generator.unwrap_or_else(Self::Group::generator);
        if Self::is_weak(generator) {
            return Err(VrfError::PublicKeyError);
        }

        let h = Self::hash_to_curve(&alpha);

        if Self::is_weak(h) {
            return Err(VrfError::HashToCurveError);
        }
        let gamma = Self::clear_cofactor(proof.gamma);

        let gk = generator * proof.s + pk * proof.c;
        let hk = h * proof.s + gamma * proof.c;

        let c = Self::generate_challenge(&[generator, h, pk, gamma, gk, hk]);

        if c.is_zero().into() {
            return Err(VrfError::ChallengeGenerationError);
        }

        let beta = Self::proof_to_hash(gamma);

        let gamma_x = Self::point_to_scalar(gamma);

        let result = c.ct_eq(&proof.c) & beta.ct_eq(&proof.beta) & gamma_x.ct_eq(&proof.gamma_x);

        if result.into() { Ok(()) } else { Err(VrfError::InvalidProof) }
    }
}
