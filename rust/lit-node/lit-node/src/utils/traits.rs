use lit_node_core::CurveType;
use lit_rust_crypto::{
    blsful::inner_types, decaf377, ed448_goldilocks, group::Group, jubjub, k256, p256, p384,
    pallas, vsss_rs,
};

pub trait SignatureCurve {
    const CURVE_TYPE: CurveType;
    type Point;

    fn signing_generator() -> Self::Point;
}

impl SignatureCurve for k256::Secp256k1 {
    const CURVE_TYPE: CurveType = CurveType::K256;
    type Point = k256::ProjectivePoint;

    fn signing_generator() -> Self::Point {
        k256::ProjectivePoint::GENERATOR
    }
}

impl SignatureCurve for p256::NistP256 {
    const CURVE_TYPE: CurveType = CurveType::P256;
    type Point = p256::ProjectivePoint;

    fn signing_generator() -> Self::Point {
        p256::ProjectivePoint::GENERATOR
    }
}

impl SignatureCurve for p384::NistP384 {
    const CURVE_TYPE: CurveType = CurveType::P384;
    type Point = p384::ProjectivePoint;

    fn signing_generator() -> Self::Point {
        p384::ProjectivePoint::GENERATOR
    }
}

impl SignatureCurve for bulletproofs::Ed25519 {
    const CURVE_TYPE: CurveType = CurveType::Ed25519;
    type Point = vsss_rs::curve25519::WrappedEdwards;

    fn signing_generator() -> Self::Point {
        vsss_rs::curve25519::WrappedEdwards::generator()
    }
}

impl SignatureCurve for bulletproofs::Ristretto25519 {
    const CURVE_TYPE: CurveType = CurveType::Ristretto25519;
    type Point = vsss_rs::curve25519::WrappedRistretto;

    fn signing_generator() -> Self::Point {
        vsss_rs::curve25519::WrappedRistretto::generator()
    }
}

impl SignatureCurve for ed448_goldilocks::Ed448 {
    const CURVE_TYPE: CurveType = CurveType::Ed448;
    type Point = ed448_goldilocks::EdwardsPoint;

    fn signing_generator() -> Self::Point {
        ed448_goldilocks::EdwardsPoint::GENERATOR
    }
}

impl SignatureCurve for bulletproofs::JubJub {
    const CURVE_TYPE: CurveType = CurveType::RedJubjub;
    type Point = jubjub::SubgroupPoint;

    fn signing_generator() -> Self::Point {
        lit_rust_crypto::red_jubjub_signing_generator()
    }
}

impl SignatureCurve for pallas::Pallas {
    const CURVE_TYPE: CurveType = CurveType::RedPallas;
    type Point = pallas::Point;

    fn signing_generator() -> Self::Point {
        lit_rust_crypto::red_pallas_signing_generator()
    }
}

impl SignatureCurve for bulletproofs::Decaf377 {
    const CURVE_TYPE: CurveType = CurveType::RedDecaf377;
    type Point = decaf377::Element;

    fn signing_generator() -> Self::Point {
        decaf377::Element::GENERATOR
    }
}

impl SignatureCurve for inner_types::InnerBls12381G1 {
    const CURVE_TYPE: CurveType = CurveType::BLS12381G1;
    type Point = inner_types::G1Projective;

    fn signing_generator() -> Self::Point {
        inner_types::G1Projective::GENERATOR
    }
}
