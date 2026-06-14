use std::fs;

use rusqlite::{OptionalExtension, params};

use crate::{CacheError, CacheStore, ContentHash, util::sqlite_len};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRef {
  pub hash: ContentHash,
  pub byte_len: u64,
}

impl CacheStore {
  pub fn write_blob(&self, bytes: &[u8]) -> Result<BlobRef, CacheError> {
    let hash = ContentHash::digest(bytes);
    let byte_len = bytes.len() as u64;
    let sqlite_byte_len = sqlite_len(byte_len, "blob byte length")?;
    let path = self.blob_path(&hash);

    if !path.is_file() {
      if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
      }

      fs::write(&path, bytes)?;
    }

    let mut statement = self.connection.prepare_cached(
      "INSERT INTO blobs (hash, byte_len)
       VALUES (?1, ?2)
       ON CONFLICT(hash) DO UPDATE SET byte_len = excluded.byte_len",
    )?;

    statement.execute(params![hash.as_bytes().as_slice(), sqlite_byte_len])?;

    Ok(BlobRef { hash, byte_len })
  }

  pub fn read_blob(&self, hash: &ContentHash) -> Result<Vec<u8>, CacheError> {
    let path = self.blob_path(hash);

    if !path.is_file() {
      return Err(CacheError::MissingBlob(hash.to_hex()));
    }

    Ok(fs::read(path)?)
  }

  pub fn has_blob(&self, hash: &ContentHash) -> Result<bool, CacheError> {
    if !self.blob_path(hash).is_file() {
      return Ok(false);
    }

    let mut statement = self
      .connection
      .prepare_cached("SELECT 1 FROM blobs WHERE hash = ?1")?;

    Ok(
      statement
        .query_row(params![hash.as_bytes().as_slice()], |_| Ok(()))
        .optional()?
        .is_some(),
    )
  }
}
