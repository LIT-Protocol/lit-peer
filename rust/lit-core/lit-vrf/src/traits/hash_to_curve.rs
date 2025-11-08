use elliptic_curve::Group;

use crate::Handler;

/// Trait for hashing to a point on the curve for the Elliptic Curve VRF.
/// [RFC9381](https://datatracker.ietf.org/doc/rfc9381/) section 5.4.1
/// We use hash to curve instead of encode to curve for security reasons.
pub trait HashToCurve: Handler {
    /// Hashes a message to a point on the curve.
    fn hash_to_curve(msg: &<Self::Group as Group>::Scalar) -> Self::Group;
}
