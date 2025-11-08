use elliptic_curve::Group;

use crate::Handler;

/// Trait for generating challenges for the Elliptic Curve VRF.
/// [RFC9381](https://datatracker.ietf.org/doc/rfc9381/) section 5.4.3
pub trait ChallengeGeneration: Handler {
    /// Generate a challenge from the given points.
    fn generate_challenge(points: &[Self::Group]) -> <Self::Group as Group>::Scalar;
}
