use criterion::{Criterion, criterion_group, criterion_main};
use digest::generic_array::ArrayLength;
use ecdsa::PrimeCurve;
use ecdsa::elliptic_curve::{CurveArithmetic, FieldBytesSize, group::GroupEncoding};
use ecdsa::hazmat::DigestPrimitive;
use hd_keys_curves_wasm::{HDDerivable, HDDeriver};
use lit_fast_ecdsa::{
    ParticipantList, PreSignature, PreSignatureParticipant, PreSignatureRoundOutputGenerator,
    SignatureShare,
};
use lit_poly::DensePrimeField;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::collections::BTreeSet;
use std::ops::Add;

fn bench_p256(c: &mut Criterion) {
    let mut group = c.benchmark_group("p256");

    for &(t, n) in &[(6, 10), (12, 20), (20, 30)] {
        let participants_list = ParticipantList::new((1..=n).collect::<BTreeSet<_>>()).unwrap();
        let params = lit_fast_ecdsa::PreSignatureParams {
            threshold: t,
            participant_list: participants_list,
        };

        group.bench_function(format!("pre-signature {}-out-of-{}", t, n), |b| {
            b.iter(|| {
                let mut participants = (1..=n)
                    .map(|i| PreSignatureParticipant::<p256::NistP256>::new(i, &params).unwrap())
                    .collect::<Vec<_>>();
                let round1_generators = next_round(&mut participants);
                receive(&mut participants, &round1_generators);
                let round2_generators = next_round(&mut participants);
                receive(&mut participants, &round2_generators);
                let round3_generators = next_round(&mut participants);
                receive(&mut participants, &round3_generators);

                let mut presigs = Vec::with_capacity(n as usize);
                for participant in participants.iter_mut() {
                    let presig = participant.run().unwrap();
                    presigs.push(presig.output().unwrap());
                }
            })
        });

        let pre_signatures = PreSignature::<p256::NistP256>::trusted_dealer_only_for_testing(
            &params.participant_list,
            t,
            rand_chacha::ChaChaRng::from_entropy(),
        );
        let key_poly =
            DensePrimeField::random((t - 1) as usize, rand_chacha::ChaChaRng::from_entropy());
        let mut key_shares = (1..=n)
            .map(|id| (id, key_poly.evaluate(p256::Scalar::from(id as u64))))
            .collect::<Vec<_>>();

        group.bench_function(format!("Signature Share Generation {}-out-of-{}", t, n), |b| {
            b.iter(|| {
                key_shares.shuffle(&mut rand_chacha::ChaChaRng::from_entropy());
                let participant_list = ParticipantList::new(
                    key_shares.iter().take(t as usize).map(|(id, _)| *id).collect(),
                )
                .unwrap();
                SignatureShare::new_prehash(
                    &pre_signatures[&key_shares[0].0],
                    &participant_list,
                    format!("Signature Share Generation {}-out-of-{} nonce", t, n).as_bytes(),
                    format!("Signature Share Generation {}-out-of-{}", t, n).as_bytes(),
                    &key_shares[0].1,
                    key_shares[0].0,
                    &participant_list,
                )
                .unwrap();
            })
        });
    }
    group.finish();
}

fn receive<C>(
    participants: &mut [PreSignatureParticipant<C>],
    round_generators: &[PreSignatureRoundOutputGenerator<C>],
) where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    let len = participants.len();
    for i in 1..=len {
        let participant = participants.get_mut(i - 1).unwrap();
        for j in 1..=len {
            if i == j {
                continue;
            }
            let data = round_generators[j - 1].get(i).unwrap();
            participant.receive(&data).unwrap();
        }
    }
}

fn next_round<C>(
    participants: &mut [PreSignatureParticipant<C>],
) -> Vec<PreSignatureRoundOutputGenerator<C>>
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive,
    C::ProjectivePoint: GroupEncoding + HDDerivable,
    C::Scalar: HDDeriver,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    let mut round_generators = Vec::with_capacity(participants.len());
    for participant in participants {
        let round_generator = participant.run().unwrap();
        round_generators.push(round_generator);
    }
    round_generators
}

criterion_group!(benches, bench_p256);
criterion_main!(benches);
