#![allow(unused_crate_dependencies)]

mod support;

use camino::Utf8Path;
use rusqlite::Connection;
use tempfile::TempDir;
use yam_cache::{CacheError, CacheStore, schema_version};

#[test]
fn open_creates_sqlite_database_and_blob_directory() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();

  let store = CacheStore::open(root).unwrap();

  assert_eq!(store.schema_version().unwrap(), schema_version());
  assert!(root.join("yam-cache.sqlite").is_file());
  assert!(root.join("blobs").is_dir());
}

#[test]
fn open_in_memory_initializes_schema_and_blob_directory() {
  let case = support::CacheCase::in_memory();

  assert_eq!(case.store().schema_version().unwrap(), schema_version());
  assert!(case.root().is_dir());
  assert!(!case.root().join("yam-cache.sqlite").exists());
}

#[test]
fn future_schema_version_is_rejected() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  std::fs::create_dir_all(root).unwrap();
  let db_path = root.join("yam-cache.sqlite");
  let connection = Connection::open(&db_path).unwrap();
  connection
    .pragma_update(None, "user_version", schema_version() + 1)
    .unwrap();
  drop(connection);

  let error = CacheStore::open(root).unwrap_err();

  assert!(matches!(error, CacheError::SchemaMigration(_)));
}
