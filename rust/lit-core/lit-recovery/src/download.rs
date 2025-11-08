use bulletproofs::k256::{
    ecdsa::SigningKey,
    elliptic_curve::{
        Field, PrimeField, consts::U32, generic_array::GenericArray, group::GroupEncoding,
        ops::Reduce, point::AffineCoordinates, sec1::ToEncodedPoint,
    },
};
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use sha2::Digest;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::chain_manager::ChainManager;
use crate::error::Error;
use crate::models::DownloadedShareData;
use crate::{LitRecovery, RecoveryResult, ShareData, auth::JsonAuthSig};

// sha2::Sha256::digest("secp256k1")
const CURVE_NAME_SECP256K1: [u8; 32] = [
    56, 59, 39, 83, 33, 83, 243, 83, 250, 76, 198, 137, 35, 159, 115, 101, 223, 233, 36, 235, 207,
    103, 128, 126, 182, 145, 99, 7, 164, 226, 112, 30,
];
// sha2::Sha256::digest("prime256v1")
const CURVE_NAME_PRIME256V1: [u8; 32] = [
    236, 151, 14, 250, 71, 58, 162, 250, 152, 240, 56, 58, 218, 26, 64, 52, 15, 149, 88, 58, 236,
    119, 101, 93, 71, 74, 121, 18, 49, 68, 120, 167,
];
// sha2::Sha256::digest("ed25519")
const CURVE_NAME_ED25519: [u8; 32] = [
    61, 95, 79, 149, 205, 177, 205, 252, 113, 1, 78, 250, 26, 102, 159, 212, 37, 153, 160, 206, 32,
    0, 217, 20, 164, 9, 228, 139, 204, 174, 213, 132,
];
// sha2::Sha256::digest("ristretto25519")
const CURVE_NAME_RISTRETTO25519: [u8; 32] = [
    120, 127, 205, 246, 51, 127, 133, 190, 112, 226, 16, 214, 75, 27, 135, 48, 63, 170, 71, 224,
    84, 134, 23, 127, 69, 88, 177, 252, 50, 180, 148, 63,
];
// sha2::Sha256::digest("secp384r1")
const CURVE_NAME_SECP384R1: [u8; 32] = [
    186, 177, 41, 47, 70, 175, 220, 252, 148, 37, 181, 41, 191, 16, 142, 88, 170, 179, 147, 33,
    237, 86, 1, 244, 50, 172, 231, 197, 128, 13, 102, 124,
];
// sha2::Sha256::digest("ed448")
const CURVE_NAME_ED448: [u8; 32] = [
    64, 21, 148, 72, 229, 32, 60, 112, 169, 236, 0, 249, 73, 10, 229, 199, 214, 14, 0, 188, 177,
    188, 162, 237, 50, 200, 182, 177, 34, 76, 212, 90,
];
// sha2::Sha256::digest("jubjub")
const CURVE_NAME_JUBJUB: [u8; 32] = [
    134, 207, 207, 62, 155, 118, 130, 42, 187, 158, 186, 128, 70, 96, 138, 78, 235, 13, 173, 62,
    30, 220, 174, 128, 204, 21, 33, 35, 77, 117, 80, 189,
];
// sha2::Sha256::digest("decaf377")
const CURVE_NAME_DECAF377: [u8; 32] = [
    230, 94, 124, 238, 33, 113, 222, 166, 20, 37, 159, 94, 157, 139, 217, 176, 3, 222, 135, 65,
    189, 98, 129, 57, 205, 178, 255, 43, 212, 70, 169, 55,
];
// sha2::Sha256::digest("bls12381g1")
const CURVE_NAME_BLS12381G1: [u8; 32] = [
    157, 137, 108, 202, 42, 239, 133, 106, 124, 17, 78, 140, 254, 165, 166, 3, 68, 236, 72, 237,
    26, 60, 125, 231, 225, 12, 198, 231, 69, 129, 98, 109,
];
// sha2::Sha256::digest("bls12381g2")
#[allow(dead_code)]
const CURVE_NAME_BLS12381G2: [u8; 32] = [
    234, 117, 92, 131, 99, 84, 34, 238, 113, 135, 28, 154, 84, 213, 205, 6, 52, 142, 9, 84, 93, 98,
    145, 179, 160, 123, 115, 254, 95, 105, 154, 249,
];
// sha2::Sha256::digest("bls12381gt")
#[allow(dead_code)]
const CURVE_NAME_BLS12381GT: [u8; 32] = [
    72, 104, 114, 249, 247, 74, 129, 138, 239, 93, 192, 105, 87, 88, 22, 147, 201, 72, 247, 204,
    168, 110, 248, 13, 211, 195, 253, 59, 152, 53, 40, 135,
];

#[allow(dead_code)]
trait ContractProof {
    const CURRENT_VERSION: u8;
    const BYTES: usize;
    const CURVE: [u8; 32];

    fn generate_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    fn timestamp(&self) -> u64;

    fn participant_id(&self) -> u8;

    fn schnorr_proof(&self) -> &[u8];

    fn schnorr_verification_key(&self) -> &[u8];

    fn to_bytes(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(Self::BYTES);
        let mut writer = std::io::Cursor::new(&mut output[..]);
        writer.write_all(&[Self::CURRENT_VERSION]).unwrap();
        writer.write_all(&self.timestamp().to_be_bytes()).unwrap();
        writer.write_all(&[self.participant_id()]).unwrap();
        writer.write_all(&Self::CURVE).unwrap();
        writer.write_all(self.schnorr_proof()).unwrap();
        writer.write_all(self.schnorr_verification_key()).unwrap();
        output
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ContractProofK256 {
    pub version: u8,
    pub timestamp: u64,
    pub participant_id: u8,
    pub schnorr_proof: [u8; 64],
    pub schnorr_verification_key: [u8; 64],
}

impl ContractProofK256 {
    #[allow(dead_code)]
    pub fn generate(share: &[u8], participant_id: u8) -> RecoveryResult<Self> {
        use bulletproofs::k256::*;

        let mut repr = FieldBytes::default();
        repr.copy_from_slice(share);
        let share = Option::<Scalar>::from(Scalar::from_repr(repr))
            .ok_or(Error::General("invalid share".to_string()))?;
        let share_pub = ProjectivePoint::GENERATOR * share;
        let timestamp = <Self as ContractProof>::generate_timestamp();

        let r = Scalar::random(rand::rngs::OsRng);
        let big_r = ProjectivePoint::GENERATOR * r;
        let big_r_affine = big_r.to_affine();
        let big_r_bytes = big_r_affine.x();

        let msg = gen_schnorr_msg(
            Self::CURRENT_VERSION,
            timestamp,
            participant_id,
            &CURVE_NAME_SECP256K1,
        );

        let c_bytes =
            gen_schnorr_challenge_bytes(big_r_bytes.as_slice(), &share_pub.to_bytes(), &msg);
        let c = <Scalar as Reduce<U256>>::reduce_bytes(&c_bytes);

        let s = r + c * share;

        let mut schnorr_proof = [0u8; 64];
        schnorr_proof[..32].copy_from_slice(big_r_bytes.as_slice());
        schnorr_proof[32..].copy_from_slice(&s.to_bytes());

        let mut schnorr_verification_key = [0u8; 64];
        schnorr_verification_key
            .copy_from_slice(&share_pub.to_encoded_point(false).as_bytes()[1..]);

        Ok(Self {
            version: Self::CURRENT_VERSION,
            timestamp,
            participant_id,
            schnorr_proof,
            schnorr_verification_key,
        })
    }
}

#[allow(dead_code)]
impl ContractProof for ContractProofK256 {
    const CURRENT_VERSION: u8 = 1;
    const BYTES: usize = 170;
    const CURVE: [u8; 32] = CURVE_NAME_SECP256K1;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
struct ContractProofBls12381G1 {
    pub version: u8,
    pub timestamp: u64,
    pub participant_id: u8,
    pub schnorr_proof: [u8; 128],
    pub schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
impl ContractProofBls12381G1 {
    #[allow(dead_code)]
    pub fn generate(share: &[u8], participant_id: u8) -> RecoveryResult<Self> {
        use bulletproofs::blstrs_plus::*;

        let share_bytes = <[u8; 32]>::try_from(share).unwrap();
        let share = Option::<Scalar>::from(Scalar::from_be_bytes(&share_bytes))
            .ok_or(Error::General("invalid share".to_string()))?;
        let share_pub = G1Projective::GENERATOR * share;
        let timestamp = <Self as ContractProof>::generate_timestamp();

        let r = Scalar::random(rand::rngs::OsRng);
        let big_r = G1Projective::GENERATOR * r;

        let msg = gen_schnorr_msg(
            Self::CURRENT_VERSION,
            timestamp,
            participant_id,
            &CURVE_NAME_BLS12381G1,
        );

        let c_bytes =
            gen_schnorr_challenge_bytes(&big_r.to_compressed(), &share_pub.to_compressed(), &msg);
        let mut c_arr = [0u8; 64];
        c_arr[64 - c_bytes.len()..].copy_from_slice(&c_bytes[..]);
        let c = Scalar::from_bytes_wide(&c_arr);

        let s = r + c * share;

        let mut schnorr_proof = [0u8; 128];
        schnorr_proof[..96].copy_from_slice(&big_r.to_uncompressed());
        schnorr_proof[96..].copy_from_slice(&s.to_be_bytes());

        let mut schnorr_verification_key = [0u8; 96];
        schnorr_verification_key.copy_from_slice(&share_pub.to_uncompressed());

        Ok(Self {
            version: Self::CURRENT_VERSION,
            timestamp,
            participant_id,
            schnorr_proof,
            schnorr_verification_key,
        })
    }
}

#[allow(dead_code)]
impl ContractProof for ContractProofBls12381G1 {
    const CURRENT_VERSION: u8 = 1;
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_BLS12381G1;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofNistP256 {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofNistP384 {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofEd25519 {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofRistretto25519 {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofEd448 {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofJubJub {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofDecaf377 {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ContractProofBls12381G1Sign {
    version: u8,
    timestamp: u64,
    participant_id: u8,
    schnorr_proof: [u8; 128],
    schnorr_verification_key: [u8; 96],
}

impl ContractProof for ContractProofNistP256 {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 170;
    const CURVE: [u8; 32] = CURVE_NAME_PRIME256V1;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofNistP384 {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_SECP384R1;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofEd25519 {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_ED25519;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofRistretto25519 {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_RISTRETTO25519;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofEd448 {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_ED448;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofJubJub {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_JUBJUB;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofDecaf377 {
    const CURRENT_VERSION: u8 = 1;
    // TODO: Fix this amount
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_DECAF377;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

impl ContractProof for ContractProofBls12381G1Sign {
    const CURRENT_VERSION: u8 = 1;
    const BYTES: usize = 266;
    const CURVE: [u8; 32] = CURVE_NAME_BLS12381G1;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn participant_id(&self) -> u8 {
        self.participant_id
    }

    fn schnorr_proof(&self) -> &[u8] {
        &self.schnorr_proof
    }

    fn schnorr_verification_key(&self) -> &[u8] {
        &self.schnorr_verification_key
    }
}

fn gen_schnorr_msg(
    version: u8, timestamp: u64, participant_id: u8, curve_name: &[u8],
) -> GenericArray<u8, U32> {
    let mut hasher = sha2::Sha256::default();
    hasher.update([version]);
    hasher.update(timestamp.to_be_bytes());
    hasher.update([participant_id]);
    hasher.update(curve_name);
    hasher.finalize()
}

#[allow(dead_code)]
fn gen_schnorr_challenge_bytes(
    big_r_bytes: &[u8], share_pub_bytes: &[u8], msg: &[u8],
) -> GenericArray<u8, U32> {
    let mut hasher = sha2::Sha256::default();
    hasher.update(big_r_bytes);
    hasher.update(share_pub_bytes);
    hasher.update(msg);
    hasher.finalize()
}

pub async fn download_share(
    recovery: &LitRecovery, url: &str,
    _cm: &ChainManager<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> RecoveryResult<()> {
    let key = recovery.get_signing_key().await?;
    let auth_sig = JsonAuthSig::new(
        &key,
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string(),
    );
    let mut json_map = serde_json::Map::new();
    let auth_sig_val = serde_json::to_value(auth_sig).unwrap();
    json_map.insert("authSig".to_string(), auth_sig_val);
    println!("sending request for share download");

    let client = reqwest::ClientBuilder::new().tls_sni(false).build().unwrap();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_map).unwrap())
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(e) => {
            return Err(Error::General(format!("failed to download share: {}", e,)));
        }
    };
    let response_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(Error::General(format!("failed to get bytes from response body: {}", e,)));
        }
    };
    let decryption_share_data: Vec<DownloadedShareData> = serde_json::from_slice(&response_bytes)
        .inspect_err(|e| {
        println!(
            "unable to deserialize response from JSON. error response from chain: {} \n\
                     response from node holding recovery share: {:?}",
            e,
            String::from_utf8(response_bytes.to_vec()).expect("a valid utf-8 string"),
        );
    })?;
    println!("got share data: {:#?}", decryption_share_data);

    let share_db = recovery.get_shared_database().await?;

    for share in decryption_share_data {
        let share_data = ShareData::from((share.clone(), url.to_string()));
        share_db.insert_share(&share_data)?;
    }
    Ok(())
}

pub async fn delete_share(recovery: &LitRecovery, url: &String) -> RecoveryResult<bool> {
    let key = recovery.get_signing_key().await?;
    let auth_sig = JsonAuthSig::new(
        &key,
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string(),
    );
    let mut json_map = serde_json::Map::new();
    let auth_sig_val = serde_json::to_value(auth_sig)?;
    json_map.insert("authSig".to_string(), auth_sig_val);
    println!("sending request for share deletion");
    let client = reqwest::ClientBuilder::new().tls_sni(false).build().unwrap();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_map)?)
        .send()
        .await;
    match response {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::General(format!("failed to delete share: {}", e,))),
    }
}

impl std::fmt::Debug for DownloadedShareData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("DownloadedShareData")
            .field("session_id", &self.session_id)
            .field("encryption_key", &self.encryption_key)
            .field("decryption_key_share", &"******")
            .field("subnet_id", &self.subnet_id)
            .field("curve", &self.curve)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_contract_proof_k256() {
        use bulletproofs::k256::*;

        let share = Scalar::random(rand::rngs::OsRng);
        let res = ContractProofK256::generate(&share.to_bytes(), 1);
        assert!(res.is_ok());
        let proof = res.unwrap();
        assert_eq!(proof.version, 1);
        assert_eq!(proof.participant_id, 1);
        assert_eq!(proof.schnorr_proof.len(), 64);
        assert_eq!(
            &proof.schnorr_verification_key,
            &(ProjectivePoint::GENERATOR * share).to_encoded_point(false).as_bytes()[1..]
        );
        let out = proof.to_bytes();
        assert_eq!(out.len(), ContractProofK256::BYTES);
    }

    #[ignore]
    #[test]
    fn test_contract_proof_bls() {
        use bulletproofs::blstrs_plus::*;

        let share = Scalar::random(rand::rngs::OsRng);
        let res = ContractProofBls12381G1::generate(&share.to_be_bytes(), 1);
        assert!(res.is_ok());
        let proof = res.unwrap();
        assert_eq!(proof.version, 1);
        assert_eq!(proof.participant_id, 1);
        assert_eq!(proof.schnorr_proof.len(), 128);
        assert_eq!(
            &proof.schnorr_verification_key[..],
            (G1Projective::GENERATOR * share).to_uncompressed()
        );
        let out = proof.to_bytes();
        assert_eq!(out.len(), ContractProofBls12381G1::BYTES);
    }
}
