use crate::{
    ParticipantList, PreSignature, PreSignatureParams, PreSignatureParticipant, Round,
    SignatureShare, utils::lagrange,
};
use ecdsa::elliptic_curve::{NonZeroScalar, rand_core::SeedableRng};
use ecdsa::signature::Verifier;
use lit_poly::DensePrimeField;
use lit_rust_crypto::k256;

#[test]
fn lowest_threshold_trusted_dealer() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    let participant_list = ParticipantList::new(&[
        k256::Scalar::from(1u64),
        k256::Scalar::from(2u64),
        k256::Scalar::from(3u64),
    ])
    .unwrap();

    let poly = DensePrimeField::<k256::Scalar>::random(2, &mut rng);
    let mut key_shares = vec![k256::Scalar::ZERO; 3];
    for (share, id) in key_shares.iter_mut().zip(participant_list.set.iter()) {
        let id = NonZeroScalar::<k256::Secp256k1>::as_ref(id);
        *share = poly.evaluate(*id);
    }

    let pre_signatures = PreSignature::<k256::Secp256k1>::trusted_dealer_only_for_testing(
        &participant_list, 3, &mut rng,
    )
    .unwrap();
    let nonce = b"lowest_threshold_nonce";
    let msg = b"lowest_threshold";

    let signing_participant_list =
        ParticipantList::new(&[pre_signatures[0].id, pre_signatures[1].id, pre_signatures[2].id])
            .unwrap();

    assert_eq!(&pre_signatures[0].id, participant_list.set[0].as_ref());
    let sig_share1 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[0], &signing_participant_list, &nonce, &msg, &key_shares[0],
        &participant_list.set[0], &signing_participant_list,
    )
    .unwrap();

    assert_eq!(&pre_signatures[1].id, participant_list.set[1].as_ref());
    let sig_share2 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[1], &signing_participant_list, &nonce, &msg, &key_shares[1],
        &participant_list.set[1], &signing_participant_list,
    )
    .unwrap();

    assert_eq!(&pre_signatures[2].id, participant_list.set[2].as_ref());
    let sig_share3 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[2], &signing_participant_list, &nonce, &msg, &key_shares[2],
        &participant_list.set[2], &signing_participant_list,
    )
    .unwrap();

    let pk = k256::ProjectivePoint::GENERATOR * poly.0[0];
    let full_signature = SignatureShare::<k256::Secp256k1>::combine_into_signature(&[
        sig_share1, sig_share2, sig_share3,
    ])
    .unwrap();
    assert!(full_signature.verify_prehash(msg, &pk).is_ok());

    let signature: k256::ecdsa::Signature = full_signature.try_into().unwrap();
    let vk = k256::ecdsa::VerifyingKey::from_affine(pk.to_affine()).unwrap();
    assert!(vk.verify(msg, &signature).is_ok());
}

#[test]
fn lowest_threshold_distributed() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    let participant_list = ParticipantList::new(&[
        k256::Scalar::from(1u64),
        k256::Scalar::from(2u64),
        k256::Scalar::from(3u64),
    ])
    .unwrap();

    let poly = DensePrimeField::<k256::Scalar>::random(2, &mut rng);
    let mut key_shares = vec![k256::Scalar::ZERO; 3];
    for (share, id) in key_shares.iter_mut().zip(participant_list.set.iter()) {
        let id = NonZeroScalar::<k256::Secp256k1>::as_ref(id);
        *share = poly.evaluate(*id);
    }

    let params = PreSignatureParams { threshold: 2, participant_list };
    let mut participants = Vec::with_capacity(3);

    for i in 0..3 {
        let participant = PreSignatureParticipant::<k256::Secp256k1>::new(
            &params.participant_list.set[i], &params,
        )
        .unwrap();
        participants.push(participant);
    }

    for _ in [Round::Round1, Round::Round2, Round::Round3].iter() {
        let round_generators = crate::tests::full::next_round(&mut participants);
        crate::tests::full::receive(&mut participants, &round_generators);
    }

    let mut pre_signatures = Vec::with_capacity(3);
    for participant in participants.iter_mut() {
        let presig = participant.run().unwrap();
        pre_signatures.push(presig.output().unwrap());
    }

    let nonce = b"lowest_threshold_nonce";
    let msg = b"lowest_threshold";

    let signing_participant_list =
        ParticipantList::new(&[pre_signatures[0].id, pre_signatures[1].id, pre_signatures[2].id])
            .unwrap();

    assert_eq!(&pre_signatures[0].id, params.participant_list.set[0].as_ref());
    let sig_share1 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[0], &signing_participant_list, &nonce, &msg, &key_shares[0],
        &params.participant_list.set[0], &signing_participant_list,
    )
    .unwrap();

    assert_eq!(&pre_signatures[1].id, params.participant_list.set[1].as_ref());
    let sig_share2 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[1], &signing_participant_list, &nonce, &msg, &key_shares[1],
        &params.participant_list.set[1], &signing_participant_list,
    )
    .unwrap();

    let sig_share3 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[2], &signing_participant_list, &nonce, &msg, &key_shares[2],
        &params.participant_list.set[2], &signing_participant_list,
    )
    .unwrap();

    let pk = k256::ProjectivePoint::GENERATOR * poly.0[0];
    let full_signature = SignatureShare::<k256::Secp256k1>::combine_into_signature(&[
        sig_share1, sig_share2, sig_share3,
    ])
    .unwrap();
    assert!(full_signature.verify_prehash(msg, &pk).is_ok());

    let signature: k256::ecdsa::Signature = full_signature.try_into().unwrap();
    let vk = k256::ecdsa::VerifyingKey::from_affine(pk.to_affine()).unwrap();
    assert!(vk.verify(msg, &signature).is_ok());
}

#[test]
fn sign_with_re_randomizer() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    let participant_list = ParticipantList::new(&[
        k256::Scalar::from(1u64),
        k256::Scalar::from(2u64),
        k256::Scalar::from(3u64),
        k256::Scalar::from(4u64),
        k256::Scalar::from(5u64),
    ])
    .unwrap();

    let poly = DensePrimeField::<k256::Scalar>::random(1, &mut rng);
    let mut key_shares = vec![k256::Scalar::ZERO; 3];
    for (share, id) in key_shares.iter_mut().zip(participant_list.set.iter()) {
        let id = NonZeroScalar::<k256::Secp256k1>::as_ref(id);
        *share = poly.evaluate(*id);
    }

    let pre_signatures = PreSignature::<k256::Secp256k1>::trusted_dealer_only_for_testing(
        &participant_list, 3, &mut rng,
    )
    .unwrap();
    let nonce = b"sign_with_rerandomizer_nonce";
    let msg = b"sign_with_rerandomizer";

    let signing_participant_list =
        ParticipantList::new(&[pre_signatures[0].id, pre_signatures[1].id, pre_signatures[2].id])
            .unwrap();

    assert_eq!(&pre_signatures[0].id, participant_list.set[0].as_ref());
    let sig_share1 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[0], &signing_participant_list, &nonce, &msg, &key_shares[0],
        &participant_list.set[0], &signing_participant_list,
    )
    .unwrap();

    assert_eq!(&pre_signatures[1].id, participant_list.set[1].as_ref());
    let sig_share2 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[1], &signing_participant_list, &nonce, &msg, &key_shares[1],
        &participant_list.set[1], &signing_participant_list,
    )
    .unwrap();

    assert_eq!(&pre_signatures[2].id, participant_list.set[2].as_ref());
    let sig_share3 = SignatureShare::<k256::Secp256k1>::new_prehash(
        &pre_signatures[2], &signing_participant_list, &nonce, &msg, &key_shares[2],
        &participant_list.set[2], &signing_participant_list,
    )
    .unwrap();

    let pk = k256::ProjectivePoint::GENERATOR * poly.0[0];
    let full_signature = SignatureShare::<k256::Secp256k1>::combine_into_signature(&[
        sig_share1, sig_share2, sig_share3,
    ])
    .unwrap();
    assert!(full_signature.verify_prehash(msg, &pk).is_ok());

    let signature: k256::ecdsa::Signature = full_signature.try_into().unwrap();
    let vk = k256::ecdsa::VerifyingKey::from_affine(pk.to_affine()).unwrap();
    assert!(vk.verify(msg, &signature).is_ok());
}

#[test]
fn poly_reduce() {
    const SIGNERS: usize = 20;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut a_polys = Vec::with_capacity(SIGNERS);
    let mut k_polys = Vec::with_capacity(SIGNERS);
    let mut b_polys = Vec::with_capacity(SIGNERS);

    let mut a = k256::Scalar::ZERO;
    let mut k = k256::Scalar::ZERO;

    let mut ids = Vec::with_capacity(SIGNERS);

    let min_threshold = std::cmp::max((SIGNERS - 1) >> 1, 2);
    for i in 0..SIGNERS {
        let a_poly = DensePrimeField::<k256::Scalar>::random(min_threshold - 1, &mut rng);
        let k_poly = DensePrimeField::<k256::Scalar>::random(min_threshold - 1, &mut rng);
        let mut b_poly = DensePrimeField::<k256::Scalar>::random(SIGNERS - 1, &mut rng);
        b_poly.0[0] = k256::Scalar::ZERO;

        a += a_poly.0[0];
        k += k_poly.0[0];

        a_polys.push(a_poly);
        k_polys.push(k_poly);
        b_polys.push(b_poly);

        ids.push(NonZeroScalar::new(k256::Scalar::from((i + 1) as u64)).unwrap());
    }

    let mut a_shares = Vec::with_capacity(SIGNERS);
    let mut k_shares = Vec::with_capacity(SIGNERS);
    let mut b_shares = Vec::with_capacity(SIGNERS);
    for i in 0..SIGNERS {
        a_shares.push(
            ids.iter()
                .map(|id| {
                    let id = NonZeroScalar::<k256::Secp256k1>::as_ref(id);
                    a_polys[i].evaluate(*id)
                })
                .collect::<Vec<_>>(),
        );
        k_shares.push(
            ids.iter()
                .map(|id| {
                    let id = NonZeroScalar::<k256::Secp256k1>::as_ref(id);
                    k_polys[i].evaluate(*id)
                })
                .collect::<Vec<_>>(),
        );
        b_shares.push(
            ids.iter()
                .map(|id| {
                    let id = NonZeroScalar::<k256::Secp256k1>::as_ref(id);
                    b_polys[i].evaluate(*id)
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut a_s = Vec::with_capacity(SIGNERS);
    let mut k_s = Vec::with_capacity(SIGNERS);
    let mut b_s = Vec::with_capacity(SIGNERS);

    for i in 0..SIGNERS {
        let mut aa = k256::Scalar::ZERO;
        let mut kk = k256::Scalar::ZERO;
        let mut bb = k256::Scalar::ZERO;
        for j in 0..SIGNERS {
            aa += a_shares[j][i];
            kk += k_shares[j][i];
            bb += b_shares[j][i];
        }
        a_s.push(aa);
        k_s.push(kk);
        b_s.push(bb);
    }
    let mut big_r = k256::ProjectivePoint::IDENTITY;

    let mut w = k256::Scalar::ZERO;
    for i in 0..SIGNERS {
        let lambda = lagrange(&ids[i], &ids);
        w += (a_s[i] * k_s[i] + b_s[i]) * lambda;
        big_r += k256::ProjectivePoint::GENERATOR * k_s[i] * lambda;
    }
    assert_eq!(w, a * k);
    assert_eq!(k256::ProjectivePoint::GENERATOR * w, k256::ProjectivePoint::GENERATOR * a * k);
    assert_eq!(big_r, k256::ProjectivePoint::GENERATOR * k);
    let mut big_w = k256::ProjectivePoint::IDENTITY;
    for i in 0..SIGNERS {
        let lambda = lagrange(&ids[i], &ids);
        big_w += big_r * a_s[i] * lambda;
    }

    assert_eq!(big_w.to_affine(), (big_r * a).to_affine());
    assert_eq!(big_w, k256::ProjectivePoint::GENERATOR * w);
}
