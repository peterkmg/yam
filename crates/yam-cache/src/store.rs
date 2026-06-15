use std::{
  fs,
  path::{Path, PathBuf},
};

use rusqlite::Connection;

use crate::{CacheError, ContentHash, schema};

const DATABASE_FILE: &str = "yam-cache.sqlite";
const BLOB_DIR: &str = "blobs";

pub struct CacheStore {
  database_path: PathBuf,
  pub(crate) blob_root: PathBuf,
  pub(crate) connection: Connection,
}

impl std::fmt::Debug for CacheStore {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CacheStore")
      .field("database_path", &self.database_path)
      .field("blob_root", &self.blob_root)
      .finish_non_exhaustive()
  }
}

impl CacheStore {
  pub fn open(root: impl AsRef<Path>) -> Result<Self, CacheError> {
    let root = root.as_ref();
    tracing::debug!(cache_root = %root.display(), "opening cache store");

    fs::create_dir_all(root)?;
    let blob_root = root.join(BLOB_DIR);
    let database_path = root.join(DATABASE_FILE);
    let connection = Connection::open(&database_path)?;

    Self::from_connection(database_path, blob_root, connection)
  }

  pub fn open_in_memory(blob_root: impl AsRef<Path>) -> Result<Self, CacheError> {
    let database_path = PathBuf::from(":memory:");
    let connection = Connection::open_in_memory()?;

    Self::from_connection(database_path, blob_root.as_ref().to_path_buf(), connection)
  }

  fn from_connection(
    database_path: PathBuf,
    blob_root: PathBuf,
    mut connection: Connection,
  ) -> Result<Self, CacheError> {
    fs::create_dir_all(&blob_root)?;
    apply_pragmas(&connection)?;
    schema::migrate(&mut connection)?;
    tracing::debug!(
      database_path = %database_path.display(),
      blob_root = %blob_root.display(),
      "cache store ready"
    );
    Ok(Self {
      database_path,
      blob_root,
      connection,
    })
  }

  pub fn schema_version(&self) -> Result<u32, CacheError> {
    Ok(
      self
        .connection
        .pragma_query_value(None, "user_version", |row| row.get::<_, u32>(0))?,
    )
  }

  pub(crate) fn blob_path(&self, hash: &ContentHash) -> PathBuf {
    let hex = hash.to_hex();
    self.blob_root.join(&hex[..2]).join(hex)
  }
}

fn apply_pragmas(connection: &Connection) -> Result<(), CacheError> {
  connection.execute_batch(
    "PRAGMA foreign_keys = ON;
     PRAGMA journal_mode = WAL;
     PRAGMA synchronous = NORMAL;
     PRAGMA busy_timeout = 5000;",
  )?;
  Ok(())
}
