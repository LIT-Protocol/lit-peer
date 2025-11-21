use lit_rust_crypto::{ff::Field, group::Group};

use crate::{
    ChallengeGeneration, Coordinate, HashToCurve, NonceGeneration, Proof, ProofToHash, VrfError,
    VrfResult,
};

/// Prover trait for the Elliptic Curve VRF.
/// Implements [RFC9381](https://datatracker.ietf.org/doc/rfc9381/) section 5.1 VRF_prove
pub trait VrfProver:
    ChallengeGeneration + NonceGeneration + HashToCurve + ProofToHash + Coordinate
{
    /// Proves the VRF output for a given secret key and alpha.
    fn vrf_prove(
        sk: &<Self::Group as Group>::Scalar, alpha: &<Self::Group as Group>::Scalar,
        generator: Option<Self::Group>,
    ) -> VrfResult<Proof<Self::Group>> {
        if sk.is_zero().into() {
            return Err(VrfError::SecretKeyError);
        }

        let generator = Self::clear_cofactor(generator.unwrap_or_else(Self::Group::generator));
        if Self::is_weak(generator) {
            return Err(VrfError::PublicKeyError);
        }

        let pk = generator * *sk;
        let h = Self::hash_to_curve(alpha);

        if Self::is_weak(h) {
            return Err(VrfError::HashToCurveError);
        }

        let k = Self::generate_nonce(sk, alpha);

        if k.is_zero().into() {
            return Err(VrfError::NonceGenerationError);
        }

        let gk = generator * k;
        let hk = h * k;

        let gamma = Self::clear_cofactor(h * *sk);
        let c = Self::generate_challenge(&[generator, h, pk, gamma, gk, hk]);

        if c.is_zero().into() {
            return Err(VrfError::ChallengeGenerationError);
        }

        let s = k - c * *sk;

        let beta = Self::proof_to_hash(gamma);

        let gamma_x = Self::point_to_scalar(gamma);

        Ok(Proof { gamma, gamma_x, c, s, beta })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VrfVerifier;
    use crate::utils::lagrange;
    use k256::{ProjectivePoint, Scalar, Secp256k1};
    use lit_rust_crypto::{k256, vsss_rs};
    use rand::SeedableRng;
    use vsss_rs::{DefaultShare, IdentifierPrimeField, ValuePrimeField, shamir};
    type SecretShare = DefaultShare<IdentifierPrimeField<Scalar>, ValuePrimeField<Scalar>>;

    #[test]
    fn different_challenges() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        let secret = Scalar::random(&mut rng);
        let alpha = Scalar::random(&mut rng);
        let pk = ProjectivePoint::GENERATOR * secret;
        let h = Secp256k1::hash_to_curve(&alpha);

        let shares = shamir::split_secret::<SecretShare>(3, 5, &(secret.into()), &mut rng).unwrap();

        let pk1 = ProjectivePoint::GENERATOR * shares[0].value.0;
        let pk2 = ProjectivePoint::GENERATOR * shares[1].value.0;
        let pk3 = ProjectivePoint::GENERATOR * shares[2].value.0;

        let proof1 = Secp256k1::vrf_prove(&shares[0].value.0, &alpha, None).unwrap();
        let proof2 = Secp256k1::vrf_prove(&shares[1].value.0, &alpha, None).unwrap();
        let proof3 = Secp256k1::vrf_prove(&shares[2].value.0, &alpha, None).unwrap();

        assert_ne!(proof1.c, proof2.c);
        assert_ne!(proof1.c, proof3.c);
        assert!(Secp256k1::vrf_verify(pk1, alpha, &proof1, None).is_ok());
        assert!(Secp256k1::vrf_verify(pk2, alpha, &proof2, None).is_ok());
        assert!(Secp256k1::vrf_verify(pk3, alpha, &proof3, None).is_ok());

        let coeff1 = lagrange(
            shares[0].identifier.0,
            &[shares[0].identifier.0, shares[1].identifier.0, shares[2].identifier.0],
        );
        let coeff2 = lagrange(
            shares[1].identifier.0,
            &[shares[0].identifier.0, shares[1].identifier.0, shares[2].identifier.0],
        );
        let coeff3 = lagrange(
            shares[2].identifier.0,
            &[shares[0].identifier.0, shares[1].identifier.0, shares[2].identifier.0],
        );

        let k1 = Secp256k1::generate_nonce(&shares[0].value.0, &alpha);
        let k2 = Secp256k1::generate_nonce(&shares[1].value.0, &alpha);
        let k3 = Secp256k1::generate_nonce(&shares[2].value.0, &alpha);

        let gamma = proof1.gamma * coeff1 + proof2.gamma * coeff2 + proof3.gamma * coeff3;
        let c = coeff1 * proof1.c + coeff2 * proof2.c + coeff3 * proof3.c;
        let s = coeff1 * proof1.s + coeff2 * proof2.s + coeff3 * proof3.s;
        let k = coeff1 * k1 + coeff2 * k2 + coeff3 * k3;

        let gk = ProjectivePoint::GENERATOR * s + pk * c;
        let hk = h * s + gamma * c;
        assert_ne!(ProjectivePoint::GENERATOR * k, gk);
        assert_ne!(h * k, hk);
    }
}
