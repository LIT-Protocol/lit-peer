use crate::*;
use elliptic_curve_tools::{group, prime_field, prime_field_vec};
use lit_rust_crypto::elliptic_curve::{
    Field, Group, PrimeField, group::GroupEncoding, subtle::Choice,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap},
    fmt::{self, Display, Formatter},
};

/// EC-VRF proof
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Proof<G>
where
    G: Group + GroupEncoding + Default,
{
    /// Gamma
    #[serde(with = "group")]
    pub gamma: G,
    /// The x-coordinate of gamma
    #[serde(with = "prime_field")]
    pub gamma_x: G::Scalar,
    /// c - challenge
    #[serde(with = "prime_field")]
    pub c: G::Scalar,
    /// s - schnorr proof
    #[serde(with = "prime_field")]
    pub s: G::Scalar,
    /// VRF hash output
    #[serde(with = "prime_field")]
    pub beta: G::Scalar,
}

impl<G> Proof<G>
where
    G: Group + GroupEncoding + Default,
{
    /// Check if the proof contains valid data
    pub(crate) fn is_invalid(&self) -> Choice {
        self.gamma.is_identity() | self.c.is_zero() | self.s.is_zero() | self.beta.is_zero()
    }

    /// Combine multiple proofs into a value according to
    /// <https://eprint.iacr.org/2023/312.pdf> section A.2.
    /// The input are id-proof pairs
    pub fn combine(proofs: &[(G::Scalar, Self)]) -> VrfResult<G> {
        let mut ids = BTreeSet::<Vec<u8>>::new();
        let mut lagrange_ids = Vec::with_capacity(proofs.len());
        for (id, _) in proofs {
            let bytes = id.to_repr().as_ref().to_vec();
            if !ids.insert(bytes) {
                return Err(VrfError::DuplicateId);
            }
            if id.is_zero().into() {
                return Err(VrfError::InvalidProofId);
            }
            lagrange_ids.push(*id);
        }
        Ok(proofs.iter().fold(G::default(), |acc, &(id, proof)| {
            let coefficient = lagrange(id, &lagrange_ids);
            acc + proof.gamma * coefficient
        }))
    }
}

/// Facilitates creating a distributed VRF with compact proofs
/// See <https://eprint.iacr.org/2024/1130.pdf> for more details.
/// These are the parameters that are shared among all participants
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AggregatedProofParams<G>
where
    G: Group + GroupEncoding + Default,
{
    #[serde(with = "group")]
    pub pk: G,
    #[serde(with = "prime_field")]
    pub alpha: G::Scalar,
    #[serde(with = "prime_field_vec")]
    pub participants: Vec<G::Scalar>,
}

impl<G> AggregatedProofParams<G>
where
    G: Group + GroupEncoding + Default,
{
    pub(crate) fn is_invalid(&self) -> bool {
        bool::from(self.pk.is_identity() | self.alpha.is_zero()) || self.participants.is_empty()
    }
}

/// The current Aggregate Proof Round
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Round {
    #[default]
    One,
    Two,
    Three,
    Four,
}

macro_rules! impl_round_to_int {
    ($($ident:ident),+$(,)*) => {
        $(
            impl From<Round> for $ident {
                fn from(value: Round) -> Self {
                    match value {
                        Round::One => 1,
                        Round::Two => 2,
                        Round::Three => 3,
                        Round::Four => 4,
                    }
                }
            }

            impl TryFrom<$ident> for Round {
                type Error = String;

                fn try_from(value: $ident) -> Result<Self, Self::Error> {
                    match value {
                        1 => Ok(Round::One),
                        2 => Ok(Round::Two),
                        3 => Ok(Round::Three),
                        4 => Ok(Round::Four),
                        _ => Err(format!("Invalid round: {}", value)),
                    }
                }
            }
        )+
    };
}

impl_round_to_int!(u8, u16, u32, u128, usize);

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
        }
    }
}

/// The Aggregated Proof Round Data
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregatedProofRoundData<G>
where
    G: Group + GroupEncoding + Default,
{
    Round1(AggregatedProofOutputRound1<G>),
    Round2(AggregatedProofOutputRound2<G>),
    Round3(Proof<G>),
}

/// Facilitates creating a distributed VRF with compact proofs
/// See <https://eprint.iacr.org/2024/1130.pdf> for more details.
/// The Aggregated Proof Builder
#[derive(Debug, Clone, Default)]
pub struct AggregatedProofBuilder<V>
where
    V: VrfProver,
{
    id: <V::Group as Group>::Scalar,
    ordinal: usize,
    round: Round,
    participants: HashMap<usize, <V::Group as Group>::Scalar>,
    lagrange: HashMap<usize, <V::Group as Group>::Scalar>,
    received_round1: HashMap<usize, AggregatedProofOutputRound1<V::Group>>,
    received_round2: HashMap<usize, AggregatedProofOutputRound2<V::Group>>,
    sk: <V::Group as Group>::Scalar,
    pk: V::Group,
    h: V::Group,
    gamma: V::Group,
    k: <V::Group as Group>::Scalar,
    gk: V::Group,
    hk: V::Group,
    c: <V::Group as Group>::Scalar,
    s: <V::Group as Group>::Scalar,
    beta: <V::Group as Group>::Scalar,
}

impl<V> AggregatedProofBuilder<V>
where
    V: VrfProver,
{
    /// Create a new AggregatedProofBuilder
    pub fn new(
        my_id: &<V::Group as Group>::Scalar, key_share: &<V::Group as Group>::Scalar,
        params: &AggregatedProofParams<V::Group>,
    ) -> VrfResult<Self> {
        if params.is_invalid() {
            return Err(VrfError::InvalidProofParams);
        }

        let mut index = None;
        let mut lagrange_values = HashMap::with_capacity(params.participants.len());
        let mut participants_map = HashMap::with_capacity(params.participants.len());
        for (i, &d) in params.participants.iter().enumerate() {
            let coefficient = lagrange(d, &params.participants);
            lagrange_values.insert(i, coefficient);
            if *my_id == d {
                index = Some(i);
            }
            participants_map.insert(i, d);
        }

        if index.is_none() {
            return Err(VrfError::InvalidProofId);
        }

        let h = V::hash_to_curve(&params.alpha);
        if h.is_identity().into() {
            return Err(VrfError::HashToCurveError);
        }
        let k = V::generate_nonce(key_share, &params.alpha);
        if k.is_zero().into() {
            return Err(VrfError::NonceGenerationError);
        }
        let gamma = h * *key_share;
        let gk = <V::Group as Group>::generator() * k;
        let hk = h * k;

        let mut received_round1 = HashMap::with_capacity(params.participants.len());
        let received_round2 = HashMap::with_capacity(params.participants.len());

        let index = index.unwrap();

        received_round1
            .insert(index, AggregatedProofOutputRound1 { id: *my_id, index, gamma, gk, hk });

        Ok(Self {
            id: *my_id,
            ordinal: index,
            pk: params.pk,
            round: Round::One,
            participants: participants_map,
            lagrange: lagrange_values,
            sk: *key_share,
            h,
            k,
            gamma,
            gk,
            hk,
            c: <V::Group as Group>::Scalar::ZERO,
            s: <V::Group as Group>::Scalar::ZERO,
            beta: <V::Group as Group>::Scalar::ZERO,
            received_round1,
            received_round2,
        })
    }

    pub fn run(&mut self) -> VrfResult<AggregatedProofRoundData<V::Group>> {
        match self.round {
            Round::One => self.round1().map(AggregatedProofRoundData::Round1),
            Round::Two => self.round2().map(AggregatedProofRoundData::Round2),
            Round::Three => self.round3().map(AggregatedProofRoundData::Round3),
            Round::Four => Ok(AggregatedProofRoundData::Round3(Proof {
                gamma: self.gamma,
                gamma_x: V::point_to_scalar(self.gamma),
                c: self.c,
                s: self.s,
                beta: self.beta,
            })),
        }
    }

    pub(crate) fn round1(&self) -> VrfResult<AggregatedProofOutputRound1<V::Group>> {
        if self.round != Round::One {
            return Err(VrfError::InvalidProofRound);
        }
        Ok(AggregatedProofOutputRound1 {
            id: self.id,
            index: self.ordinal,
            gamma: self.gamma,
            gk: self.gk,
            hk: self.hk,
        })
    }

    pub(crate) fn round2(&mut self) -> VrfResult<AggregatedProofOutputRound2<V::Group>> {
        if self.round != Round::Two {
            return Err(VrfError::InvalidProofRound);
        }
        // Check if all participants have sent their round1 commitments
        if self.received_round1.len() != self.participants.len() {
            return Err(VrfError::InvalidProofInput);
        }
        if let Some(proof) = self.received_round2.get(&self.ordinal) {
            return Ok(*proof);
        }

        let mut gk = V::Group::identity();
        let mut hk = V::Group::identity();
        let mut gamma = V::Group::identity();

        for (idx, proof) in self.received_round1.iter() {
            let coefficient = self.lagrange[idx];
            gamma += proof.gamma * coefficient;
            gk += proof.gk * coefficient;
            hk += proof.hk * coefficient;
        }
        let c = V::generate_challenge(&[V::Group::generator(), self.h, self.pk, gamma, gk, hk]);
        if c.is_zero().into() {
            return Err(VrfError::ChallengeGenerationError);
        }

        self.gamma = gamma;
        self.hk = hk;
        self.gk = gk;
        self.c = c;
        self.s = self.k - c * self.sk;
        self.received_round2.insert(
            self.ordinal,
            AggregatedProofOutputRound2 { id: self.id, ordinal: self.ordinal, c, s: self.s },
        );
        Ok(AggregatedProofOutputRound2 { id: self.id, ordinal: self.ordinal, c, s: self.s })
    }

    pub(crate) fn round3(&mut self) -> VrfResult<Proof<V::Group>> {
        if self.round != Round::Three {
            return Err(VrfError::InvalidProofRound);
        }
        // Check if all participants have sent their round2 outputs
        if self.received_round2.len() != self.participants.len() {
            return Err(VrfError::InvalidProofInput);
        }

        let mut s = <V::Group as Group>::Scalar::ZERO;
        for (idx, proof) in self.received_round2.iter() {
            let coefficient = self.lagrange[idx];
            s += proof.s * coefficient;
        }

        self.round = Round::Four;
        self.s = s;

        self.beta = V::proof_to_hash(self.gamma);
        let gamma_x = V::point_to_scalar(self.gamma);
        Ok(Proof { gamma: self.gamma, gamma_x, c: self.c, s: self.s, beta: self.beta })
    }

    /// Receive a proof from a participant
    pub fn receive(&mut self, proof: AggregatedProofRoundData<V::Group>) -> VrfResult<()> {
        match proof {
            AggregatedProofRoundData::Round1(proof) => {
                if self.round != Round::One {
                    return Err(VrfError::InvalidProofRound);
                }
                if proof.is_invalid().into() {
                    return Err(VrfError::InvalidProofInput);
                }
                if self.received_round1.contains_key(&proof.index) {
                    return Err(VrfError::DuplicateId);
                }
                self.received_round1.insert(proof.index, proof);
                if self.received_round1.len() == self.participants.len() {
                    self.round = Round::Two;
                }
            }
            AggregatedProofRoundData::Round2(proof) => {
                if self.round != Round::Two {
                    return Err(VrfError::InvalidProofRound);
                }
                if proof.is_invalid().into() {
                    return Err(VrfError::InvalidProofInput);
                }
                if self.received_round2.contains_key(&proof.ordinal) {
                    return Err(VrfError::DuplicateId);
                }
                self.received_round2.insert(proof.ordinal, proof);
                if self.received_round2.len() == self.participants.len() {
                    self.round = Round::Three;
                }
            }
            AggregatedProofRoundData::Round3(proof) => {
                if self.round != Round::Four {
                    return Err(VrfError::InvalidProofRound);
                }
                if proof.is_invalid().into() {
                    return Err(VrfError::InvalidProofInput);
                }
                if proof.gamma != self.gamma
                    || proof.c != self.c
                    || proof.s != self.s
                    || proof.beta != self.beta
                {
                    return Err(VrfError::InvalidProofInput);
                }
            }
        }
        Ok(())
    }
}

/// The Aggregated Proof Output Round 1
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AggregatedProofOutputRound1<G>
where
    G: Group + GroupEncoding + Default,
{
    #[serde(with = "prime_field")]
    pub id: G::Scalar,
    pub index: usize,
    #[serde(with = "group")]
    pub gamma: G,
    #[serde(with = "group")]
    pub gk: G,
    #[serde(with = "group")]
    pub hk: G,
}

impl<G> AggregatedProofOutputRound1<G>
where
    G: Group + GroupEncoding + Default,
{
    /// Check if the proof contains valid data
    pub(crate) fn is_invalid(&self) -> Choice {
        self.id.is_zero() | self.gamma.is_identity() | self.gk.is_identity() | self.hk.is_identity()
    }
}

/// The Aggregated Proof Output Round 2
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AggregatedProofOutputRound2<G>
where
    G: Group + GroupEncoding + Default,
{
    #[serde(with = "prime_field")]
    pub id: G::Scalar,
    pub ordinal: usize,
    #[serde(with = "prime_field")]
    pub c: G::Scalar,
    #[serde(with = "prime_field")]
    pub s: G::Scalar,
}

impl<G> AggregatedProofOutputRound2<G>
where
    G: Group + GroupEncoding + Default,
{
    /// Check if the proof contains valid data
    pub fn is_invalid(&self) -> Choice {
        self.id.is_zero() | self.c.is_zero() | self.s.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::{ProjectivePoint, Scalar, Secp256k1};
    use lit_rust_crypto::{k256, vsss_rs};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use vsss_rs::{DefaultShare, IdentifierPrimeField, ValuePrimeField, shamir};
    type SecretShare = DefaultShare<IdentifierPrimeField<Scalar>, ValuePrimeField<Scalar>>;

    #[test]
    fn combine_test() {
        let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
        let sk = Scalar::random(&mut rng);
        let msg = Scalar::random(&mut rng);
        let shares = shamir::split_secret::<SecretShare>(3, 5, &(sk.into()), &mut rng).unwrap();

        let proofs = shares
            .iter()
            .map(|share| {
                let proof = Secp256k1::vrf_prove(&share.value.0, &msg, None).unwrap();
                (share.identifier.0, proof)
            })
            .collect::<Vec<_>>();

        let combined1 = Proof::combine(&proofs[..3]).unwrap();
        let combined2 = Proof::combine(&proofs[1..4]).unwrap();
        let combined3 = Proof::combine(&proofs[2..]).unwrap();

        assert_eq!(combined1, combined2);
        assert_eq!(combined2, combined3);
    }

    #[test]
    fn builder_test() {
        let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
        let sk = Scalar::random(&mut rng);
        let pk = ProjectivePoint::GENERATOR * sk;
        let msg = Scalar::random(&mut rng);
        let shares = shamir::split_secret::<SecretShare>(3, 5, &(sk.into()), &mut rng).unwrap();

        let participants = shares.iter().take(3).map(|x| x.identifier.0).collect::<Vec<_>>();
        let params = AggregatedProofParams { pk, alpha: msg, participants };
        let mut builder1 = AggregatedProofBuilder::<Secp256k1>::new(
            &shares[0].identifier.0, &shares[0].value.0, &params,
        )
        .unwrap();
        let mut builder2 = AggregatedProofBuilder::<Secp256k1>::new(
            &shares[1].identifier.0, &shares[1].value.0, &params,
        )
        .unwrap();
        let mut builder3 = AggregatedProofBuilder::<k256::Secp256k1>::new(
            &shares[2].identifier.0, &shares[2].value.0, &params,
        )
        .unwrap();

        let round1_1 = builder1.run().unwrap();
        let round1_2 = builder2.run().unwrap();
        let round1_3 = builder3.run().unwrap();

        builder1.receive(round1_2.clone()).unwrap();
        builder1.receive(round1_3.clone()).unwrap();

        builder2.receive(round1_1.clone()).unwrap();
        builder2.receive(round1_3.clone()).unwrap();

        builder3.receive(round1_1.clone()).unwrap();
        builder3.receive(round1_2.clone()).unwrap();

        let round2_1 = builder1.run().unwrap();
        let round2_2 = builder2.run().unwrap();
        let round2_3 = builder3.run().unwrap();

        builder1.receive(round2_2.clone()).unwrap();
        builder1.receive(round2_3.clone()).unwrap();

        builder2.receive(round2_1.clone()).unwrap();
        builder2.receive(round2_3.clone()).unwrap();

        builder3.receive(round2_1.clone()).unwrap();
        builder3.receive(round2_2.clone()).unwrap();

        let round3_1 = builder1.run().unwrap();
        let round3_2 = builder2.run().unwrap();
        let round3_3 = builder3.run().unwrap();

        builder1.receive(round3_2.clone()).unwrap();
        builder1.receive(round3_3.clone()).unwrap();

        builder2.receive(round3_1.clone()).unwrap();
        builder2.receive(round3_3.clone()).unwrap();

        builder3.receive(round3_1.clone()).unwrap();
        builder3.receive(round3_2.clone()).unwrap();

        if let AggregatedProofRoundData::Round3(proof) = round3_1 {
            assert!(Secp256k1::vrf_verify(pk, msg, &proof, None).is_ok());
        } else {
            assert!(false);
        }
    }
}
