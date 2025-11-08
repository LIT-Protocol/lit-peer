use digest::generic_array::ArrayLength;
use ecdsa::elliptic_curve::FieldBytesSize;
use ecdsa::hazmat::DigestPrimitive;
use ecdsa::{
    PrimeCurve,
    elliptic_curve::{CurveArithmetic, Field, Group, group::GroupEncoding},
};
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};
use std::ops::Add;

use crate::utils::{scalar_hash, x_coordinate};
use crate::{EcdsaError, EcdsaResult, PreSignatureParticipant, SignatureShare};

impl<C> PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    /// Compute round3 that also signs a message instead of generating a pre-signature.
    /// This is meant to be used when all rounds are run with no prior pre-signature.
    /// In this scenario, the re-randomizer is not needed because there is no pre-signature
    /// and the value `k` is used immediately.
    ///
    /// For this to work, the same signer IDs must be used for the key shares also.
    /// e.g. key_share participant ID must be the same as this participant ID.
    pub fn run_sign<B: AsRef<[u8]>>(
        &self, msg: B, key_share: &C::Scalar,
    ) -> EcdsaResult<SignatureShare<C>> {
        if !self.round3_ready() {
            return Err(EcdsaError::InsufficientRound2Payloads);
        }
        let mut w = self.w * self.participants[self.ordinal].lagrange;
        let mut big_w = self.big_w * self.participants[self.ordinal].lagrange;
        for payload in &self.received_round2_payloads {
            w += payload.w * self.participants[payload.ordinal].lagrange;
            big_w += payload.big_w * self.participants[payload.ordinal].lagrange;
        }
        if big_w.is_identity().into() {
            return Err(EcdsaError::InvalidBigW);
        }
        if w.is_zero().into() {
            return Err(EcdsaError::InvalidW);
        }
        if C::ProjectivePoint::generator() * w != big_w {
            return Err(EcdsaError::InvalidRoundResult("big_w != g^w"));
        }
        let z = scalar_hash::<C>(msg.as_ref());
        if z.is_zero().into() {
            return Err(EcdsaError::InvalidScalarZ);
        }
        let r = x_coordinate::<C>(&self.big_r);
        if r.is_zero().into() {
            return Err(EcdsaError::InvalidSignatureShare);
        }

        // Invert errors if it is zero so this also checks for a zero
        let mut w_inv = Option::<C::Scalar>::from(w.invert()).ok_or(EcdsaError::InvalidScalarK)?;
        w_inv *= self.a;

        let lagrange = self.participants[self.ordinal].lagrange;

        let s = (w_inv * (r * key_share + z) + z * self.d + self.e) * lagrange;
        Ok(SignatureShare { r: self.big_r, s })
    }
}
