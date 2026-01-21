use async_std::path::{Path, PathBuf};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::instrument;

use crate::error::{EC, Result, io_err, io_err_code, unexpected_err, unexpected_err_code};

use super::key_helper::{KeyCache, KeyCacheItemWrapper, KeyCacheType};

#[doc = "Writes given data to disk"]
pub async fn write_to_disk<T>(mut path: PathBuf, file_name: &str, data: &T) -> Result<()>
where
    T: Serialize + Sync,
{
    path.push(file_name);
    // Dummy key_cache since this method is not really meant to cache anything
    let key_cache = KeyCache::with_capacity(1);
    do_write_to_disk(&path, &key_cache, KeyCacheType::Unprotected, data)
        .await
        .map(|b| ())
}

#[doc = "Reads requested type from disk"]
pub async fn read_from_disk<T>(mut path: PathBuf, file_name: &str) -> Result<T>
where
    T: DeserializeOwned + Serialize,
{
    path.push(file_name);
    let key_cache = KeyCache::default();
    do_read_from_disk(&path, &key_cache, KeyCacheType::Unprotected).await
}

#[doc = "Reads local share from disk"]
#[instrument(level = "debug", name = "do_read_from_disk", skip(key_cache))]
pub async fn do_read_from_disk<T>(
    path: &PathBuf,
    key_cache: &KeyCache,
    key_cache_type_if_read_from_disk: KeyCacheType,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let key_path = path
        .to_str()
        .expect("Could not convert path to string")
        .to_string();

    let cache = key_cache.as_ref();

    if let Some(entry) = cache.get_async(&key_path).await {
        let local_key: T = entry
            .get()
            .data::<T, _>(|data| {
                ciborium::de::from_reader(data).map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeSystemFault,
                        Some(format!("Could not deserialize file: {:?}", path)),
                    )
                })
            })
            .await?;

        return Ok(local_key);
    }
    // Not in cache, read from disk

    // First, read the file content asynchronously into a buffer
    let mut file = fs::File::open(path).await.map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not open file: {:?}", path)),
        )
    })?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not read file: {:?}", path)),
        )
    })?;
    // Then, deserialize the buffer
    let local_key: T = ciborium::de::from_reader(&*buffer).map_err(|e| {
        unexpected_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not deserialize file: {:?}", path)),
        )
    })?;

    key_cache
        .as_ref()
        .insert_async(
            key_path,
            match key_cache_type_if_read_from_disk {
                KeyCacheType::Protected => KeyCacheItemWrapper::protected(&buffer),
                KeyCacheType::Unprotected => KeyCacheItemWrapper::unprotected(&buffer),
            },
        )
        .await
        .map_err(|_| unexpected_err("unable to insert into key cache", None))?;
    Ok(local_key)
}

#[doc = "Write local data to disk"]
#[instrument(level = "debug", name = "do_write_to_disk", skip(local_key, key_cache), ret(level = tracing::Level::TRACE))]
pub(crate) async fn do_write_to_disk<T>(
    path: &PathBuf,
    key_cache: &KeyCache,
    key_cache_type: KeyCacheType,
    local_key: &T,
) -> Result<()>
where
    T: Serialize + Sync,
{
    trace!("Writing to disk: {:?}", path);

    // CBOR
    let mut buffer = Vec::new();
    ciborium::into_writer(&local_key, &mut buffer).map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not write key file: {:?}", path)),
        )
    })?;

    let mut file = fs::File::create(path).await.map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!(
                "Could not open key file for writing: {:?}",
                path.clone()
            )),
        )
    })?;
    // Now write the buffer asynchronously
    file.write_all(&buffer).await.map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!(
                "Could not open key file for writing: {:?}",
                path.clone()
            )),
        )
    })?;
    add_to_cache(path, key_cache, key_cache_type, buffer).await
}

pub async fn do_write_to_cache_only<T>(
    path: &PathBuf,
    key_cache: &KeyCache,
    key_cache_type: KeyCacheType,
    local_key: &T,
) -> Result<()>
where
    T: Serialize + Sync,
{
    trace!("Writing to cache: {:?}", path);

    // CBOR
    let mut buffer = Vec::new();
    ciborium::into_writer(&local_key, &mut buffer).map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some(format!("Could not write key data: {:?}", path)),
        )
    })?;
    add_to_cache(path, key_cache, key_cache_type, buffer).await
}

async fn add_to_cache(
    path: &PathBuf,
    key_cache: &KeyCache,
    key_cache_type: KeyCacheType,
    buffer: Vec<u8>,
) -> Result<()> {
    let key_path = path
        .to_str()
        .expect("Could not convert path to string")
        .to_string();
    let cache = key_cache.as_ref();

    let wrapper = match key_cache_type {
        KeyCacheType::Protected => KeyCacheItemWrapper::protected(&buffer),
        KeyCacheType::Unprotected => KeyCacheItemWrapper::unprotected(&buffer),
    };
    if key_cache.as_ref().contains_async(&key_path).await {
        key_cache
            .as_ref()
            .update_async(&key_path, |_, _| wrapper)
            .await
            .ok_or(unexpected_err("unable to insert into key cache", None))?;
    } else {
        key_cache
            .as_ref()
            .insert_async(key_path, wrapper)
            .await
            .map_err(|_| unexpected_err("unable to insert into key cache", None))?;
    }

    Ok(())
}

pub async fn create_storage_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.exists().await {
        return Ok(());
    }

    if let Err(e) = fs::create_dir_all(path).await.map_err(|e| io_err(e, None)) {
        // Might happen during concurrent calls, we'll check below.
        if !path.exists().await {
            return Err(e);
        }
    }

    Ok(())
}
