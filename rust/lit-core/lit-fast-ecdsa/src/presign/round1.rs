use ecdsa::{
    PrimeCurve,
    elliptic_curve::{
        CurveArithmetic, Field,
        group::GroupEncoding,
        rand_core::{CryptoRng, RngCore},
    },
};
use hd_keys_curves_wasm::HDDerivable;
use vsss_rs::{FeldmanVerifierSet, ParticipantIdGeneratorType, feldman};

use super::*;
use crate::*;

type FeldmanCreateSharesOutput<F, G> = (InnerFeldmanVerifierSet<F, G>, Vec<InnerShare<F>>);
type FeldmanSplitSecretOutput<F, G> = (Vec<InnerShare<F>>, Vec<ShareVerifierGroup<G>>);

impl<C> PreSignatureParticipant<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// Get the [`Round1Payload`] for each participant
    pub(crate) fn round1(
        &mut self, mut rng: impl RngCore + CryptoRng,
    ) -> EcdsaResult<PreSignatureRound1OutputGenerator<C>> {
        if self.round != Round::Round1 {
            return Err(EcdsaError::InvalidRoundResult("Invalid round. Must be in round 1"));
        }
        let (k_verifiers, k_shares) = self.random_sharing(self.min_threshold, &mut rng)?;
        let (a_verifiers, a_shares) = self.random_sharing(self.min_threshold, &mut rng)?;
        let (b_verifiers, b_shares) = self.zero_sharing(self.two_min_threshold, &mut rng)?;
        let (d_verifiers, d_shares) = self.zero_sharing(self.threshold, &mut rng)?;
        let (e_verifiers, e_shares) = self.zero_sharing(self.threshold, &mut rng)?;

        self.a = a_shares[self.ordinal].value.0;
        self.b = b_shares[self.ordinal].value.0;
        self.d = d_shares[self.ordinal].value.0;
        self.e = e_shares[self.ordinal].value.0;
        self.k = k_shares[self.ordinal].value.0;
        self.big_r = k_verifiers.verifiers()[0].0;

        let a_verifiers: Vec<_> = a_verifiers.verifiers().iter().map(|v| v.0).collect();
        let b_verifiers: Vec<_> = b_verifiers.verifiers().iter().map(|v| v.0).collect();
        let d_verifiers: Vec<_> = d_verifiers.verifiers().iter().map(|v| v.0).collect();
        let e_verifiers: Vec<_> = e_verifiers.verifiers().iter().map(|v| v.0).collect();
        let k_verifiers: Vec<_> = k_verifiers.verifiers().iter().map(|v| v.0).collect();

        let mut secret_share_payloads = Vec::with_capacity(self.participants.len());
        for their_id in &self.participants {
            if their_id.ordinal == self.ordinal {
                continue;
            }
            secret_share_payloads.push((
                their_id.clone(),
                Round1Payload {
                    ordinal: self.ordinal,
                    id: *(self.id.as_ref()),
                    a_verifiers: a_verifiers.clone(),
                    b_verifiers: b_verifiers.clone(),
                    d_verifiers: d_verifiers.clone(),
                    e_verifiers: e_verifiers.clone(),
                    k_verifiers: k_verifiers.clone(),
                    a: a_shares[their_id.ordinal].value.0,
                    b: b_shares[their_id.ordinal].value.0,
                    k: k_shares[their_id.ordinal].value.0,
                    d: d_shares[their_id.ordinal].value.0,
                    e: e_shares[their_id.ordinal].value.0,
                },
            ));
        }
        self.round = Round::Round2;
        Ok(PreSignatureRound1OutputGenerator { map: secret_share_payloads })
    }

    /// Receive the [`Round1Payload`] the participant with the given `id`
    pub(crate) fn receive_round1_payload(&mut self, payload: Round1Payload<C>) -> EcdsaResult<()> {
        let ordinal = payload.ordinal;
        if self.round != Round::Round2 {
            return Err(EcdsaError::IncorrectRound("Must be in round 2"));
        }
        if !payload.is_valid() {
            return Err(EcdsaError::InvalidRound1Payload);
        }
        if ordinal >= self.received_round1_payloads.len() {
            return Err(EcdsaError::InvalidId);
        }
        if self.received_round1_payloads[ordinal].id == payload.id {
            return Err(EcdsaError::DuplicateSecretPayload);
        }
        if payload.a_verifiers.len() != self.min_threshold {
            return Err(EcdsaError::InvalidRoundResult("a has an invalid threshold"));
        }
        if payload.b_verifiers.len() != self.two_min_threshold {
            return Err(EcdsaError::InvalidRoundResult("b has an invalid threshold"));
        }
        if payload.d_verifiers.len() != self.threshold {
            return Err(EcdsaError::InvalidRoundResult("d has an invalid threshold"));
        }
        if payload.e_verifiers.len() != self.threshold {
            return Err(EcdsaError::InvalidRoundResult("e has an invalid threshold"));
        }
        if payload.k_verifiers.len() != self.min_threshold {
            return Err(EcdsaError::InvalidRoundResult("k has an invalid threshold"));
        }
        if self.min_threshold > self.powers_of_i.len() {
            return Err(EcdsaError::InvalidRoundResult(
                "Min threshold is greater than powers of i",
            ));
        }
        if self.threshold > self.powers_of_i.len() {
            return Err(EcdsaError::InvalidRoundResult("Threshold is greater than powers of i"));
        }
        if !self.verify_share(
            payload.a,
            &self.powers_of_i[..self.min_threshold],
            &payload.a_verifiers,
        ) {
            return Err(EcdsaError::InvalidRoundResult("Invalid a share"));
        }
        if !self.verify_share(
            payload.b,
            &self.powers_of_i[..self.two_min_threshold],
            &payload.b_verifiers,
        ) {
            return Err(EcdsaError::InvalidRoundResult("Invalid b share"));
        }
        if !self.verify_share(payload.d, &self.powers_of_i[..self.threshold], &payload.d_verifiers)
        {
            return Err(EcdsaError::InvalidRoundResult("Invalid d share"));
        }
        if !self.verify_share(payload.e, &self.powers_of_i[..self.threshold], &payload.e_verifiers)
        {
            return Err(EcdsaError::InvalidRoundResult("Invalid e share"));
        }
        if !self.verify_share(
            payload.k,
            &self.powers_of_i[..self.min_threshold],
            &payload.k_verifiers,
        ) {
            return Err(EcdsaError::InvalidRoundResult("Invalid K share"));
        }
        self.received_round1_payloads[ordinal] = payload;
        self.received_round1_count += 1;
        Ok(())
    }

    fn verify_share(
        &self, share: C::Scalar, powers_of_i: &[C::Scalar], coefficients: &[C::ProjectivePoint],
    ) -> bool {
        let rhs = C::ProjectivePoint::sum_of_products(coefficients, powers_of_i);
        let lhs = C::ProjectivePoint::generator() * share;
        (lhs - rhs).is_identity().into()
    }

    #[allow(clippy::type_complexity)]
    fn random_sharing(
        &self, threshold: usize, mut rng: impl RngCore + CryptoRng,
    ) -> EcdsaResult<FeldmanCreateSharesOutput<C::Scalar, C::ProjectivePoint>> {
        let v = C::Scalar::random(&mut rng);
        self.create_shares(threshold, v, &mut rng)
    }

    #[allow(clippy::type_complexity)]
    fn zero_sharing(
        &self, threshold: usize, mut rng: impl RngCore + CryptoRng,
    ) -> EcdsaResult<FeldmanCreateSharesOutput<C::Scalar, C::ProjectivePoint>> {
        self.create_shares(threshold, C::Scalar::ZERO, &mut rng)
    }

    fn create_shares(
        &self, threshold: usize, value: C::Scalar, rng: impl RngCore + CryptoRng,
    ) -> EcdsaResult<FeldmanCreateSharesOutput<C::Scalar, C::ProjectivePoint>> {
        let participant_ids = self
            .participants
            .iter()
            .map(|p| IdentifierPrimeField(*(p.id.as_ref())))
            .collect::<Vec<_>>();
        let seq = vec![ParticipantIdGeneratorType::list(participant_ids.as_slice())];

        let (shares, verifier_set): FeldmanSplitSecretOutput<C::Scalar, C::ProjectivePoint> =
            feldman::split_secret_with_participant_generator(
                threshold,
                participant_ids.len(),
                &IdentifierPrimeField(value),
                None,
                rng,
                &seq,
            )?;

        Ok((verifier_set.into(), shares))
    }
}
