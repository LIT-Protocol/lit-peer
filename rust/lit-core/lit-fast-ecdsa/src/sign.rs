use crate::utils::*;
use crate::*;
use digest::{FixedOutput, Update};
use ecdsa::{
    Signature,
    elliptic_curve::{
        Curve, CurveArithmetic, Field, FieldBytesSize, Group, NonZeroScalar, PrimeCurve,
        PrimeField, ScalarPrimitive,
        generic_array::{ArrayLength, typenum::Unsigned},
        group::GroupEncoding,
        ops::Reduce,
    },
    hazmat::DigestPrimitive,
};
use elliptic_curve_tools::{group, prime_field};
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use zeroize::ZeroizeOnDrop;

/// A full ecdsa signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSignature<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    /// The signature `r` component
    #[serde(with = "group")]
    pub r: C::ProjectivePoint,
    /// The signature `s` component
    #[serde(with = "prime_field")]
    pub s: C::Scalar,
}

impl<C> TryFrom<FullSignature<C>> for Signature<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    type Error = EcdsaError;

    fn try_from(value: FullSignature<C>) -> EcdsaResult<Self> {
        let r = x_coordinate::<C>(&value.r);
        let r = <C::Scalar as Into<ScalarPrimitive<C>>>::into(r);
        let s = <C::Scalar as Into<ScalarPrimitive<C>>>::into(value.s);
        // from_scalars checks that both r and s are not zero
        let signature = Signature::<C>::from_scalars(r.to_bytes(), s.to_bytes())
            .map_err(|_| EcdsaError::InvalidSignatureResult)?;
        match signature.normalize_s() {
            Some(normalized) => Ok(normalized),
            None => Ok(signature),
        }
    }
}

impl<C> FullSignature<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    /// Verify the signature where message is the original byte sequence.
    /// This is not the standard ECDSA method but a simplified version
    /// with no inversions.
    /// However, [`FullSignature`] can be converted to ecdsa::Signature<C> using try_from
    /// and verified using the standard method.
    pub fn verify_prehash<B: AsRef<[u8]>>(
        &self, msg: B, public_key: &C::ProjectivePoint,
    ) -> EcdsaResult<()> {
        let z = scalar_hash::<C>(msg.as_ref());
        self.verify_scalar(z, public_key)
    }

    /// Verify the signature where `msg_digest` is the hash of the original byte sequence.
    /// This is not the standard ECDSA method but a simplified version
    /// with no inversions.
    /// However, [`FullSignature`] can be converted to ecdsa::Signature<C> using try_from
    /// and verified using the standard method.
    pub fn verify_digest<B: AsRef<[u8]>>(
        &self, msg_digest: B, public_key: &C::ProjectivePoint,
    ) -> EcdsaResult<()> {
        let digest = msg_digest.as_ref();
        if digest.len() != C::FieldBytesSize::to_usize() {
            return Err(EcdsaError::InvalidScalarZ);
        }
        let z_bytes = <C::Scalar as Reduce<<C as Curve>::Uint>>::Bytes::from_slice(digest);
        let z = <C::Scalar as Reduce<<C as Curve>::Uint>>::reduce_bytes(z_bytes);
        self.verify_scalar(z, public_key)
    }

    /// Verify the signature where `z` is the hashed scalar form of the original byte sequence.
    /// This is not the standard ECDSA method but a simplified version
    /// with no inversions.
    /// However, [`FullSignature`] can be converted to ecdsa::Signature<C> using try_from
    /// and verified using the standard method.
    pub fn verify_scalar(&self, z: C::Scalar, public_key: &C::ProjectivePoint) -> EcdsaResult<()> {
        if z.is_zero().into() {
            return Err(EcdsaError::InvalidScalarZ);
        }
        if (self.s.is_zero() | self.r.is_identity()).into() {
            return Err(EcdsaError::InvalidSignatureResult);
        }
        let r = x_coordinate::<C>(&self.r);
        if r.is_zero().into() {
            return Err(EcdsaError::InvalidSignatureResult);
        }
        // sR == zG * rY =
        // (z + rx/k) * k * G == zG + rxG =
        // (z + rx) G == (z + rx) G
        if (self.r * self.s - (*public_key * r + C::ProjectivePoint::generator() * z))
            .is_identity()
            .into()
        {
            Ok(())
        } else {
            Err(EcdsaError::InvalidSignatureResult)
        }
    }
}

/// A signature share
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureShare<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    /// The signature `r` component
    #[serde(with = "group")]
    pub r: C::ProjectivePoint,
    /// The signature `s` component
    #[serde(with = "prime_field")]
    pub s: C::Scalar,
}

impl<C> SignatureShare<C>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    /// Create a new `SignatureShare` with the `pre_signature`, the secret key `x`,
    /// and the message `msg`.
    /// The nonce serves as a re-randomizer to prevent key recovery attacks as
    /// described in <https://eprint.iacr.org/2021/1330>
    pub fn new_prehash<N, B>(
        pre_signature: &PreSignature<C>, pre_signature_participants: &ParticipantList<C>, nonce: N,
        msg: B, key_share: &C::Scalar, key_share_id: &NonZeroScalar<C>,
        key_share_participants: &ParticipantList<C>,
    ) -> EcdsaResult<Self>
    where
        B: AsRef<[u8]>,
        N: AsRef<[u8]>,
    {
        let z = scalar_hash::<C>(msg.as_ref());
        if z.is_zero().into() {
            return Err(EcdsaError::InvalidScalarZ);
        }

        Self::new_scalar(
            pre_signature, pre_signature_participants, nonce, z, key_share, key_share_id,
            key_share_participants,
        )
    }

    /// Create a new `SignatureShare` with the `pre_signature`, the secret key `x`,
    /// and the message `msg_digest`. `msg_digest` is the hash of the message.
    /// The nonce serves as a re-randomizer to prevent key recovery attacks as
    /// described in <https://eprint.iacr.org/2021/1330>
    pub fn new_digest<N, B>(
        pre_signature: &PreSignature<C>, pre_signature_participants: &ParticipantList<C>, nonce: N,
        msg_digest: B, key_share: &C::Scalar, key_share_id: &NonZeroScalar<C>,
        key_share_participants: &ParticipantList<C>,
    ) -> EcdsaResult<Self>
    where
        B: AsRef<[u8]>,
        N: AsRef<[u8]>,
    {
        let digest = msg_digest.as_ref();
        if digest.len() != C::FieldBytesSize::to_usize() {
            return Err(EcdsaError::InvalidScalarZ);
        }
        let z_bytes = <C::Scalar as Reduce<<C as Curve>::Uint>>::Bytes::from_slice(digest);
        let z = <C::Scalar as Reduce<<C as Curve>::Uint>>::reduce_bytes(z_bytes);

        Self::new_scalar(
            pre_signature, pre_signature_participants, nonce, z, key_share, key_share_id,
            key_share_participants,
        )
    }

    /// Create a new `SignatureShare` with the `pre_signature`, the secret key `x`,
    /// and the digest `z`. `z` is the hashed scalar form of the message.
    /// The nonce serves as a re-randomizer to prevent key recovery attacks as
    /// described in <https://eprint.iacr.org/2021/1330>
    pub fn new_scalar<N>(
        pre_signature: &PreSignature<C>, pre_signature_participants: &ParticipantList<C>, nonce: N,
        z: C::Scalar, key_share: &C::Scalar, key_share_id: &NonZeroScalar<C>,
        key_share_participants: &ParticipantList<C>,
    ) -> EcdsaResult<Self>
    where
        N: AsRef<[u8]>,
    {
        if !pre_signature.is_valid() {
            return Err(EcdsaError::InvalidPreSignature);
        }
        if !pre_signature_participants.set.iter().any(|p| p.as_ref() == &pre_signature.id) {
            return Err(EcdsaError::InvalidId);
        }
        if !key_share_participants.set.iter().any(|p| p.as_ref() == key_share_id.as_ref()) {
            return Err(EcdsaError::InvalidId);
        }
        if z.is_zero().into() {
            return Err(EcdsaError::InvalidScalarZ);
        }

        // Create a re-randomizer to prevent key recovery attacks
        // by calling hash_to_scalar with input nonce as defined in RFC9380
        // section 5 which also mitigates bias in the result.
        let re_randomizer =
            C::Scalar::create(nonce.as_ref(), b"lit-fast-ecdsa-pre-signature-rerandomizer-0.1.0");
        if re_randomizer.is_zero().into() {
            return Err(EcdsaError::InvalidScalarK);
        }
        let big_r = pre_signature.big_r + C::ProjectivePoint::generator() * re_randomizer;
        let r = x_coordinate::<C>(&big_r);
        if r.is_zero().into() {
            return Err(EcdsaError::InvalidSignatureShare);
        }

        // Invert errors if it is zero so this also checks for a zero re-randomizer
        let k_inv = Option::<C::Scalar>::from(
            (pre_signature.k_inv.invert().expect("k_inv should not be zero") + re_randomizer)
                .invert(),
        )
        .ok_or(EcdsaError::InvalidScalarK)?;

        let presig_id = Option::<NonZeroScalar<C>>::from(NonZeroScalar::new(pre_signature.id))
            .ok_or(EcdsaError::InvalidId)?;
        let presig_lagrange = lagrange(&presig_id, &pre_signature_participants.set);
        let share_lagrange = lagrange(key_share_id, &key_share_participants.set);

        let s = (k_inv * (r * key_share + z) * share_lagrange)
            + (z * pre_signature.d + pre_signature.e) * presig_lagrange;
        Ok(Self { r: big_r, s })
    }

    /// Check if the `SignatureShare` is valid
    pub fn is_valid(&self) -> bool {
        bool::from(!self.r.is_identity() & !self.s.is_zero())
    }

    /// Combine the signature shares into a signature
    /// Verify should be called after wards to check everything
    pub fn combine_into_signature(shares: &[SignatureShare<C>]) -> EcdsaResult<FullSignature<C>> {
        // Ensure non-empty shares
        if shares.is_empty() {
            return Err(EcdsaError::InsufficientShares);
        }
        // Check that all signature shares have the same r
        if shares[1..].iter().any(|s| s.r != shares[0].r) {
            return Err(EcdsaError::InvalidSignatureShare);
        }
        let sig_s = shares.iter().fold(C::Scalar::ZERO, |acc, s| acc + s.s);

        Ok(FullSignature { r: shares[0].r, s: sig_s })
    }
}

/// A PreSignature
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PreSignature<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    /// The participant id
    #[serde(with = "prime_field")]
    pub id: C::Scalar,
    /// The threshold number of participants
    pub threshold: usize,
    /// g^{k}
    #[serde(with = "group")]
    pub big_r: C::ProjectivePoint,
    /// The inverse of the nonce k
    #[serde(with = "prime_field")]
    pub k_inv: C::Scalar,
    /// A zero sharing
    #[serde(with = "prime_field")]
    pub d: C::Scalar,
    /// A zero sharing
    #[serde(with = "prime_field")]
    pub e: C::Scalar,
}

impl<C> ZeroizeOnDrop for PreSignature<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
}

impl<C> PreSignature<C>
where
    C: PrimeCurve + CurveArithmetic,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
{
    #[cfg(feature = "presign")]
    /// Only used for testing
    pub fn trusted_dealer_only_for_testing(
        participant_list: &ParticipantList<C>, threshold: usize, mut rng: impl RngCore + CryptoRng,
    ) -> EcdsaResult<Vec<PreSignature<C>>> {
        use lit_poly::DensePrimeField;

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

        let k = C::Scalar::random(&mut rng);
        let k_inv = Option::from(k.invert()).ok_or(EcdsaError::InvalidScalarK)?;
        let big_r = C::ProjectivePoint::generator() * k;

        let mut d_poly = DensePrimeField::random(threshold - 1, &mut rng);
        d_poly.0[0] = C::Scalar::ZERO;
        let mut e_poly = DensePrimeField::random(threshold - 1, &mut rng);
        e_poly.0[0] = C::Scalar::ZERO;

        let mut presignatures = Vec::with_capacity(participant_list.set.len());
        for id in &participant_list.set {
            let id = *(id.as_ref());
            let d = d_poly.evaluate(id);
            let e = e_poly.evaluate(id);
            presignatures.push(PreSignature { id, threshold, big_r, k_inv, d, e });
        }
        Ok(presignatures)
    }

    /// Check if this pre-signature is valid
    pub fn is_valid(&self) -> bool {
        self.threshold >= 2
            && bool::from(!self.id.is_zero() & !self.big_r.is_identity() & !self.k_inv.is_zero())
    }

    /// Get the tag of this pre-signature
    /// This will be identical for all participants that
    /// collaborated to create this pre-signature
    pub fn tag(&self) -> String {
        let mut hasher = sha2::Sha512_256::default();
        hasher.update(self.big_r.to_bytes().as_ref());
        hasher.update(self.k_inv.to_repr().as_ref());
        hex::encode(hasher.finalize_fixed())
    }
}
