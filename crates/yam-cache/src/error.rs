use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
  #[error("cache I/O failed: {0}")]
  Io(#[from] io::Error),
  #[error("cache database failed: {0}")]
  Sqlite(#[from] rusqlite::Error),
  #[error("failed to migrate cache schema: {0}")]
  SchemaMigration(#[from] rusqlite_migration::Error),
  #[error("failed to encode cache metadata: {0}")]
  Json(#[from] serde_json::Error),
  #[error("cached blob is missing: {0}")]
  MissingBlob(String),
  #[error("invalid content hash: {0}")]
  InvalidContentHash(String),
  #[error("invalid {enum_name} value: {value}")]
  InvalidEnumValue {
    enum_name: &'static str,
    value: String,
  },
  #[error("cache value is too large for SQLite integer: {0}")]
  ValueTooLarge(&'static str),
}
