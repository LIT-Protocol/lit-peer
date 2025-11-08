use elliptic_curve::Group;

use crate::Handler;

/// Trait for generating nonces for the Elliptic Curve VRF.
/// [RFC9381](https://datatracker.ietf.org/doc/rfc9381/) section 5.4.2
pub trait NonceGeneration: Handler {
    /// Generate a nonce from the secret key and the alpha value.
    fn generate_nonce(
        sk: &<Self::Group as Group>::Scalar, alpha: &<Self::Group as Group>::Scalar,
    ) -> <Self::Group as Group>::Scalar;
}
