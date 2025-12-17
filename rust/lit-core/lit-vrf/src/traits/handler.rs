use lit_rust_crypto::group::{Group, GroupEncoding};

/// Root trait to eliminate duplication in the other traits
pub trait Handler {
    /// The elliptic curve group
    type Group: Group + GroupEncoding + Default;
    /// Checks if a point is weak.
    ///
    /// i.e. if the point is the identity point or the point is not on the curve
    /// or the point is not in the prime order subgroup or
    /// the point has small group order. The latter two
    /// are only applicable to curves where the cofactor is greater than 1.
    fn is_weak(point: Self::Group) -> bool {
        point.is_identity().into()
    }

    /// Clears the cofactor of a point if needed.
    ///
    /// This is a no-op for most curves.
    /// So the default implementation is to return the provided point.
    ///
    /// Curves with cofactors (h > 1) should override this method.
    fn clear_cofactor(point: Self::Group) -> Self::Group {
        point
    }
}
