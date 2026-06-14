#![allow(unused_crate_dependencies)]

use std::fs;

use camino::Utf8Path;
use rusqlite::Connection;
use serde_json::json;
use tempfile::TempDir;
use yam_cache::{
  ArtifactInput,
  ArtifactKey,
  ArtifactKind,
  BlobRef,
  CacheEntryInput,
  CacheEntryKind,
  CacheError,
  CacheStore,
  ContentHash,
  LogicalPath,
  ObservationStatus,
  ProducerIdentity,
  ProducerKind,
  SourceId,
  SourceRole,
  schema_version,
};

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
fn observe_file_tracks_new_unchanged_and_changed_artifacts() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let file = root.join("input.ws");
  fs::write(&file, "content").unwrap();

  let store = CacheStore::open(root).unwrap();
  let first = store
    .observe_file(&artifact_input(&file, "modAlpha"))
    .unwrap();

  assert_eq!(first.status, ObservationStatus::New);
  assert_eq!(
    first.artifact.input.key.source_id,
    SourceId::new("modAlpha")
  );
  assert_eq!(first.artifact.input.key.kind, ArtifactKind::LooseFile);
  assert_eq!(
    first.artifact.input.key.logical_path,
    LogicalPath::new("content/scripts/input.ws")
  );
  assert_eq!(first.artifact.byte_len, 7);

  let second = store
    .observe_file(&artifact_input(&file, "modAlpha"))
    .unwrap();
  assert_eq!(second.status, ObservationStatus::Unchanged);
  assert_eq!(first.artifact.hash, second.artifact.hash);

  fs::write(&file, "changed").unwrap();
  let third = store
    .observe_file(&artifact_input(&file, "modAlpha"))
    .unwrap();

  assert_eq!(third.status, ObservationStatus::Changed {
    previous_hash: second.artifact.hash,
  });
  assert_ne!(second.artifact.hash, third.artifact.hash);
}

#[test]
fn observe_file_keeps_sources_independent() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let file = root.join("input.ws");
  fs::write(&file, "content").unwrap();

  let store = CacheStore::open(root).unwrap();
  let alpha = store
    .observe_file(&artifact_input(&file, "modAlpha"))
    .unwrap();
  let beta = store
    .observe_file(&artifact_input(&file, "modBeta"))
    .unwrap();

  assert_eq!(alpha.status, ObservationStatus::New);
  assert_eq!(beta.status, ObservationStatus::New);
  assert_eq!(
    alpha.artifact.input.key.logical_path,
    beta.artifact.input.key.logical_path
  );
  assert_ne!(
    alpha.artifact.input.key.source_id,
    beta.artifact.input.key.source_id
  );
}

#[test]
fn observe_many_records_files_in_one_call() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let alpha = root.join("alpha.ws");
  let beta = root.join("beta.ws");
  fs::write(&alpha, "alpha").unwrap();
  fs::write(&beta, "beta").unwrap();

  let mut store = CacheStore::open(root).unwrap();
  let first = store
    .observe_many(&[
      artifact_input_with_path(&alpha, "modAlpha", "content/alpha.ws"),
      artifact_input_with_path(&beta, "modBeta", "content/beta.ws"),
    ])
    .unwrap();
  let second = store
    .observe_many(&[
      artifact_input_with_path(&alpha, "modAlpha", "content/alpha.ws"),
      artifact_input_with_path(&beta, "modBeta", "content/beta.ws"),
    ])
    .unwrap();

  assert_eq!(first.len(), 2);
  assert!(first.iter().all(|result| result.status.is_changed()));
  assert!(second.iter().all(|result| !result.status.is_changed()));
}

#[test]
fn write_blob_deduplicates_by_hash() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let store = CacheStore::open(root).unwrap();

  let first = store.write_blob(b"merged output").unwrap();
  let second = store.write_blob(b"merged output").unwrap();

  assert_eq!(first, second);
  assert_eq!(first, BlobRef {
    hash: ContentHash::digest(b"merged output"),
    byte_len: 13,
  });
  assert!(store.has_blob(&first.hash).unwrap());
  assert_eq!(store.read_blob(&first.hash).unwrap(), b"merged output");
}

#[test]
fn cache_entry_lookup_is_keyed_by_producer_compatibility() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let store = CacheStore::open(root).unwrap();
  let output = store.write_blob(b"merged").unwrap();
  let input_key = ContentHash::digest(b"input-set");
  let producer_v1 = ProducerIdentity {
    kind: ProducerKind::ScriptMerger,
    compatibility_key: ContentHash::digest(b"mergiraf-profile-v1"),
  };
  let producer_v2 = ProducerIdentity {
    kind: ProducerKind::ScriptMerger,
    compatibility_key: ContentHash::digest(b"mergiraf-profile-v2"),
  };

  let stored = store
    .put_entry(&CacheEntryInput {
      kind: CacheEntryKind::MergeResult,
      input_key,
      producer: producer_v1,
      output_hash: Some(output.hash),
      metadata: json!({ "has_conflicts": false }),
    })
    .unwrap();

  let hit = store
    .get_entry(CacheEntryKind::MergeResult, &input_key, &producer_v1)
    .unwrap()
    .unwrap();
  let miss = store
    .get_entry(CacheEntryKind::MergeResult, &input_key, &producer_v2)
    .unwrap();

  assert_eq!(stored, hit);
  assert!(miss.is_none());
}

#[test]
fn cache_entry_can_store_metadata_without_blob_output() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let store = CacheStore::open(root).unwrap();
  let input_key = ContentHash::digest(b"bundle-hash");
  let producer = ProducerIdentity {
    kind: ProducerKind::BundleIndexer,
    compatibility_key: ContentHash::digest(b"quickbms-entry-index-v1"),
  };

  let stored = store
    .put_entry(&CacheEntryInput {
      kind: CacheEntryKind::BundleIndex,
      input_key,
      producer,
      output_hash: None,
      metadata: json!({ "entries": ["gameplay/items/items.xml"] }),
    })
    .unwrap();

  assert_eq!(stored.input.output_hash, None);
  assert_eq!(
    stored.input.metadata["entries"][0],
    "gameplay/items/items.xml"
  );
}

#[test]
fn future_schema_version_is_rejected() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  fs::create_dir_all(root).unwrap();
  let db_path = root.join("yam-cache.sqlite");
  let connection = Connection::open(&db_path).unwrap();
  connection
    .pragma_update(None, "user_version", schema_version() + 1)
    .unwrap();
  drop(connection);

  let error = CacheStore::open(root).unwrap_err();

  assert!(matches!(error, CacheError::SchemaMigration(_)));
}

#[test]
fn schema_rejects_unknown_enum_storage_values() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let store = CacheStore::open(root).unwrap();
  drop(store);

  let connection = Connection::open(root.join("yam-cache.sqlite")).unwrap();

  assert_constraint_fails(
    &connection,
    "INSERT INTO observed_artifacts
       (source_id, source_role, artifact_kind, logical_path, hash, byte_len)
     VALUES ('modAlpha', 'unknown', 'loose_file', 'content/a.ws', zeroblob(32), 1)",
  );
  assert_constraint_fails(
    &connection,
    "INSERT INTO observed_artifacts
       (source_id, source_role, artifact_kind, logical_path, hash, byte_len)
     VALUES ('modAlpha', 'mod', 'unknown', 'content/b.ws', zeroblob(32), 1)",
  );
  assert_constraint_fails(
    &connection,
    "INSERT INTO cache_entries
       (entry_kind, input_key, producer_kind, producer_compatibility_key, metadata_json)
     VALUES ('unknown', zeroblob(32), 'script_merger', zeroblob(32), '{}')",
  );
  assert_constraint_fails(
    &connection,
    "INSERT INTO cache_entries
       (entry_kind, input_key, producer_kind, producer_compatibility_key, metadata_json)
     VALUES ('merge_result', zeroblob(32), 'unknown', zeroblob(32), '{}')",
  );
}

fn artifact_input(path: &Utf8Path, source_id: &str) -> ArtifactInput {
  artifact_input_with_path(path, source_id, "content/scripts/input.ws")
}

fn artifact_input_with_path(
  disk_path: &Utf8Path,
  source_id: &str,
  logical_path: &str,
) -> ArtifactInput {
  ArtifactInput {
    key: ArtifactKey::new(
      SourceId::new(source_id),
      SourceRole::Mod,
      ArtifactKind::LooseFile,
      LogicalPath::new(logical_path),
    ),
    disk_path: disk_path.to_path_buf(),
  }
}

fn assert_constraint_fails(connection: &Connection, sql: &str) {
  let error = connection.execute(sql, []).unwrap_err();
  assert!(matches!(error, rusqlite::Error::SqliteFailure(_, _)));
}
