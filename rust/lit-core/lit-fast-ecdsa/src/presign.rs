mod data;
mod params;
mod round;
mod round1;
mod round2;
mod round3;
mod round3_sign;

pub use data::*;
pub use params::*;
pub use round::*;

use digest::generic_array::ArrayLength;
use ecdsa::elliptic_curve::{FieldBytesSize, NonZeroScalar};
use ecdsa::hazmat::DigestPrimitive;
use ecdsa::{
    PrimeCurve,
    elliptic_curve::{CurveArithmetic, Field, Group, group::GroupEncoding},
};
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};
use lit_rust_crypto::vsss_rs::{
    DefaultShare, IdentifierPrimeField, ShareVerifierGroup, ValuePrimeField, VecFeldmanVerifierSet,
};
use std::{
    fmt::{self, Debug, Formatter},
    ops::Add,
};
use subtle::ConstantTimeEq;
use zeroize::ZeroizeOnDrop;

use crate::utils::{calc_min_threshold, lagrange};
use crate::*;

pub(crate) type InnerShare<F> = DefaultShare<IdentifierPrimeField<F>, ValuePrimeField<F>>;
type InnerFeldmanVerifierSet<F, G> = VecFeldmanVerifierSet<InnerShare<F>, ShareVerifierGroup<G>>;

/// A PreSignature participant
#[derive(Clone)]
pub struct PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    pub(crate) ordinal: usize,
    pub(crate) id: NonZeroScalar<C>,
    pub(crate) round: Round,
    pub(crate) threshold: usize,
    pub(crate) min_threshold: usize,
    pub(crate) two_min_threshold: usize,
    pub(crate) powers_of_i: Vec<C::Scalar>,
    pub(crate) participants: Vec<ParticipantListItem<C>>,
    pub(crate) received_round1_payloads: Vec<Round1Payload<C>>,
    pub(crate) received_round1_count: usize,
    pub(crate) received_round2_payloads: Vec<Round2Payload<C>>,
    pub(crate) received_round2_count: usize,
    pub(crate) received_round3_payloads: Vec<Round3Payload<C::Scalar>>,
    pub(crate) received_round3_count: usize,

    pub(crate) big_r: C::ProjectivePoint,
    pub(crate) big_w: C::ProjectivePoint,
    pub(crate) a: C::Scalar,
    pub(crate) b: C::Scalar,
    pub(crate) d: C::Scalar,
    pub(crate) e: C::Scalar,
    pub(crate) k: C::Scalar,
    pub(crate) w: C::Scalar,
}

impl<C> Debug for PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreSignatureParticipant")
            .field("ordinal", &self.ordinal)
            .field("id", &self.id.to_string())
            .field("round", &self.round)
            .field("threshold", &self.threshold)
            .field("min_threshold", &self.min_threshold)
            .field("two_min_threshold", &self.two_min_threshold)
            .field("participants", &self.participants)
            .field("received_round1_payloads", &self.received_round1_payloads)
            .field("received_round1_count", &self.received_round1_count)
            .field("received_round2_payloads", &self.received_round2_payloads)
            .field("received_round2_count", &self.received_round2_count)
            .field("received_round3_payloads", &self.received_round3_payloads)
            .field("received_round3_count", &self.received_round3_count)
            .field("big_r", &self.big_r)
            .field("big_w", &self.big_w)
            .field("a", &self.a)
            .field("b", &self.b)
            .field("d", &self.d)
            .field("e", &self.e)
            .field("k", &self.k)
            .field("w", &self.w)
            .finish()
    }
}

impl<C> ZeroizeOnDrop for PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    /// Get the participant's ordinal position
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Get the participant's id
    pub fn id(&self) -> NonZeroScalar<C> {
        self.id
    }

    /// Get the participant's threshold
    pub fn threshold(&self) -> usize {
        self.threshold
    }

    /// Get the current round
    pub fn round(&self) -> Round {
        self.round
    }

    /// Create a new `PreSignatureParticipant` with the given `id` and `threshold`
    /// The participant collaborates with others to generate a [`PreSignature`]
    pub fn new(id: &NonZeroScalar<C>, params: &PreSignatureParams<C>) -> EcdsaResult<Self> {
        let ordinal =
            check_parameters_and_get_ordinal(id, params.threshold, &params.participant_list)?;
        let participants = params
            .participant_list
            .set
            .iter()
            .enumerate()
            .map(|(ordinal, id)| ParticipantListItem {
                ordinal,
                id: *id,
                lagrange: lagrange(id, &params.participant_list.set),
            })
            .collect::<Vec<_>>();
        // t for FastECDSA security, is computed s.t. n = 2t + 1
        // t represents the number of malicious participants
        // n is the number of participants
        // e.g. n = 6, 7 then t == 3
        // e.g. n = 4, 5 then t == 2
        // However we don't allow the honest threshold < 3 regardless of n < 5
        // LIT uses higher honest threshold = 2/3 n.
        // According to Ivan Damgard, this approach is still acceptable
        let len = participants.len();
        let min_threshold = calc_min_threshold(len);
        let two_t = std::cmp::min(min_threshold * 2, len);
        let my_id = participants[ordinal].id;
        let powers_of_i_len = std::cmp::max(params.threshold, two_t);
        let mut powers_of_i = vec![C::Scalar::ONE; powers_of_i_len];
        for i in 1..powers_of_i_len {
            powers_of_i[i] = powers_of_i[i - 1] * my_id.as_ref();
        }

        Ok(Self {
            ordinal,
            id: *id,
            round: Round::Round1,
            threshold: params.threshold,
            min_threshold,
            two_min_threshold: two_t,
            powers_of_i,
            participants,
            received_round1_payloads: vec![Round1Payload::default(); len],
            received_round1_count: 0,
            received_round2_payloads: vec![Round2Payload::default(); len],
            received_round2_count: 0,
            received_round3_payloads: vec![Round3Payload::default(); len],
            received_round3_count: 0,
            big_r: C::ProjectivePoint::identity(),
            big_w: C::ProjectivePoint::identity(),
            a: C::Scalar::ZERO,
            b: C::Scalar::ZERO,
            d: C::Scalar::ZERO,
            e: C::Scalar::ZERO,
            k: C::Scalar::ZERO,
            w: C::Scalar::ZERO,
        })
    }

    /// True if the participant is ready to run the next round
    pub fn ready(&self) -> bool {
        match self.round {
            Round::Round1 => true,
            Round::Round2 => self.round2_ready(),
            Round::Round3 => self.round3_ready(),
            Round::Round4 => self.presign_ready(),
        }
    }

    /// Run the next round
    pub fn run(&mut self) -> EcdsaResult<PreSignatureRoundOutputGenerator<'_, C>> {
        match self.round {
            Round::Round1 => {
                Ok(PreSignatureRoundOutputGenerator::Round1(self.round1(rand::rngs::OsRng)?))
            }
            Round::Round2 => Ok(PreSignatureRoundOutputGenerator::Round2(self.round2()?)),
            Round::Round3 => Ok(PreSignatureRoundOutputGenerator::Round3(self.round3()?)),
            Round::Round4 => Ok(PreSignatureRoundOutputGenerator::Round4(self.presign()?)),
        }
    }

    /// Receive a payload from another participant
    pub fn receive(&mut self, payload: RoundPayload<C>) -> EcdsaResult<()> {
        match (self.round, payload) {
            (Round::Round2, RoundPayload::Round1(payload)) => self.receive_round1_payload(payload),
            (Round::Round3, RoundPayload::Round2(payload)) => self.receive_round2_payload(payload),
            (Round::Round4, RoundPayload::Round3(payload)) => self.receive_round3_payload(payload),
            (_, _) => Err(EcdsaError::IncorrectRound("round doesn't expect to receive a payload")),
        }
    }

    /// Check if this participant can compute the pre-signature
    /// Previous rounds 1-3 must have been completed already
    pub(crate) fn presign_ready(&self) -> bool {
        self.received_round3_count == self.participants.len() - 1 && self.round == Round::Round4
    }

    /// Compute the pre-signature
    pub(crate) fn presign(&self) -> EcdsaResult<PreSignature<C>> {
        if !self.presign_ready() {
            return Err(EcdsaError::InsufficientRound3Payloads);
        }
        let w_inv = Option::<C::Scalar>::from(self.w.invert()).ok_or(EcdsaError::InvalidScalarK)?;
        let k_inv = self.received_round3_payloads.iter().enumerate().fold(
            self.a * w_inv * self.participants[self.ordinal].lagrange,
            |acc, (ordinal, payload)| {
                if ordinal == self.ordinal {
                    return acc;
                }
                acc + payload.w_inv * self.participants[payload.ordinal].lagrange
            },
        );
        if !bool::from((self.big_r * k_inv).ct_eq(&C::ProjectivePoint::generator())) {
            return Err(EcdsaError::InvalidScalarK);
        }
        Ok(PreSignature {
            id: *(self.id.as_ref()),
            threshold: self.threshold,
            big_r: self.big_r,
            k_inv,
            d: self.d,
            e: self.e,
        })
    }
}

#[derive(Clone)]
pub(crate) struct ParticipantListItem<C>
where
    C: PrimeCurve + CurveArithmetic,
{
    pub(crate) ordinal: usize,
    pub(crate) id: NonZeroScalar<C>,
    pub(crate) lagrange: C::Scalar,
}

impl<C> Debug for ParticipantListItem<C>
where
    C: PrimeCurve + CurveArithmetic,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParticipantListItem")
            .field("ordinal", &self.ordinal)
            .field("id", &self.id.to_string())
            .field("lagrange", &self.lagrange)
            .finish()
    }
}

fn check_parameters_and_get_ordinal<C>(
    id: &NonZeroScalar<C>, threshold: usize, participant_list: &ParticipantList<C>,
) -> EcdsaResult<usize>
where
    C: CurveArithmetic,
{
    if threshold > participant_list.set.len() {
        return Err(EcdsaError::ThresholdGreaterThanParticipants);
    }
    let min_threshold = calc_min_threshold(participant_list.set.len());
    if threshold < min_threshold {
        return Err(EcdsaError::MinThreshold(format!(
            "threshold < min_threshold: {} < {}",
            threshold, min_threshold
        )));
    }
    participant_list
        .set
        .iter()
        .position(|pid| pid.as_ref() == id.as_ref())
        .ok_or(EcdsaError::MissingParticipant)
}
