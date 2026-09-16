use crate::presign::ParticipantListItem;
use crate::sign::PreSignature;
use ecdsa::elliptic_curve::{Field, NonZeroScalar};
use ecdsa::{
    PrimeCurve,
    elliptic_curve::{CurveArithmetic, Group, PrimeField, group::GroupEncoding},
};
use elliptic_curve_tools::{group, group_vec, prime_field};
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use subtle::Choice;
use zeroize::ZeroizeOnDrop;

/// A round payload
#[derive(Clone)]
pub struct PreSignatureRoundOutput<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// The destination participant's ordinal position
    pub ordinal: usize,
    /// The destination participant's id
    pub id: NonZeroScalar<C>,
    /// The destination participant's round payload
    pub round_payload: RoundPayload<C>,
}

impl<C> Debug for PreSignatureRoundOutput<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreSignatureRoundOutput")
            .field("ordinal", &self.ordinal)
            .field("id", &self.id.to_string())
            .field("round_payload", &self.round_payload)
            .finish()
    }
}

/// A PreSignature Round Output Generator
#[derive(Debug, Clone)]
pub enum PreSignatureRoundOutputGenerator<'a, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Round 1 output generator
    Round1(PreSignatureRound1OutputGenerator<C>),
    /// Round 2 output generator
    Round2(PreSignatureRound2OutputGenerator<'a, C>),
    /// Round 3 output generator
    Round3(PreSignatureRound3OutputGenerator<'a, C>),
    /// Round 4 output generator
    Round4(PreSignature<C>),
}

impl<C> ZeroizeOnDrop for PreSignatureRoundOutputGenerator<'_, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> PreSignatureRoundOutputGenerator<'_, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Iterate over the data for the participants
    pub fn iter(&self) -> Box<dyn Iterator<Item = PreSignatureRoundOutput<C>> + '_> {
        match self {
            PreSignatureRoundOutputGenerator::Round1(r#gen) => Box::new(r#gen.iter()),
            PreSignatureRoundOutputGenerator::Round2(r#gen) => Box::new(r#gen.iter()),
            PreSignatureRoundOutputGenerator::Round3(r#gen) => Box::new(r#gen.iter()),
            _ => Box::new(std::iter::empty()),
        }
    }

    /// Get the output of the presignature if available
    pub fn output(&self) -> Option<PreSignature<C>> {
        if let PreSignatureRoundOutputGenerator::Round4(pre_signature) = self {
            Some(*pre_signature)
        } else {
            None
        }
    }
}

/// Enumeration of the payloads for each round
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RoundPayload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Round1 payload
    #[serde(bound(
        serialize = "Round1Payload<C>: Serialize",
        deserialize = "Round1Payload<C>: Deserialize<'de>"
    ))]
    Round1(Round1Payload<C>),
    /// Round2 payload
    #[serde(bound(
        serialize = "Round1Payload<C>: Serialize",
        deserialize = "Round1Payload<C>: Deserialize<'de>"
    ))]
    Round2(Round2Payload<C>),
    /// Round3 payload
    #[serde(bound(
        serialize = "Round1Payload<C>: Serialize",
        deserialize = "Round1Payload<C>: Deserialize<'de>"
    ))]
    Round3(Round3Payload<C::Scalar>),
    /// Round4 payload
    #[serde(bound(
        serialize = "Round1Payload<C>: Serialize",
        deserialize = "Round1Payload<C>: Deserialize<'de>"
    ))]
    Round4(PreSignature<C>),
}

impl<C> ZeroizeOnDrop for RoundPayload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

/// Pre-Signature Round 3 Output Generator
#[derive(Debug, Clone)]
pub struct PreSignatureRound3OutputGenerator<'a, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// The other participants to send round3 payloads
    pub(crate) participants: &'a [ParticipantListItem<C>],
    /// The payload to send to the other participants
    pub(crate) round3: Round3Payload<C::Scalar>,
}

impl<C> ZeroizeOnDrop for PreSignatureRound3OutputGenerator<'_, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> PreSignatureRound3OutputGenerator<'_, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Iterate over all the payloads to send out
    pub fn iter(&self) -> impl Iterator<Item = PreSignatureRoundOutput<C>> + '_ {
        self.participants.iter().filter_map(|item| {
            if item.id.as_ref() == &self.round3.id {
                return None;
            }
            Some(PreSignatureRoundOutput {
                ordinal: item.ordinal,
                id: item.id,
                round_payload: RoundPayload::Round3(self.round3),
            })
        })
    }
}

/// A payload for round 3
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Round3Payload<F: PrimeField + HDDeriver> {
    /// The ordinal position of the participant
    pub(crate) ordinal: usize,
    /// The sending participant id
    #[serde(with = "prime_field")]
    pub(crate) id: F,
    /// a_i / w e.g. a_i / a * k
    #[serde(with = "prime_field")]
    pub(crate) w_inv: F,
}

impl<F: PrimeField + HDDeriver> ZeroizeOnDrop for Round3Payload<F> {}

impl<F: PrimeField + HDDeriver> Round3Payload<F> {
    /// Check if the payload is valid
    pub fn is_valid(&self) -> bool {
        bool::from(!self.id.is_zero())
    }
}

/// Pre-Signature Round 2 Output Generator
#[derive(Debug, Clone)]
pub struct PreSignatureRound2OutputGenerator<'a, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// The payload to send to the other participants
    pub(crate) round2: Round2Payload<C>,
    /// All participants to send round2 payloads, the sending id will be excluded
    pub(crate) participants: &'a [ParticipantListItem<C>],
}

impl<C> ZeroizeOnDrop for PreSignatureRound2OutputGenerator<'_, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> PreSignatureRound2OutputGenerator<'_, C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Iterate over all participants to send round2 payloads
    pub fn iter(&self) -> impl Iterator<Item = PreSignatureRoundOutput<C>> + '_ {
        self.participants.iter().filter_map(|item| {
            if item.id.as_ref() == &self.round2.id {
                return None;
            }
            Some(PreSignatureRoundOutput {
                ordinal: item.ordinal,
                id: item.id,
                round_payload: RoundPayload::Round2(self.round2),
            })
        })
    }
}

/// A payload for sharing secrets in round 2
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Round2Payload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// The sending participant's ordinal position
    pub ordinal: usize,
    /// The sending participant id
    #[serde(with = "prime_field")]
    pub id: C::Scalar,
    /// The big R value a.k.a g^k
    #[serde(with = "group")]
    pub big_r: C::ProjectivePoint,
    #[serde(with = "group")]
    /// The big R value a.k.a g^ka_i
    pub big_w: C::ProjectivePoint,
    /// w_i = k_i * a1_i + b_i
    #[serde(with = "prime_field")]
    pub w: C::Scalar,
}

impl<C> ZeroizeOnDrop for Round2Payload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> Round2Payload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Check if the payload is valid
    pub fn is_valid(&self) -> bool {
        bool::from(!self.id.is_zero() & !self.big_r.is_identity())
    }
}

/// Pre-Signature Round 1 Output Generator
/// creates payloads for sending to other participants
#[derive(Debug, Clone)]
pub struct PreSignatureRound1OutputGenerator<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    pub(crate) map: Vec<(ParticipantListItem<C>, Round1Payload<C>)>,
}

impl<C> ZeroizeOnDrop for PreSignatureRound1OutputGenerator<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> PreSignatureRound1OutputGenerator<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Iterate over all the payloads to send out
    pub fn iter(&self) -> impl Iterator<Item = PreSignatureRoundOutput<C>> + '_ {
        self.map.iter().map(|(item, payload)| PreSignatureRoundOutput {
            ordinal: item.ordinal,
            id: item.id,
            round_payload: RoundPayload::Round1(payload.clone()),
        })
    }
}

/// A payload for sharing secrets in round 1
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Round1Payload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// The sending participant's ordinal position
    pub ordinal: usize,
    /// The sending participant id
    #[serde(with = "prime_field")]
    pub id: C::Scalar,
    /// The verifiers for the `a1` secret shares
    #[serde(with = "group_vec")]
    pub a_verifiers: Vec<C::ProjectivePoint>,
    /// The verifiers for the `b` secret shares
    #[serde(with = "group_vec")]
    pub b_verifiers: Vec<C::ProjectivePoint>,
    /// The verifiers for the `k` secret shares
    #[serde(with = "group_vec")]
    pub k_verifiers: Vec<C::ProjectivePoint>,
    /// The verifiers for the `d` secret shares
    #[serde(with = "group_vec")]
    pub d_verifiers: Vec<C::ProjectivePoint>,
    /// The verifiers for the `e` secret shares
    #[serde(with = "group_vec")]
    pub e_verifiers: Vec<C::ProjectivePoint>,
    /// The secret share a_j
    #[serde(with = "prime_field")]
    pub a: C::Scalar,
    /// The secret share b_j
    #[serde(with = "prime_field")]
    pub b: C::Scalar,
    /// The secret share k_j
    #[serde(with = "prime_field")]
    pub k: C::Scalar,
    /// The secret share d_j
    #[serde(with = "prime_field")]
    pub d: C::Scalar,
    /// The secret share e_j
    #[serde(with = "prime_field")]
    pub e: C::Scalar,
}

impl<C> ZeroizeOnDrop for Round1Payload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> Round1Payload<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Check if the payload is valid
    pub fn is_valid(&self) -> bool {
        if self.a_verifiers.is_empty() || self.b_verifiers.is_empty() || self.k_verifiers.is_empty()
        {
            return false;
        }
        let id = !self.id.is_zero();
        let a = self.a_verifiers.iter().rfold(Choice::from(1u8), |acc, v| acc & !v.is_identity());
        let k = self.k_verifiers.iter().rfold(Choice::from(1u8), |acc, v| acc & !v.is_identity());
        let b = self.b_verifiers[1..]
            .iter()
            .rfold(self.b_verifiers[0].is_identity(), |acc, v| acc & !v.is_identity());
        let d = self.d_verifiers[1..]
            .iter()
            .rfold(self.d_verifiers[0].is_identity(), |acc, v| acc & !v.is_identity());
        let e = self.e_verifiers[1..]
            .iter()
            .rfold(self.e_verifiers[0].is_identity(), |acc, v| acc & !v.is_identity());
        bool::from(id & a & k & b & d & e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_tests() {
        use lit_rust_crypto::k256;

        let round_data = RoundPayload::Round1(Round1Payload {
            ordinal: 1,
            id: k256::Scalar::from(2u64),
            a_verifiers: vec![k256::ProjectivePoint::GENERATOR; 3],
            b_verifiers: vec![k256::ProjectivePoint::GENERATOR; 3],
            k_verifiers: vec![k256::ProjectivePoint::GENERATOR; 3],
            d_verifiers: vec![k256::ProjectivePoint::GENERATOR; 3],
            e_verifiers: vec![k256::ProjectivePoint::GENERATOR; 3],
            a: k256::Scalar::from(3u64),
            b: k256::Scalar::from(5u64),
            k: k256::Scalar::from(6u64),
            d: k256::Scalar::from(7u64),
            e: k256::Scalar::from(8u64),
        });

        let s = serde_json::to_string(&round_data).unwrap();
        let round_data2: RoundPayload<k256::Secp256k1> = serde_json::from_str(&s).unwrap();
        assert_eq!(round_data, round_data2);
    }
}
