use crate::Handler;
use elliptic_curve::Group;

/// Trait for extracting the x coordinate of a point on the curve.
pub trait Coordinate: Handler {
    /// Returns the point reduced to fix in the scalar space.
    fn point_to_scalar(pt: Self::Group) -> <Self::Group as Group>::Scalar;
}
