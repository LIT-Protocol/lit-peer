use crate::{
    EcdsaError, EcdsaResult, PreSignatureParticipant, PreSignatureRound3OutputGenerator, Round,
    Round3Payload,
};
use ecdsa::elliptic_curve::Field;
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
    pub(crate) fn round3_ready(&self) -> bool {
        self.received_round2_count == self.participants.len() - 1 && self.round == Round::Round3
    }

    /// Get the [`ShareSecretsPayload`] for each participant
    pub(crate) fn round3(&mut self) -> EcdsaResult<PreSignatureRound3OutputGenerator<'_, C>> {
        if self.round != Round::Round3 {
            return Err(EcdsaError::InsufficientRound2Payloads);
        }
        self.w *= self.participants[self.ordinal].lagrange;
        self.big_w *= self.participants[self.ordinal].lagrange;
        for (ordinal, payload) in self.received_round2_payloads.iter().enumerate() {
            if ordinal == self.ordinal {
                continue;
            }
            self.w += payload.w * self.participants[payload.ordinal].lagrange;
            self.big_w += payload.big_w * self.participants[payload.ordinal].lagrange;
        }
        if self.big_w.is_identity().into() {
            return Err(EcdsaError::InvalidBigW);
        }
        if self.w.is_zero().into() {
            return Err(EcdsaError::InvalidW);
        }
        if C::ProjectivePoint::generator() * self.w != self.big_w {
            return Err(EcdsaError::InvalidRoundResult("big_w != g^w"));
        }
        self.round = Round::Round4;
        Ok(PreSignatureRound3OutputGenerator {
            participants: self.participants.as_slice(),
            round3: Round3Payload {
                ordinal: self.ordinal,
                id: *(self.id.as_ref()),
                w_inv: self.w.invert().unwrap() * self.a,
            },
        })
    }

    /// Receive the round 3 payload from the participant with the given `id`
    pub(crate) fn receive_round3_payload(
        &mut self, payload: Round3Payload<C::Scalar>,
    ) -> EcdsaResult<()> {
        if self.round != Round::Round4 {
            return Err(EcdsaError::IncorrectRound("Must be in round 4"));
        }
        if !payload.is_valid() {
            return Err(EcdsaError::InvalidRound3Payload);
        }
        if payload.ordinal >= self.received_round3_payloads.len() {
            return Err(EcdsaError::InvalidId);
        }
        if self.received_round3_payloads[payload.ordinal].id == payload.id {
            return Err(EcdsaError::InvalidRound3Payload);
        }
        self.received_round3_payloads[payload.ordinal] = payload;
        self.received_round3_count += 1;
        Ok(())
    }
}
