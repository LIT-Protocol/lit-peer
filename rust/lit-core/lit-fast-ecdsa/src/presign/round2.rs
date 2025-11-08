use crate::{
    EcdsaError, EcdsaResult, PreSignatureParticipant, PreSignatureRound2OutputGenerator, Round,
    Round2Payload,
};
use ecdsa::{
    PrimeCurve,
    elliptic_curve::{CurveArithmetic, Group, group::GroupEncoding},
};
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};

impl<C> PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Check if this participant can begin round 2
    pub(crate) fn round2_ready(&self) -> bool {
        self.received_round1_count == self.participants.len() - 1 && self.round == Round::Round2
    }

    /// Begin round 2
    pub(crate) fn round2(&mut self) -> EcdsaResult<PreSignatureRound2OutputGenerator<'_, C>> {
        if !self.round2_ready() {
            return Err(EcdsaError::InsufficientRound1Payloads);
        }
        for (ordinal, payload) in self.received_round1_payloads.iter().enumerate() {
            if ordinal == self.ordinal {
                continue;
            }
            self.a += payload.a;
            self.b += payload.b;
            self.d += payload.d;
            self.e += payload.e;
            self.k += payload.k;
            self.big_r += payload.k_verifiers[0];
        }
        if self.big_r.is_identity().into() {
            return Err(EcdsaError::InvalidBigR);
        }
        self.big_w = self.big_r * self.a;
        self.w = self.a * self.k + self.b;

        self.round = Round::Round3;
        Ok(PreSignatureRound2OutputGenerator {
            participants: self.participants.as_slice(),
            round2: Round2Payload {
                ordinal: self.ordinal,
                id: *(self.id.as_ref()),
                big_r: self.big_r,
                big_w: self.big_w,
                w: self.w,
            },
        })
    }

    /// Receive the round 2 payload from the participant with the given `id`
    pub(crate) fn receive_round2_payload(&mut self, payload: Round2Payload<C>) -> EcdsaResult<()> {
        if self.round != Round::Round3 {
            return Err(EcdsaError::IncorrectRound("Must be in round 3"));
        }
        if !payload.is_valid() {
            return Err(EcdsaError::InvalidRound2Payload);
        }
        if payload.ordinal >= self.received_round2_payloads.len() {
            return Err(EcdsaError::InvalidId);
        }
        if payload.big_r.is_identity().into() {
            return Err(EcdsaError::InvalidBigR);
        }
        if payload.big_r != self.big_r {
            return Err(EcdsaError::InvalidBigR);
        }
        if self.received_round2_payloads[payload.ordinal].id == payload.id {
            return Err(EcdsaError::DuplicateCommitment);
        }
        self.received_round2_payloads[payload.ordinal] = payload;
        self.received_round2_count += 1;
        Ok(())
    }
}
