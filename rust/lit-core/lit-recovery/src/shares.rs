use crate::LitRecovery;
use crate::error::{Error, RecoveryResult};
use crate::models::DownloadedShareData;
use argon2::{Algorithm, Argon2, Params as Argon2Params, Version};
use rusqlite::{Connection, params, params_from_iter};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const COLUMN_SESSION_ID: &str = "session_id";
pub const COLUMN_ENCRYPTION_KEY: &str = "encryption_key";
pub const COLUMN_SUBNET_ID: &str = "subnet_id";
pub const COLUMN_CURVE: &str = "curve";
pub const COLUMN_DECRYPTION_KEY_SHARE: &str = "decryption_key_share";
pub const COLUMN_URL: &str = "url";

pub const COLUMNS: [&str; 6] = [
    COLUMN_SESSION_ID, COLUMN_ENCRYPTION_KEY, COLUMN_DECRYPTION_KEY_SHARE, COLUMN_SUBNET_ID,
    COLUMN_CURVE, COLUMN_URL,
];

/// Decryption Key Share and its Metadata
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareData {
    pub session_id: String,
    pub encryption_key: String,
    pub decryption_key_share: String,
    pub subnet_id: String,
    pub curve: String,
    pub url: String,
}

impl From<(DownloadedShareData, String)> for ShareData {
    fn from(value: (DownloadedShareData, String)) -> Self {
        Self {
            session_id: value.0.session_id,
            encryption_key: value.0.encryption_key,
            decryption_key_share: value.0.decryption_key_share,
            subnet_id: value.0.subnet_id,
            curve: value.0.curve,
            url: value.1,
        }
    }
}

pub struct ShareDatabase(Connection);

impl ShareDatabase {
    pub async fn open(recovery: &LitRecovery) -> RecoveryResult<Self> {
        let mut protected = recovery.db_key.lock().await;
        let db_key = protected
            .unprotect()
            .ok_or(Error::General("unable to unprotect database key".to_string()))?;
        let conn = Connection::open(get_share_database())?;
        init_db(&conn, db_key.as_ref())?;
        Ok(Self(conn))
    }

    pub async fn open_with_path(recovery: &LitRecovery, path: &PathBuf) -> RecoveryResult<Self> {
        let mut protected = recovery.db_key.lock().await;
        let db_key = protected
            .unprotect()
            .ok_or(Error::General("unable to unprotect database key".to_string()))?;
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let conn = Connection::open(path)?;
        init_db(&conn, db_key.as_ref())?;
        Ok(Self(conn))
    }

    pub fn open_with_path_and_secret(path: &PathBuf, secret: &[u8]) -> RecoveryResult<Self> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let conn = Connection::open(path)?;
        init_db(&conn, secret)?;
        Ok(Self(conn))
    }

    pub fn open_with_path_and_password(path: &PathBuf, password: &str) -> RecoveryResult<Self> {
        let params = Argon2Params::new(
            32 * 1024, // 32 KiB converted to bytes
            Argon2Params::DEFAULT_T_COST,
            Argon2Params::DEFAULT_P_COST,
            Some(Argon2Params::DEFAULT_OUTPUT_LEN),
        )
        .unwrap();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut okm = [0u8; 32];
        argon2.hash_password_into(password.as_bytes(), &[0xFF; 32], &mut okm).unwrap();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let connection = Connection::open(path)?;
        init_db(&connection, &okm)?;
        Ok(Self(connection))
    }

    pub fn get_shares(
        &self, columns: Option<&[&'static str]>, filter: Option<BTreeMap<&'static str, String>>,
    ) -> RecoveryResult<Vec<ShareData>> {
        let mut stmt_str = "SELECT ".to_string();
        if let Some(columns) = columns {
            stmt_str.push_str(&columns.join(","));
        } else {
            stmt_str.push('*');
        }
        stmt_str.push_str(" FROM shares");
        let mut params = vec![];
        if let Some(filter) = filter {
            stmt_str.push_str(" WHERE ");
            let mut first = true;
            for (key, value) in filter {
                if !first {
                    stmt_str.push_str(" AND ");
                }
                stmt_str.push_str(&format!("{} = ?", key));
                params.push(value);
                first = false;
            }
        }
        let mut stmt = self.0.prepare(&stmt_str)?;
        let shares_iter = stmt.query_map(params_from_iter(params.iter()), |row| {
            Ok(ShareData {
                session_id: row.get_or_default(COLUMN_SESSION_ID),
                encryption_key: row.get_or_default(COLUMN_ENCRYPTION_KEY),
                decryption_key_share: row.get_or_default(COLUMN_DECRYPTION_KEY_SHARE),
                subnet_id: row.get_or_default(COLUMN_SUBNET_ID),
                curve: row.get_or_default(COLUMN_CURVE),
                url: row.get_or_default(COLUMN_URL),
            })
        })?;
        let mut shares = vec![];
        for share in shares_iter {
            shares.push(share?);
        }
        Ok(shares)
    }

    pub fn insert_share(&self, share: &ShareData) -> RecoveryResult<()> {
        let sql = format!(
            "INSERT INTO shares ({}) VALUES({})",
            COLUMNS.join(", "),
            COLUMNS.map(|_| "?").join(", ")
        );
        let mut stmt = self.0.prepare(&sql)?;
        stmt.execute(params![
            share.session_id, share.encryption_key, share.decryption_key_share, share.subnet_id,
            share.curve, share.url,
        ])?;
        Ok(())
    }

    pub fn delete_share(&self, filter: &BTreeMap<&'static str, String>) -> RecoveryResult<()> {
        if filter.is_empty() {
            return Err(Error::General(
                "no specified filters means all shares would be deleted".to_string(),
            ));
        }
        let mut params = vec![];
        let mut stmt_str = "DELETE FROM shares WHERE ".to_string();
        let mut first = true;
        for (key, value) in filter {
            if !first {
                stmt_str.push_str(" AND ");
            }
            stmt_str.push_str(&format!("{} = ?", key));
            params.push(value);
            first = false;
        }
        let mut stmt = self.0.prepare(&stmt_str)?;
        stmt.execute(params_from_iter(params.iter()))?;
        Ok(())
    }
}

trait RowGetOrDefault {
    fn get_or_default<I: rusqlite::RowIndex, T: rusqlite::types::FromSql + Default>(
        &self, index: I,
    ) -> T;
}

impl RowGetOrDefault for rusqlite::Row<'_> {
    fn get_or_default<I: rusqlite::RowIndex, T: rusqlite::types::FromSql + Default>(
        &self, index: I,
    ) -> T {
        self.get(index).unwrap_or_default()
    }
}

fn init_db(conn: &Connection, db_key: &[u8]) -> RecoveryResult<()> {
    println!("db_key: {}", hex::encode(db_key));
    conn.pragma_update(None, "key", format!("x'{}'", hex::encode(db_key)))?;
    conn.pragma_update(None, "cipher_memory_security", "ON")?;
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS shares ({} TEXT NOT NULL, {} TEXT NOT NULL PRIMARY KEY, {} TEXT NOT NULL, {} TEXT NOT NULL, {} TEXT NOT NULL, {} TEXT NOT NULL)",
        COLUMNS[0], COLUMNS[1], COLUMNS[2], COLUMNS[3], COLUMNS[4], COLUMNS[5],
    );
    conn.execute(&sql, ())?;
    Ok(())
}

fn get_share_database() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        dirs::document_dir().unwrap_or_else(|| {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR").to_string()))
        })
    });
    path.push(format!(".{}", env!("CARGO_PKG_NAME")));

    if !path.is_dir() {
        fs::create_dir_all(&path).unwrap_or_else(|e| {
            panic!("Unable to create folder {}: {:?}", path.to_str().unwrap(), e)
        });
    }
    make_hidden(&path);
    let share_db_override = std::env::var("SHARE_DB").ok();
    let filename = match share_db_override {
        Some(s) => {
            if s.is_empty() {
                "share_data.db3".to_string()
            } else {
                s
            }
        }
        None => "share_data.db3".to_string(),
    };
    path.push(filename);
    path
}

#[cfg(target_os = "windows")]
fn make_hidden(path: &Path) {
    use std::ffi::CString;
    unsafe {
        let file_name = path.to_str().unwrap();
        winapi::um::fileapi::SetFileAttributesA(
            file_name.as_ptr(),
            0x2, // Hidden
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn make_hidden(_path: &Path) {}
