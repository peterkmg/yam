#![allow(unused_crate_dependencies)]

use camino::Utf8Path;
use rusqlite::Connection;
use tempfile::TempDir;
use yam_cache::CacheStore;

#[test]
fn migration_rejects_unknown_enum_storage_values() {
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

fn assert_constraint_fails(connection: &Connection, sql: &str) {
  let error = connection.execute(sql, []).unwrap_err();
  assert!(matches!(error, rusqlite::Error::SqliteFailure(_, _)));
}
