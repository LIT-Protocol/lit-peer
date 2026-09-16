use lit_rust_crypto::group::Group;

use crate::Handler;

/// Trait for hashing to a proof for the Elliptic Curve VRF.
/// [RFC9381](https://datatracker.ietf.org/doc/rfc9381/) section 5.2
pub trait ProofToHash: Handler {
    /// Hashes the proof to a scalar.
    fn proof_to_hash(gamma: Self::Group) -> <Self::Group as Group>::Scalar;
}
