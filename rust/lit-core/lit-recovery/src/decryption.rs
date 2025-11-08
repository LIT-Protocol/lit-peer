use crate::io::writer;
use crate::{
    LitRecovery, RecoveryConfig,
    auth::JsonAuthSig,
    chain_manager::ChainManager,
    consts::LIT_NODE_UPLOAD_DECRYPTION_SHARE_ENDPOINT,
    error::{Error, RecoveryResult},
    io::reader,
    models::{EncryptedKeyShare, OldEncryptedKeyShare, UploadedShareData},
    shares::{COLUMN_ENCRYPTION_KEY, ShareData, ShareDatabase},
};
use bulletproofs::BulletproofCurveArithmetic;
use bulletproofs::vsss_rs::{DefaultShare, IdentifierPrimeField};
use ethers::types::H160;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use verifiable_share_encryption::{
    Ciphertext, DecryptionShare, VerifiableEncryption, VerifiableEncryptionDecryptor,
};

pub async fn load_upload_shares<C>(
    recovery: &LitRecovery, encryption_key: String, shares: &[PathBuf],
    upload_shares_by_staker_address: &mut HashMap<String, Vec<UploadedShareData>>,
) -> RecoveryResult<()>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
    C::Scalar: Serialize + DeserializeOwned,
{
    for share in shares {
        let backup = read_ciphertext::<C>(share.clone())?;
        let (decryption_share, recovery_share) = generate_decryption_share::<C>(
            recovery,
            &backup.ciphertext,
            encryption_key.clone(),
            None,
        )
        .await?;
        let share_data = UploadedShareData {
            participant_id: 0, // Temporarily for backward compatibility with Datil
            session_id: recovery_share.session_id,
            subnet_id: recovery_share.subnet_id,
            curve: recovery_share.curve,
            verification_key: backup.public_key.clone(),
            decryption_share: serde_json::to_string(&decryption_share)?,
            encryption_key: recovery_share.encryption_key,
        };
        upload_shares_by_staker_address
            .entry(backup.staker_address.clone())
            .and_modify(|s| s.push(share_data.clone()))
            .or_insert(vec![share_data]);
        println!("Generated decryption share for root key: {}", &backup.public_key);
    }
    Ok(())
}

pub async fn generate_and_send_decryption_shares_to_nodes<C>(
    recovery: &LitRecovery, ciphertext_file: PathBuf, encryption_key: String,
) -> RecoveryResult<()>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
    C::Scalar: Serialize + DeserializeOwned,
{
    let backup = read_ciphertext::<C>(ciphertext_file)?;
    let (decryption_share, recovery_share) =
        generate_decryption_share::<C>(recovery, &backup.ciphertext, encryption_key, None).await?;

    send_decryption_share_to_node(recovery, decryption_share, recovery_share, &backup).await?;

    Ok(())
}

pub async fn send_decryption_shares_to_nodes(
    recovery: &LitRecovery,
    upload_shares_by_staker_address: &HashMap<String, Vec<UploadedShareData>>,
) -> RecoveryResult<()> {
    let client = reqwest::ClientBuilder::new()
        .tls_sni(false)
        .build()
        .map_err(|e| Error::General(e.to_string()))?;

    let cfg = recovery.get_config().await;
    let protocol = get_protocol(&cfg);
    let sk = recovery.get_secret_for_wallet().await?;
    let key = recovery.get_signing_key().await?;

    for (staker_address, shares) in upload_shares_by_staker_address.iter() {
        println!("Sending decryption shares to {}", staker_address);
        let contracts = ChainManager::new_with_signer(&sk, &cfg).await?;
        let validator_info = contracts
            .get_validator_struct_from_staker_address(
                staker_address
                    .parse::<H160>()
                    .map_err(|e| Error::InvalidEthereumAddress(e.to_owned().to_string()))?,
            )
            .await;

        let validator_info = match validator_info {
            Ok(validator) => validator,
            Err(e) => {
                return Err(Error::Contract(e.to_string()));
            }
        };

        println!("validator_info: {:?}", validator_info);

        let url = format!(
            "{}://{}:{}{}",
            protocol,
            std::net::Ipv4Addr::from(validator_info.ip),
            validator_info.port,
            LIT_NODE_UPLOAD_DECRYPTION_SHARE_ENDPOINT,
        );

        // Get AuthSig
        let auth_sig = JsonAuthSig::new(
            &key,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| Error::General(e.to_string()))?
                .as_millis()
                .to_string(),
        );
        let mut json_map = serde_json::Map::new();
        let auth_sig_val = serde_json::to_value(auth_sig)?;
        json_map.insert("authSig".to_string(), auth_sig_val);
        let share_data_val = serde_json::to_value(shares)?;
        json_map.insert("shareData".to_string(), share_data_val);

        println!("sending request for uploading share to {}", url);

        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json_map)?)
            .send()
            .await;

        println!("response {:?}", response);
        response.map_err(|e| Error::General(e.to_string()))?;
    }
    Ok(())
}

async fn send_decryption_share_to_node<C>(
    recovery: &LitRecovery, decryption_share: DecryptionShare<C>, recovery_share: ShareData,
    backup: &EncryptedKeyShare<C>,
) -> RecoveryResult<()>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
{
    println!("Sending decryption share to {}", backup.staker_address);
    let cfg = recovery.get_config().await;
    let sk = recovery.get_secret_for_wallet().await?;
    let contracts = ChainManager::new_with_signer(&sk, &cfg).await?;
    let validator_info = contracts
        .get_validator_struct_from_staker_address(
            backup
                .staker_address
                .parse::<H160>()
                .map_err(|e| Error::InvalidEthereumAddress(e.to_owned().to_string()))?,
        )
        .await;

    let validator_info = match validator_info {
        Ok(validator) => validator,
        Err(e) => {
            return Err(Error::Contract(e.to_string()));
        }
    };

    println!("validator_info: {:?}", validator_info);

    let protocol = get_protocol(&cfg);
    let url = format!(
        "{}://{}:{}{}",
        protocol,
        std::net::Ipv4Addr::from(validator_info.ip),
        validator_info.port,
        LIT_NODE_UPLOAD_DECRYPTION_SHARE_ENDPOINT,
    );

    // Get AuthSig
    let key = recovery.get_signing_key().await?;
    let auth_sig = JsonAuthSig::new(
        &key,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::General(e.to_string()))?
            .as_millis()
            .to_string(),
    );
    let mut json_map = serde_json::Map::new();
    let auth_sig_val = serde_json::to_value(auth_sig)?;
    json_map.insert("authSig".to_string(), auth_sig_val);

    // Get Share Data to upload
    let share_data = UploadedShareData {
        participant_id: 0, // Temporarily for backward compatibility with Datil
        session_id: recovery_share.session_id,
        subnet_id: recovery_share.subnet_id,
        curve: recovery_share.curve,
        verification_key: backup.public_key.clone(),
        decryption_share: serde_json::to_string(&decryption_share)?,
        encryption_key: recovery_share.encryption_key,
    };
    let share_data_val = serde_json::to_value(share_data)?;
    json_map.insert("shareData".to_string(), share_data_val);

    println!("sending request for uploading share to {}", url);

    let client = reqwest::ClientBuilder::new()
        .tls_sni(false)
        .build()
        .map_err(|e| Error::General(e.to_string()))?;
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_map)?)
        .send()
        .await;

    println!("response {:?}", response);
    response.map_err(|e| Error::General(e.to_string()))?;

    Ok(())
}

pub fn read_ciphertext<C>(ciphertext_file: PathBuf) -> RecoveryResult<EncryptedKeyShare<C>>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
{
    let reader1 = reader(&Some(ciphertext_file.clone()))?;

    let backup = ciborium::from_reader(reader1);
    match backup {
        Ok(backup) => Ok(backup),
        Err(e) => {
            let reader2 = reader(&Some(ciphertext_file))?;
            let old_backup = ciborium::from_reader::<OldEncryptedKeyShare<C>, _>(reader2);
            match old_backup {
                Ok(old_backup) => Ok(EncryptedKeyShare::from(old_backup)),
                Err(_) => Err(Error::InvalidCborFormat(e.to_string())),
            }
        }
    }
}

pub async fn generate_decryption_share<C>(
    recovery: &LitRecovery, ciphertext: &Ciphertext<C>, encryption_key: String,
    share_file: Option<PathBuf>,
) -> RecoveryResult<(DecryptionShare<C>, ShareData)>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
    <C as BulletproofCurveArithmetic>::Scalar: Serialize + DeserializeOwned,
{
    let shares_db = match share_file {
        Some(share_file) => ShareDatabase::open_with_path(recovery, &share_file).await?,
        None => recovery.get_shared_database().await?,
    };
    let mut filter = BTreeMap::new();
    filter.insert(COLUMN_ENCRYPTION_KEY, encryption_key.clone());
    let shares = shares_db.get_shares(None, Some(filter))?;

    if shares.is_empty() {
        return Err(Error::General(format!(
            "No shares found under the specified encryption key: {}",
            encryption_key
        )));
    }

    let share_data = shares[0].clone();
    let share = serde_json::from_str::<
        DefaultShare<IdentifierPrimeField<C::Scalar>, IdentifierPrimeField<C::Scalar>>,
    >(&share_data.decryption_key_share)?;
    let decryption_share = DecryptionShare::<C>::new(&share, ciphertext);

    Ok((decryption_share, share_data))
}

pub async fn write_local_decrypt_share<C>(
    recovery: &LitRecovery, ciphertext_file: PathBuf, encryption_key: String,
    share_file: Option<PathBuf>, output_share_file: PathBuf,
) -> RecoveryResult<()>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
    <C as BulletproofCurveArithmetic>::Scalar: Serialize + DeserializeOwned,
{
    let backup = read_ciphertext::<C>(ciphertext_file)?;
    let (decryption_share, recovery_share) =
        generate_decryption_share::<C>(recovery, &backup.ciphertext, encryption_key, share_file)
            .await?;
    // Get Share Data to upload
    let share_data = UploadedShareData {
        participant_id: 0, // Temporarily for backward compatibility with Datil
        session_id: recovery_share.session_id,
        subnet_id: recovery_share.subnet_id,
        curve: recovery_share.curve,
        verification_key: backup.public_key.clone(),
        decryption_share: serde_json::to_string(&decryption_share)?,
        encryption_key: recovery_share.encryption_key,
    };
    let mut w = writer(&Some(output_share_file))?;
    serde_json::to_writer(&mut w, &share_data)?;
    Ok(())
}

pub fn read_blinder<C>(blinder_file: PathBuf, name: &str) -> RecoveryResult<C::Scalar>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
{
    let blinder_reader = reader(&Some(blinder_file))?;
    let map: BTreeMap<String, String> = serde_json::from_reader(blinder_reader)?;
    let scalar_hexits =
        map.get(name).ok_or(Error::General(format!("{} not found in blinder file", name)))?;
    let blinder_bytes = hex::decode(scalar_hexits)?;
    C::deserialize_scalar(blinder_bytes.as_slice())
        .map_err(|_| Error::General("Invalid blinder file".to_string()))
}

pub fn merge_decryption_shares<C>(
    ciphertext_file: PathBuf, blinder: C::Scalar, decrypted_share_files: Vec<PathBuf>,
    output_file: PathBuf, generator: Option<C::Point>,
) -> RecoveryResult<()>
where
    C: VerifiableEncryption + VerifiableEncryptionDecryptor,
{
    println!("Merging decryption shares");
    let backup = read_ciphertext::<C>(ciphertext_file)?;

    let mut shares = Vec::new();
    for share_file in decrypted_share_files {
        let reader = reader(&Some(share_file))?;
        let share_data: UploadedShareData = serde_json::from_reader(reader)?;
        let decryption_share: DecryptionShare<C> =
            serde_json::from_str(&share_data.decryption_share)?;
        shares.push(decryption_share);
    }

    let plaintext =
        C::decrypt_with_shares_and_unblind(&blinder, &shares, &backup.ciphertext, generator)
            .map_err(|e| Error::General(e.to_string()))?;

    let mut w = writer(&Some(output_file))?;
    let bytes = C::serialize_scalar(&plaintext);
    ciborium::into_writer(&bytes, &mut w).map_err(|e| Error::General(e.to_string()))?;
    Ok(())
}

fn get_protocol(cfg: &RecoveryConfig) -> &str {
    const PROTOCOL: &str = "LIT_RECOVERY_PROTOCOL";
    match std::env::var(PROTOCOL) {
        Ok(value) if value == "http" => return "http",
        Ok(value) if value == "https" => return "https",
        Ok(value) => println!(
            "{} is set to a value other than `http` and `https`, ignoring: {}",
            PROTOCOL, value
        ),
        Err(_) => {}
    };

    // compute the value based on `env`:
    match cfg.get_env_or_default() {
        0 => "http",
        _ => "https",
    }
}
