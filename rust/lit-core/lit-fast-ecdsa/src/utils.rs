use crate::{EcdsaError, EcdsaResult};
use digest::{Digest, FixedOutput};
use ecdsa::{
    PrimeCurve,
    elliptic_curve::{
        Curve, CurveArithmetic, Field, NonZeroScalar, group::Curve as _, ops::Reduce,
        point::AffineCoordinates,
    },
    hazmat::DigestPrimitive,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};

pub fn scalar_hash<C>(msg: &[u8]) -> C::Scalar
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
{
    let digest = <C as DigestPrimitive>::Digest::new_with_prefix(msg);
    let m_bytes = digest.finalize_fixed();
    <C::Scalar as Reduce<<C as Curve>::Uint>>::reduce_bytes(&m_bytes)
}

pub fn x_coordinate<C>(point: &C::ProjectivePoint) -> C::Scalar
where
    C: PrimeCurve + CurveArithmetic,
{
    let pt = point.to_affine();
    <C::Scalar as Reduce<<C as Curve>::Uint>>::reduce_bytes(&pt.x())
}

pub fn lagrange<C>(xi: &NonZeroScalar<C>, participants: &[NonZeroScalar<C>]) -> C::Scalar
where
    C: CurveArithmetic,
{
    let xi = *(xi.as_ref());
    let mut num = C::Scalar::ONE;
    let mut den = C::Scalar::ONE;
    for xj in participants {
        let xj = *(xj.as_ref());
        if xi == xj {
            continue;
        }
        num *= xj;
        den *= xj - xi;
    }
    num * den.invert().expect("Denominator should not be zero")
}

pub fn calc_min_threshold(total_participants: usize) -> usize {
    std::cmp::max((total_participants - (total_participants & 1)) >> 1, 2)
}

/// A list of participants
#[derive(Clone, Serialize, Deserialize)]
pub struct ParticipantList<C: CurveArithmetic> {
    pub(crate) set: Vec<NonZeroScalar<C>>,
    pub(crate) _marker: PhantomData<C>,
}

impl<C> Debug for ParticipantList<C>
where
    C: CurveArithmetic,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParticipantList")
            .field("set", &self.set.iter().map(|p| p.to_string()).collect::<Vec<_>>())
            .finish()
    }
}

impl<C> Eq for ParticipantList<C> where C: CurveArithmetic {}

impl<C> PartialEq for ParticipantList<C>
where
    C: CurveArithmetic,
{
    fn eq(&self, other: &Self) -> bool {
        if self.set.len() != other.set.len() {
            return false;
        }
        for (a, b) in self.set.iter().zip(other.set.iter()) {
            if a.as_ref() != b.as_ref() {
                return false;
            }
        }
        true
    }
}

impl<C: CurveArithmetic> ParticipantList<C> {
    /// Create a new `ParticipantList` with the given `participants`
    pub fn new<B: AsRef<[C::Scalar]>>(participants: B) -> EcdsaResult<Self> {
        let participants = participants.as_ref();
        // Can't have less than 3 participants
        if participants.len() < 3 {
            return Err(EcdsaError::MinParticipants);
        }
        // Can't have more than 65536 participants
        if participants.len() > u16::MAX as usize {
            return Err(EcdsaError::MaxParticipants);
        }
        let mut set = Vec::with_capacity(participants.len());
        for p in participants {
            let id = Option::from(NonZeroScalar::new(*p)).ok_or(EcdsaError::InvalidId)?;
            set.push(id);
        }
        Ok(Self { set, _marker: PhantomData })
    }

    /// Get an iterator over the participants
    pub fn iter(&self) -> impl Iterator<Item = &NonZeroScalar<C>> + '_ {
        self.set.iter()
    }
}

#[test]
fn test_lagrange() {
    use lit_rust_crypto::k256;

    let participants: [NonZeroScalar<k256::Secp256k1>; 3] = [
        NonZeroScalar::new(k256::Scalar::ONE).unwrap(),
        NonZeroScalar::new(k256::Scalar::from(2u32)).unwrap(),
        NonZeroScalar::new(k256::Scalar::from(3u32)).unwrap(),
    ];
    let ell = lagrange(&NonZeroScalar::new(k256::Scalar::ONE).unwrap(), &participants);
    assert_eq!(ell, k256::Scalar::from(3u32));
    let ell = lagrange(&NonZeroScalar::new(k256::Scalar::from(2u32)).unwrap(), &participants);
    assert_eq!(ell, -k256::Scalar::from(3u32));
    let ell = lagrange(&NonZeroScalar::new(k256::Scalar::from(3u32)).unwrap(), &participants);
    assert_eq!(ell, k256::Scalar::ONE);
}
