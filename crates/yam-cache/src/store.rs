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
    fs::create_dir_all(root.as_ref())?;

    let blob_root = root.as_ref().join(BLOB_DIR);
    fs::create_dir_all(&blob_root)?;

    let database_path = root.as_ref().join(DATABASE_FILE);
    let mut connection = Connection::open(&database_path)?;

    apply_pragmas(&connection)?;

    schema::migrate(&mut connection)?;

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
