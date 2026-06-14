#![allow(unused_crate_dependencies)]

mod support;

use serde_json::json;
use yam_cache::{CacheEntryInput, CacheEntryKind, ContentHash, ProducerKind};

#[test]
fn lookup_is_keyed_by_producer_compatibility() {
  let case = support::CacheCase::in_memory();
  let output = case.store().write_blob(b"merged").unwrap();
  let input_key = ContentHash::digest(b"input-set");
  let producer_v1 = support::producer(ProducerKind::ScriptMerger, b"mergiraf-profile-v1");
  let producer_v2 = support::producer(ProducerKind::ScriptMerger, b"mergiraf-profile-v2");

  let stored = case
    .store()
    .put_entry(&CacheEntryInput {
      kind: CacheEntryKind::MergeResult,
      input_key,
      producer: producer_v1,
      output_hash: Some(output.hash),
      metadata: json!({ "has_conflicts": false }),
    })
    .unwrap();

  let hit = case
    .store()
    .get_entry(CacheEntryKind::MergeResult, &input_key, &producer_v1)
    .unwrap()
    .unwrap();
  let miss = case
    .store()
    .get_entry(CacheEntryKind::MergeResult, &input_key, &producer_v2)
    .unwrap();

  assert_eq!(stored, hit);
  assert!(miss.is_none());
}

#[test]
fn metadata_can_be_stored_without_blob_output() {
  let case = support::CacheCase::in_memory();
  let input_key = ContentHash::digest(b"bundle-hash");
  let producer = support::producer(ProducerKind::BundleIndexer, b"quickbms-entry-index-v1");

  let stored = case
    .store()
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
