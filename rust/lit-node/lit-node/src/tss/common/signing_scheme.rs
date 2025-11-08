use crate::error::{Result, unexpected_err};
use lit_node_core::SigningScheme;

pub fn signing_scheme_to_frost_scheme(value: SigningScheme) -> Result<lit_frost::Scheme> {
    match value {
        SigningScheme::Bls12381 | SigningScheme::Bls12381G1ProofOfPossession => Err(
            unexpected_err("BLS signatures are not supported by FROST", None),
        ),
        SigningScheme::EcdsaK256Sha256
        | SigningScheme::EcdsaP256Sha256
        | SigningScheme::EcdsaP384Sha384 => Err(unexpected_err(
            "ECDSA signatures are not supported by FROST",
            None,
        )),
        SigningScheme::SchnorrEd25519Sha512 => Ok(lit_frost::Scheme::Ed25519Sha512),
        SigningScheme::SchnorrK256Sha256 => Ok(lit_frost::Scheme::K256Sha256),
        SigningScheme::SchnorrP256Sha256 => Ok(lit_frost::Scheme::P256Sha256),
        SigningScheme::SchnorrP384Sha384 => Ok(lit_frost::Scheme::P384Sha384),
        SigningScheme::SchnorrRistretto25519Sha512 => Ok(lit_frost::Scheme::Ristretto25519Sha512),
        SigningScheme::SchnorrEd448Shake256 => Ok(lit_frost::Scheme::Ed448Shake256),
        SigningScheme::SchnorrRedJubjubBlake2b512 => Ok(lit_frost::Scheme::RedJubjubBlake2b512),
        SigningScheme::SchnorrK256Taproot => Ok(lit_frost::Scheme::K256Taproot),
        SigningScheme::SchnorrRedDecaf377Blake2b512 => Ok(lit_frost::Scheme::RedDecaf377Blake2b512),
        SigningScheme::SchnorrkelSubstrate => Ok(lit_frost::Scheme::SchnorrkelSubstrate),
    }
}
