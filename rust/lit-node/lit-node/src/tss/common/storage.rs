use super::key_persistence::RECOVERY_DKG_EPOCH;
use crate::common::key_helper::{KeyCache, KeyCacheType};
use crate::common::storage::do_write_to_cache_only;
use crate::common::storage::{create_storage_dir, do_read_from_disk, do_write_to_disk};
use crate::error::{EC, Error, Result, io_err, io_err_code, unexpected_err, unexpected_err_code};
use async_std::path::PathBuf;
use async_std::stream::StreamExt;
use glob::glob;
use lit_node_common::config::{key_commitment_path, presign_path, segmented_paths, typed_key_path};
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use serde::{Serialize, de::DeserializeOwned};
use tracing::{instrument, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    KeyShare(CurveType),
    Presign(CurveType),
    KeyShareCommitment(CurveType),
}

impl StorageType {
    pub(crate) fn is_key_share(&self) -> bool {
        matches!(self, StorageType::KeyShare(_))
    }

    pub(crate) fn get_root_dir(&self, staker_address: &str) -> Result<PathBuf> {
        match self {
            StorageType::KeyShare(curve_type) => {
                Ok(typed_key_path(curve_type.as_str(), staker_address))
            }
            StorageType::Presign(curve_type) => {
                Ok(presign_path(curve_type.as_str(), staker_address))
            }
            StorageType::KeyShareCommitment(_) => Ok(key_commitment_path(staker_address)),
        }
    }

    pub(crate) fn file_name_prefix(&self) -> &'static str {
        match self {
            StorageType::KeyShare(_) => "Key",
            StorageType::Presign(_) => "Presign",
            StorageType::KeyShareCommitment(_) => "KeyShareCommitment",
        }
    }
}

impl From<StorageType> for CurveType {
    fn from(storage_type: StorageType) -> CurveType {
        match storage_type {
            StorageType::KeyShare(curve_type) => curve_type,
            StorageType::Presign(curve_type) => curve_type,
            StorageType::KeyShareCommitment(curve_type) => curve_type,
        }
    }
}

/**************** KEY SHARE ****************/

#[doc = "Reads a local key share from disk"]
#[instrument(level = "debug", name = "read_key_share_from_disk", skip(key_cache))]
pub async fn read_key_share_from_disk<T>(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShare(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_read_from_disk(&path, key_cache, KeyCacheType::Protected).await
}

#[allow(clippy::too_many_arguments)]
#[doc = "Writes a local key share to disk"]
#[instrument(level = "debug", name = "write_key_share_to_disk", skip_all)]
pub async fn write_key_share_to_disk<T>(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
    local_key: &T,
) -> Result<()>
where
    T: Serialize + Sync,
{
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShare(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_write_to_disk(&path, key_cache, KeyCacheType::Protected, local_key).await
}

#[allow(clippy::too_many_arguments)]
#[doc = "Writes a local key share to a key cache"]
#[instrument(level = "debug", name = "write_key_share_to_cache_only", skip_all)]
pub async fn write_key_share_to_cache_only<T>(
    curve_type: CurveType,
    pubkey: &str,
    peer_id: &PeerId,
    staker_address: &str,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
    local_key: &T,
) -> Result<()>
where
    T: Serialize + Sync,
{
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShare(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_write_to_cache_only(&path, key_cache, KeyCacheType::Protected, local_key).await
}

#[doc = "Checks if a local key share exists on disk"]
#[instrument(level = "trace")]
pub(crate) async fn any_key_share_exists(
    pubkey: &str,
    staker_address: &str,
) -> Result<Option<(CurveType, PeerId)>> {
    let pubkey = match pubkey.starts_with("0x") {
        true => &pubkey[2..],
        false => pubkey,
    };

    for key_type in CurveType::into_iter() {
        let file_names =
            fetch_recovery_file_names(StorageType::KeyShare(key_type), pubkey, staker_address)
                .await?;
        if let Some(file_name) = file_names.first() {
            let file = StorableFile::try_from(file_name)?;
            debug!("Found key share: {} - {:?}", file_name, key_type);
            return Ok(Some((key_type, file.peer_id)));
        }
    }

    trace!("No key share found for {}", pubkey);
    Ok(None)
}

#[allow(dead_code)]
#[doc = "Delete a key share from disk."]
#[instrument(level = "debug", name = "delete_keyshare", skip(key_cache))]
pub(crate) async fn delete_keyshare(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<()> {
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShare(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    delete_from_disk(path, key_cache).await
}

#[allow(dead_code)]
#[doc = "Deletes key shares from disk with epochs older than X.  The index value is ignore, as it may have changed."]
#[instrument(
    level = "debug",
    name = "delete_keyshares_older_than_epoch",
    skip(key_cache)
)]
pub(crate) async fn delete_keyshares_older_than_epoch(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    min_epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<()> {
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShare(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch: min_epoch,
        realm_id,
    };
    storable_file
        .delete_older_than_epoch(staker_address, key_cache)
        .await
}

#[doc = "Reads a presign from disk"]
#[instrument(level = "debug", name = "read_presign_from_disk", skip(key_cache))]
pub async fn read_presign_from_disk<T>(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let storable_file = StorableFile {
        storage_type: StorageType::Presign(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: PeerId::ONE,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_read_from_disk(&path, key_cache, KeyCacheType::Protected).await
}

#[doc = "Reads a presign from disk"]
#[instrument(
    level = "trace",
    name = "read_presign_from_disk_direct",
    skip(key_cache)
)]
pub async fn read_presign_from_disk_direct<T>(filename: &str, key_cache: &KeyCache) -> Result<T>
where
    T: DeserializeOwned,
{
    let path = PathBuf::from(filename);
    do_read_from_disk(&path, key_cache, KeyCacheType::Protected).await
}

#[doc = "Writes a presign to disk"]
#[instrument(level = "debug", name = "write_presign_to_disk", skip_all)]
pub async fn write_presign_to_disk<T>(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
    local_key: &T,
) -> Result<()>
where
    T: DeserializeOwned + Serialize + Sync,
{
    let storable_file = StorableFile {
        storage_type: StorageType::Presign(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: PeerId::ONE,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_write_to_disk(&path, key_cache, KeyCacheType::Protected, local_key).await
}

#[doc = "Delete a presign from disk."]
#[instrument(level = "debug", name = "delete_presign", skip(key_cache))]
pub(crate) async fn delete_presign(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<()> {
    let storable_file = StorableFile {
        storage_type: StorageType::Presign(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: PeerId::ONE,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    delete_from_disk(path, key_cache).await
}

/**************** BACKUP KEYS ****************/

#[doc = "Reads a key share commitment from disk"]
#[instrument(
    level = "trace",
    name = "read_key_share_commitments_from_disk",
    skip(key_cache)
)]
pub async fn read_key_share_commitments_from_disk<T>(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShareCommitment(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_read_from_disk(&path, key_cache, KeyCacheType::Unprotected).await
}

#[doc = "Copy a key share commitment from path to another"]
#[instrument(level = "trace", name = "copy_key_share_commitments_to_another_path")]
pub async fn copy_key_share_commitments_to_another_path(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    dst_folder: PathBuf,
) -> Result<()> {
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShareCommitment(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let src = storable_file.get_full_path(staker_address).await?;
    let mut dst = dst_folder;
    dst.push(storable_file.file_name());

    async_std::fs::hard_link(&src, &dst).await.map_err(|e| {
        io_err(
            e,
            Some(format!(
                "Unable to copy {} to {}",
                src.display(),
                dst.display()
            )),
        )
    })
}

#[doc = "Writes a key share commitment to disk"]
#[instrument(
    level = "trace",
    name = "write_key_share_commitments_to_disk",
    skip_all
)]
#[allow(clippy::too_many_arguments)]
pub async fn write_key_share_commitments_to_disk<T>(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
    commitments: &T,
) -> Result<()>
where
    T: Serialize + Sync,
{
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShareCommitment(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    do_write_to_disk(&path, key_cache, KeyCacheType::Unprotected, commitments).await
}

#[doc = "Deletes key share commitments from disk with epochs older than X."]
#[instrument(
    level = "trace",
    name = "delete_key_share_commitments_older_than_epoch",
    skip(key_cache)
)]
pub async fn delete_key_share_commitments_older_than_epoch(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    min_epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<()> {
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShareCommitment(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch: min_epoch,
        realm_id,
    };
    storable_file
        .delete_older_than_epoch(staker_address, key_cache)
        .await
}

#[doc = "Deletes specific key share commitments from disk."]
#[instrument(
    level = "trace",
    name = "delete_key_share_commitments",
    skip(key_cache)
)]
pub async fn delete_key_share_commitments(
    curve_type: CurveType,
    pubkey: &str,
    staker_address: &str,
    peer_id: &PeerId,
    epoch: u64,
    realm_id: u64,
    key_cache: &KeyCache,
) -> Result<()> {
    let storable_file = StorableFile {
        storage_type: StorageType::KeyShareCommitment(curve_type),
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    };
    let path = storable_file.get_full_path(staker_address).await?;
    delete_from_disk(path, key_cache).await
}

/**************** RESTORATION DATA ****************/

#[doc = "Reads encrypted keys from disk"]
pub(crate) async fn read_recovery_data_from_disk<T>(
    path: &PathBuf,
    pubkey: &str,
    storage_type: StorageType,
    key_cache: &KeyCache,
) -> Result<Vec<T>>
where
    T: DeserializeOwned + Serialize + Sync,
{
    let file_names = fetch_recovery_file_names_in_path(storage_type, pubkey, path.clone()).await?;

    let mut shares = Vec::with_capacity(file_names.len());
    for file_name in file_names.into_iter() {
        let mut path = path.clone();
        path.push(file_name);
        let share: T = do_read_from_disk(&path, key_cache, KeyCacheType::Unprotected).await?;
        shares.push(share);
    }

    Ok(shares)
}

#[doc = "Delete data file from disk."]
#[instrument(level = "debug", name = "delete_from_disk", skip(key_cache), ret)]
async fn delete_from_disk(path: PathBuf, key_cache: &KeyCache) -> Result<()> {
    tokio::fs::remove_file(path.clone()).await.map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not delete file: {:?}", path)),
        )
    })?;

    let key_path = path
        .to_str()
        .expect("Could not convert path to string")
        .to_string();
    key_cache.as_ref().remove_async(&key_path).await;

    Ok(())
}

pub async fn get_full_path(
    storage_type: StorageType,
    pubkey: &str,
    peer_id: &PeerId,
    epoch: u64,
    staker_address: &str,
    realm_id: u64,
) -> Result<PathBuf> {
    let mut path = get_directory(storage_type, pubkey, staker_address).await?;
    let file_name = StorableFile {
        storage_type,
        pubkey: pubkey.to_string(),
        peer_id: *peer_id,
        epoch,
        realm_id,
    }
    .file_name();
    path.push(file_name);
    Ok(path)
}

async fn get_directory(
    storage_type: StorageType,
    pubkey: &str,
    staker_address: &str,
) -> Result<PathBuf> {
    let root_dir = storage_type.get_root_dir(staker_address)?;
    let path = segmented_paths(root_dir, pubkey, 3, true)?;
    create_storage_dir(path.as_path()).await?;
    Ok(path)
}

#[doc = "Returns file names if such data exists on disk"]
async fn fetch_recovery_file_names(
    storage_type: StorageType,
    pubkey: &str,
    staker_address: &str,
) -> Result<Vec<String>> {
    let path = get_directory(storage_type, pubkey, staker_address).await?;
    fetch_recovery_file_names_in_path(storage_type, pubkey, path).await
}

#[doc = "Returns file names from the directory if such data exists on disk"]
pub(crate) async fn fetch_recovery_file_names_in_path(
    storage_type: StorageType,
    pubkey: &str,
    path: PathBuf,
) -> Result<Vec<String>> {
    let key_type = CurveType::from(storage_type) as u8;
    let mut files = match storage_type {
        StorageType::KeyShare(_) => {
            let legacy_pattern = path.join(format!(
                "Backup-{}{}-{}-*.cbor",
                if storage_type.is_key_share() {
                    "H-"
                } else {
                    ""
                },
                key_type,
                pubkey,
            ));
            let legacy_pattern = legacy_pattern.display().to_string();
            files_with_pattern(&legacy_pattern)?
        }
        _ => Vec::new(),
    };

    let pattern = path.join(format!(
        "{}-H-{}-{}-*.cbor",
        storage_type.file_name_prefix(),
        key_type,
        pubkey,
    ));

    let pattern = pattern.display().to_string();
    debug!("Checking for data existence: {}", pattern);
    files.append(&mut files_with_pattern(&pattern)?);
    Ok(files)
}

fn files_with_pattern(pattern: &str) -> Result<Vec<String>> {
    debug!("Checking for data existence: {}", pattern);

    match glob(pattern) {
        Err(e) => {
            warn!("Error reading glob pattern: {} - {:?}", pattern, e);
            Err(io_err(e, None))
        }
        Ok(entries) => Ok(entries
            .flatten()
            .filter_map(|entry| match entry.as_path().file_name() {
                Some(name) => name.to_str().map(String::from),
                None => None,
            })
            .collect()),
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct StorableFile {
    pub storage_type: StorageType,
    pub pubkey: String,
    pub peer_id: PeerId,
    pub epoch: u64,
    pub realm_id: u64,
}

impl TryFrom<&String> for StorableFile {
    type Error = Error;

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for StorableFile {
    type Error = Error;

    fn try_from(file_name: &str) -> Result<Self> {
        const PREFIX_MAPPINGS: &[(&str, &str)] = &[
            ("Key-H-", "Key"),
            ("Presign-H-", "Presign"),
            ("KeyShareCommitment-H-", "KeyShareCommitment"),
        ];

        let (storage, file_name) = PREFIX_MAPPINGS
            .iter()
            .find_map(|(prefix, storage_type)| {
                file_name
                    .strip_prefix(prefix)
                    .map(|name| (*storage_type, name))
            })
            .ok_or_else(|| {
                unexpected_err(format!("{} is not a valid key file name", file_name), None)
            })?;

        let parts = file_name.split('-').collect::<Vec<&str>>();

        if parts.len() < 4 {
            return Err(unexpected_err(
                format!("{} is not a valid file name", file_name),
                None,
            ));
        }

        let curve_type_u8 = parts[0].parse::<u8>().map_err(|e| io_err(e, None))?;
        let curve_type = CurveType::try_from(curve_type_u8).map_err(|e| io_err(e, None))?;
        let pubkey = parts[1].to_string();

        let storage_type = match storage {
            "Key" => StorageType::KeyShare(curve_type),
            "Presign" => StorageType::Presign(curve_type),
            "KeyShareCommitment" => StorageType::KeyShareCommitment(curve_type),
            _ => {
                return Err(unexpected_err(
                    format!(
                        "{} is not a valid key file name. Expected 'Key', 'Presign', 'KeyShareCommitment'",
                        file_name
                    ),
                    None,
                ));
            }
        };

        let peer_id = parts[2]
            .parse::<PeerId>()
            .map_err(|e| unexpected_err(e, None))?;

        let epoch = parts[3].parse::<u64>().map_err(|e| io_err(e, None))?;

        let realm_id_str = parts[4]
            .strip_suffix(".cbor")
            .expect("Could not strip filename suffix");
        let realm_id = realm_id_str.parse::<u64>().map_err(|e| io_err(e, None))?;

        Ok(Self {
            storage_type,
            pubkey,
            peer_id,
            epoch,
            realm_id,
        })
    }
}

impl std::str::FromStr for StorableFile {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl StorableFile {
    pub async fn get_full_path(&self, staker_address: &str) -> Result<PathBuf> {
        let mut path = get_directory(self.storage_type, &self.pubkey, staker_address).await?;
        path.push(self.file_name());
        Ok(path)
    }

    pub fn file_name(&self) -> String {
        let prefix = self.storage_type.file_name_prefix();
        let key_type = CurveType::from(self.storage_type) as u8;
        format!(
            "{}-H-{}-{}-{}-{}-{}.cbor",
            prefix, key_type, self.pubkey, self.peer_id, self.epoch, self.realm_id,
        )
    }

    #[doc = "Delete data from disk if older than commitments."]
    #[instrument(
        level = "debug",
        name = "delete_older_than_epoch",
        skip(key_cache),
        ret
    )]
    async fn delete_older_than_epoch(
        &self,
        staker_address: &str,
        key_cache: &KeyCache,
    ) -> Result<()> {
        let path = get_directory(self.storage_type, &self.pubkey, staker_address)
            .await
            .map_err(|e| {
                io_err_code(
                    e,
                    EC::NodeSystemFault,
                    Some(format!(
                        "Could not open file for reading key: {:?}",
                        self.pubkey
                    )),
                )
            })?;
        // read all files in path buf
        let mut files = path.read_dir().await.map_err(|e| {
            io_err_code(
                e,
                EC::NodeSystemFault,
                Some(format!("Could not read dir: {:?}", path)),
            )
        })?;

        while let Some(Ok(entry)) = files.next().await {
            let file_type = entry.file_type().await.map_err(|e| {
                io_err_code(
                    e,
                    EC::NodeSystemFault,
                    Some(format!("Could not determine file type: {:?}", entry)),
                )
            })?;

            if file_type.is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    let storable_file: StorableFile = file_name.parse()?;
                    if storable_file.realm_id == self.realm_id
                        && storable_file.epoch < self.epoch
                        && storable_file.epoch != RECOVERY_DKG_EPOCH
                    {
                        let _r = delete_from_disk(entry.path(), key_cache).await;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::common::key_helper::KeyCache;
    use crate::peers::peer_state::models::{SimplePeer, SimplePeerCollection};
    use crate::tss::common::key_persistence::KeyPersistence;
    use crate::tss::common::key_share_commitment::KeyShareCommitments;
    use crate::tss::common::storage::{
        StorableFile, StorageType, delete_key_share_commitments_older_than_epoch,
        delete_keyshares_older_than_epoch, read_key_share_commitments_from_disk,
        write_key_share_commitments_to_disk,
    };
    use elliptic_curve::Group;
    use lit_node_core::PeerId;
    use lit_node_core::{CompressedHex, CurveType};
    use rand_core::SeedableRng;
    use semver::Version;

    #[test]
    fn parse() {
        let file_name = "Key-H-1-8e4a88cf6a62635c9fba12c27020c9b25fdd38cac0d0b05c318f0be72e605bc821573c50c965fcc051c1ecd7268cdd7c-d42772f62ba997cc1e99d7288de88a1a1de45f6fdcb4f0200cdf72363255d4ce-999-1.cbor";
        let file: StorableFile = file_name.parse().unwrap();
        assert_eq!(file.epoch, 999);
        assert_eq!(file.realm_id, 1);
        assert_eq!(
            &file.pubkey,
            "8e4a88cf6a62635c9fba12c27020c9b25fdd38cac0d0b05c318f0be72e605bc821573c50c965fcc051c1ecd7268cdd7c"
        );
    }

    #[tokio::test]
    async fn delete_key_shares_older_than_epoch_test() {
        let peer_id = PeerId::from_u8(7);
        let sk = blsful::inner_types::Scalar::from_bytes_wide(&[1u8; 64]);
        let pk = blsful::inner_types::G1Projective::GENERATOR * sk;
        let pubkey = pk.to_compressed_hex();

        let stkr = k256::Scalar::from(137u64);
        let stkr_pub = k256::ProjectivePoint::GENERATOR * stkr;
        let staker_address = stkr_pub.to_compressed_hex();

        let key_persistence =
            KeyPersistence::<blsful::inner_types::G1Projective>::new(CurveType::BLS);
        let key_cache = KeyCache::default();
        let peers = dummy_peers();

        for epoch in 1..=5 {
            key_persistence
                .write_key(
                    Some(pubkey.clone()),
                    pk,
                    sk,
                    &peer_id,
                    "",
                    epoch,
                    &peers,
                    &staker_address,
                    1,
                    3,
                    &key_cache,
                )
                .await
                .unwrap();
        }

        delete_keyshares_older_than_epoch(
            CurveType::BLS,
            &pubkey,
            &staker_address,
            &peer_id,
            4,
            1,
            &key_cache,
        )
        .await
        .unwrap();

        for epoch in 1..=3 {
            let r = key_persistence
                .read_key(&pubkey, &peer_id, epoch, &staker_address, 1, &key_cache)
                .await;
            assert!(r.is_err(), "epoch {}", epoch);
        }
        for epoch in 4..=5 {
            let r = key_persistence
                .read_key(&pubkey, &peer_id, epoch, &staker_address, 1, &key_cache)
                .await;
            assert!(r.is_ok());
            let share = r.unwrap();
            assert!(share.is_some());
            let (secret, public) = share.unwrap();
            assert_eq!(public, pk);
            assert_eq!(secret, sk);
        }
    }

    #[tokio::test]
    async fn delete_key_commitments_older_than_epoch_test() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(7);
        let peer_id = PeerId::from_u8(7);

        let public_key = k256::ProjectivePoint::random(&mut rng);
        let pubkey = public_key.to_compressed_hex();

        let stkr = k256::Scalar::from(137u64);
        let stkr_pub = k256::ProjectivePoint::GENERATOR * stkr;
        let staker_address = stkr_pub.to_compressed_hex();

        let key_cache = KeyCache::default();
        let peers = dummy_peers();
        for epoch in 1..=5 {
            let commitments = KeyShareCommitments {
                dkg_id: format!("DKG_ID_{}", epoch),
                commitments: (0..4)
                    .map(|i| k256::ProjectivePoint::random(&mut rng))
                    .collect(),
            };
            write_key_share_commitments_to_disk(
                CurveType::K256,
                &pubkey,
                &staker_address,
                &peer_id,
                epoch,
                1,
                &key_cache,
                &commitments,
            )
            .await
            .unwrap();
        }

        delete_key_share_commitments_older_than_epoch(
            CurveType::K256,
            &pubkey,
            &staker_address,
            &peer_id,
            4,
            1,
            &key_cache,
        )
        .await
        .unwrap();

        for epoch in 1..=3 {
            let r =
                read_key_share_commitments_from_disk::<KeyShareCommitments<k256::ProjectivePoint>>(
                    CurveType::K256,
                    &pubkey,
                    &staker_address,
                    &peer_id,
                    epoch,
                    1,
                    &key_cache,
                )
                .await;
            assert!(r.is_err(), "epoch {}", epoch);
        }
        for epoch in 4..=5 {
            let r =
                read_key_share_commitments_from_disk::<KeyShareCommitments<k256::ProjectivePoint>>(
                    CurveType::K256,
                    &pubkey,
                    &staker_address,
                    &peer_id,
                    epoch,
                    1,
                    &key_cache,
                )
                .await;
            assert!(r.is_ok());
            let commitments = r.unwrap();
            assert_eq!(commitments.commitments.len(), 4);
            assert_eq!(commitments.dkg_id, format!("DKG_ID_{}", epoch));
        }
    }

    #[test]
    fn test_fetch_public_key_from_file_name() {
        use crate::tss::common::storage::StorageType;
        let pub_key = "0123456789abcdef";
        let file_name = StorableFile {
            storage_type: StorageType::KeyShare(CurveType::BLS),
            pubkey: pub_key.to_string(),
            peer_id: PeerId::from_u8(2u8),
            epoch: 1,
            realm_id: 1,
        }
        .file_name();
        let storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(&storable_file.pubkey, pub_key);
    }

    #[test]
    fn test_storable_file_name() {
        let mut storable_file = StorableFile {
            storage_type: StorageType::KeyShare(CurveType::BLS),
            pubkey: "0123456789abcdef".to_string(),
            peer_id: PeerId::from_u8(2u8),
            epoch: 1,
            realm_id: 1,
        };
        let file_name = storable_file.file_name();
        let res_storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(res_storable_file, storable_file);

        let file_name = storable_file.file_name();
        let res_storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(res_storable_file, storable_file);

        storable_file.epoch = 0;
        storable_file.realm_id = 0;
        let file_name = storable_file.file_name();
        let res_storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(res_storable_file, storable_file);

        let file_name = storable_file.file_name();
        let res_storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(res_storable_file, storable_file);

        storable_file.storage_type = StorageType::KeyShareCommitment(CurveType::K256);
        let file_name = storable_file.file_name();
        let res_storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(res_storable_file, storable_file);

        let file_name = storable_file.file_name();
        let res_storable_file = StorableFile::try_from(&file_name).unwrap();
        assert_eq!(res_storable_file, storable_file);
    }

    fn dummy_peers() -> SimplePeerCollection {
        let mut peers = SimplePeerCollection::default();
        peers.0.push(SimplePeer {
            socket_address: "".to_string(),
            peer_id: PeerId::from_u8(7),
            staker_address: Default::default(),
            key_hash: 0,
            kicked: false,
            version: Version::new(1, 0, 0),
            realm_id: ethers::prelude::U256::from(1u64),
        });
        peers
    }
}
