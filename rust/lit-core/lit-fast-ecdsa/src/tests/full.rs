use crate::*;
use digest::generic_array::ArrayLength;
use ecdsa::{
    Signature,
    elliptic_curve::{
        AffinePoint, CurveArithmetic, FieldBytesSize, Group, PrimeCurve,
        group::{Curve, GroupEncoding},
        rand_core::SeedableRng,
        sec1::{self, FromEncodedPoint, ToEncodedPoint},
    },
    hazmat::{DigestPrimitive, VerifyPrimitive},
    signature::Verifier,
};
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};
use lit_poly::DensePrimeField;
use lit_rust_crypto::{k256, p256, p384};
use rand::seq::SliceRandom;
use rstest::*;
use std::{collections::HashMap, ops::Add, time::Instant};

#[test]
fn full_k256_with_pre_sigs() {
    const THRESHOLD: usize = 4;
    const SIGNERS: usize = 4;
    const NAME: &str = "k256";

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut participant_list_set = Vec::with_capacity(SIGNERS);
    for i in 1..=SIGNERS {
        participant_list_set.push(k256::Scalar::from(i as u64));
    }
    let participant_list = ParticipantList::new(&participant_list_set).unwrap();
    let params = PreSignatureParams { threshold: THRESHOLD, participant_list };
    let mut participants = Vec::with_capacity(SIGNERS);

    for i in 0..SIGNERS {
        let participant = PreSignatureParticipant::<k256::Secp256k1>::new(
            &params.participant_list.set[i], &params,
        )
        .unwrap();
        participants.push(participant);
    }

    let before = Instant::now();

    for _ in [Round::Round1, Round::Round2, Round::Round3].iter() {
        let round_generators = next_round(&mut participants);
        receive(&mut participants, &round_generators);
    }

    let mut presigs = Vec::with_capacity(SIGNERS);
    for participant in participants.iter_mut() {
        let presig = participant.run().unwrap();
        presigs.push(presig.output().unwrap());
    }
    println!(
        "{} - {} signers {} threshold, pre-signature generation took {:?}",
        NAME,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );

    presigs.shuffle(&mut rng);

    let key_poly = DensePrimeField::random(THRESHOLD - 1, &mut rng);
    let mut key_shares = (0..SIGNERS)
        .map(|i| {
            (params.participant_list.set[i].clone(), key_poly.evaluate(participant_list_set[i]))
        })
        .collect::<Vec<_>>();

    key_shares.shuffle(&mut rng);

    let presig_ids = presigs.iter().take(THRESHOLD).map(|p| p.id).collect::<Vec<_>>();
    let presig_participants = ParticipantList::new(presig_ids.as_slice()).unwrap();
    let key_share_ids = key_shares.iter().take(THRESHOLD).map(|(id, _)| *id).collect::<Vec<_>>();
    let key_share_participant_ids =
        ParticipantList { set: key_share_ids, _marker: std::marker::PhantomData };
    let before = Instant::now();
    let _ = SignatureShare::new_prehash(
        &presigs[0], &presig_participants, b"full test nonce", b"full test", &key_shares[0].1,
        &key_shares[0].0, &key_share_participant_ids,
    )
    .unwrap();
    println!(
        "{} - {} signers {} threshold, signature share generation took {:?}",
        NAME,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );

    let mut sig_shares = Vec::with_capacity(THRESHOLD);
    for i in 0..THRESHOLD {
        sig_shares.push(
            SignatureShare::new_prehash(
                &presigs[i], &presig_participants, b"full test nonce", b"full test",
                &key_shares[i].1, &key_shares[i].0, &key_share_participant_ids,
            )
            .unwrap(),
        );
    }
    let before = Instant::now();
    let signature = SignatureShare::<k256::Secp256k1>::combine_into_signature(&sig_shares).unwrap();
    println!(
        "{} - {} signers {} threshold, signature combination took {:?}",
        NAME,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );
    let y = k256::ProjectivePoint::GENERATOR * key_poly.0[0];
    assert!(signature.verify_prehash(b"full test", &y).is_ok());

    let signature: k256::ecdsa::Signature = signature.try_into().unwrap();
    let vk = k256::ecdsa::VerifyingKey::from_affine(y.to_affine()).unwrap();
    assert!(vk.verify(b"full test", &signature).is_ok());
}

#[test]
fn full_k256_run_sign() {
    const THRESHOLD: usize = 3;
    const SIGNERS: usize = 5;
    const NAME: &str = "k256";

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut participant_list_set = Vec::with_capacity(SIGNERS);
    for i in 1..=SIGNERS {
        participant_list_set.push(k256::Scalar::from(i as u64));
    }
    let participant_list = ParticipantList::new(participant_list_set.as_slice()).unwrap();
    let params = PreSignatureParams { threshold: THRESHOLD, participant_list };
    let mut participants = Vec::with_capacity(SIGNERS);

    for i in 0..SIGNERS {
        let participant = PreSignatureParticipant::<k256::Secp256k1>::new(
            &params.participant_list.set[i], &params,
        )
        .unwrap();
        participants.push(participant);
    }

    let key_poly = DensePrimeField::random(THRESHOLD - 1, &mut rng);
    let key_shares = (0..SIGNERS)
        .map(|i| {
            (params.participant_list.set[i].clone(), key_poly.evaluate(participant_list_set[i]))
        })
        .collect::<Vec<_>>();

    let before = Instant::now();
    for _ in [Round::Round1, Round::Round2].iter() {
        let round_generators = next_round(&mut participants);
        receive(&mut participants, &round_generators);
    }

    let mut sig_shares = Vec::with_capacity(SIGNERS);
    for (i, participant) in participants.iter().enumerate() {
        assert_eq!(key_shares[i].0.as_ref(), participant.id.as_ref());
        let sig_share = participant.run_sign(b"full run sign", &key_shares[i].1).unwrap();
        sig_shares.push(sig_share);
    }
    println!(
        "{} - {} signers {} threshold, pre-sign and sign generation took {:?}",
        NAME,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );

    let before = Instant::now();
    let full_signature =
        SignatureShare::<k256::Secp256k1>::combine_into_signature(&sig_shares).unwrap();
    println!(
        "{} - {} signers {} threshold, signature combination took {:?}",
        NAME,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );
    let y = k256::ProjectivePoint::GENERATOR * key_poly.0[0];
    assert!(full_signature.verify_prehash(b"full run sign", &y).is_ok());
    let signature: k256::ecdsa::Signature = full_signature.try_into().unwrap();
    let vk = k256::ecdsa::VerifyingKey::from_affine(y.to_affine()).unwrap();
    assert!(vk.verify(b"full run sign", &signature).is_ok());
}

#[rstest]
#[case::k256(k256::Secp256k1, "k256")]
#[case::p256(p256::NistP256, "p256")]
#[case::p384(p384::NistP384, "p384")]
fn full_run_sign<C>(#[case] _c: C, #[case] name: &str)
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C> + VerifyPrimitive<C>,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
    FieldBytesSize<C>: sec1::ModulusSize,
{
    const THRESHOLD: usize = 3;
    const SIGNERS: usize = 5;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut participant_list_set = Vec::with_capacity(SIGNERS);
    for i in 1..=SIGNERS {
        participant_list_set.push(C::Scalar::from(i as u64));
    }
    let participant_list = ParticipantList::new(participant_list_set.as_slice()).unwrap();
    let params =
        PreSignatureParams { threshold: THRESHOLD, participant_list: participant_list.clone() };
    let mut participants = Vec::with_capacity(SIGNERS);

    for i in 0..SIGNERS {
        let participant =
            PreSignatureParticipant::<C>::new(&participant_list.set[i], &params).unwrap();
        participants.push(participant);
    }

    let key_poly = DensePrimeField::random(THRESHOLD - 1, &mut rng);
    let key_shares = (0..SIGNERS)
        .map(|i| (participant_list.set[i], key_poly.evaluate(participant_list_set[i])))
        .collect::<Vec<_>>();

    let before = Instant::now();
    for _ in &[Round::Round1, Round::Round2] {
        let round_generators = next_round(&mut participants);
        receive(&mut participants, &round_generators);
    }

    let mut sig_shares = Vec::with_capacity(THRESHOLD);
    for (i, participant) in participants.iter().enumerate() {
        assert_eq!(key_shares[i].0.as_ref(), participant.id.as_ref());
        let sig_share = participant.run_sign(b"full run sign", &key_shares[i].1).unwrap();
        sig_shares.push(sig_share);
    }
    println!(
        "{} - {} signers {} threshold, pre-sign and sign generation took {:?}",
        name,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );

    let before = Instant::now();
    let signature = SignatureShare::<C>::combine_into_signature(&sig_shares).unwrap();
    println!(
        "{} - {} signers {} threshold, signature combination took {:?}",
        name,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );
    let y = C::ProjectivePoint::generator() * key_poly.0[0];
    assert!(signature.verify_prehash(b"full run sign", &y).is_ok());

    let signature: Signature<C> = signature.try_into().unwrap();
    let vk = ecdsa::VerifyingKey::<C>::from_affine(y.to_affine()).unwrap();
    assert!(
        <ecdsa::VerifyingKey<C> as Verifier<Signature<C>>>::verify(
            &vk, b"full run sign", &signature
        )
        .is_ok()
    );
}

#[rstest]
#[case::k256(k256::Secp256k1, "k256")]
#[case::p256(p256::NistP256, "p256")]
#[case::p384(p384::NistP384, "p384")]
fn full_run_pre_sigs<C>(#[case] _c: C, #[case] name: &str)
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C> + VerifyPrimitive<C>,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
    FieldBytesSize<C>: sec1::ModulusSize,
{
    const THRESHOLD: usize = 3;
    const SIGNERS: usize = 5;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut participant_list_set = Vec::with_capacity(SIGNERS);
    for i in 1..=SIGNERS {
        participant_list_set.push(C::Scalar::from(i as u64));
    }
    let participant_list = ParticipantList::new(participant_list_set.as_slice()).unwrap();
    let params = PreSignatureParams { threshold: THRESHOLD, participant_list };
    let mut participants = Vec::with_capacity(SIGNERS);

    for i in 0..SIGNERS {
        let participant =
            PreSignatureParticipant::<C>::new(&params.participant_list.set[i], &params).unwrap();
        participants.push(participant);
    }

    let before = Instant::now();
    for _ in &[Round::Round1, Round::Round2, Round::Round3] {
        let round_generators = next_round(&mut participants);
        receive(&mut participants, &round_generators);
    }

    let mut presigs = Vec::with_capacity(SIGNERS);
    for participant in participants.iter_mut() {
        let presig = participant.run().unwrap();
        presigs.push(presig.output().unwrap());
    }
    println!(
        "{} - {} signers {} threshold, presignature generation took {:?}",
        name,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );

    presigs.shuffle(&mut rng);

    let key_poly = DensePrimeField::random(THRESHOLD - 1, &mut rng);
    let mut key_shares = (0..SIGNERS)
        .map(|i| {
            (
                params.participant_list.set[i].clone(),
                key_poly.evaluate(participant_list_set[i].clone()),
            )
        })
        .collect::<Vec<_>>();

    key_shares.shuffle(&mut rng);

    let presig_ids = presigs.iter().take(THRESHOLD).map(|p| p.id).collect::<Vec<_>>();
    let presig_participants = ParticipantList::new(presig_ids.as_slice()).unwrap();
    let key_share_ids = key_shares.iter().take(THRESHOLD).map(|(id, _)| *id).collect::<Vec<_>>();
    let key_share_participant_ids =
        ParticipantList { set: key_share_ids, _marker: std::marker::PhantomData };
    let before = Instant::now();
    let _ = SignatureShare::new_prehash(
        &presigs[0], &presig_participants, b"full test nonce", b"full test", &key_shares[0].1,
        &key_shares[0].0, &key_share_participant_ids,
    )
    .unwrap();
    println!(
        "{} - {} signers {} threshold, signature share generation took {:?}",
        name,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );

    let mut sig_shares = Vec::with_capacity(THRESHOLD);
    for i in 0..THRESHOLD {
        sig_shares.push(
            SignatureShare::new_prehash(
                &presigs[i], &presig_participants, b"full test nonce", b"full test",
                &key_shares[i].1, &key_shares[i].0, &key_share_participant_ids,
            )
            .unwrap(),
        );
    }
    let before = Instant::now();
    let full_signature = SignatureShare::<C>::combine_into_signature(&sig_shares).unwrap();
    println!(
        "{} - {} signers {} threshold, signature combination took {:?}",
        name,
        SIGNERS,
        THRESHOLD,
        before.elapsed()
    );
    let y = C::ProjectivePoint::generator() * key_poly.0[0];
    assert!(full_signature.verify_prehash(b"full test", &y).is_ok());

    let signature: Signature<C> = full_signature.try_into().unwrap();
    let vk = ecdsa::VerifyingKey::<C>::from_affine(y.to_affine()).unwrap();
    assert!(
        <ecdsa::VerifyingKey<C> as Verifier<Signature<C>>>::verify(&vk, b"full test", &signature)
            .is_ok()
    );
}

pub(crate) fn receive<C>(
    participants: &mut [PreSignatureParticipant<C>],
    round_outputs: &HashMap<usize, Vec<PreSignatureRoundOutput<C>>>,
) where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    for participant in participants {
        for PreSignatureRoundOutput { ordinal: _, id, round_payload } in
            round_outputs.get(&participant.ordinal).unwrap()
        {
            assert_eq!(id.as_ref(), participant.id.as_ref());
            let res = participant.receive(round_payload.clone());
            assert!(res.is_ok());
        }
    }
}

pub(crate) fn next_round<C>(
    participants: &mut [PreSignatureParticipant<C>],
) -> HashMap<usize, Vec<PreSignatureRoundOutput<C>>>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    let len = participants.len();
    let mut outputs = HashMap::with_capacity(len);

    for participant in participants {
        let round_generator = participant.run().unwrap();
        for output in round_generator.iter() {
            outputs
                .entry(output.ordinal)
                .and_modify(|v: &mut Vec<PreSignatureRoundOutput<C>>| v.push(output.clone()))
                .or_insert({
                    let mut v = Vec::with_capacity(len);
                    v.push(output);
                    v
                });
        }
    }

    outputs
}
