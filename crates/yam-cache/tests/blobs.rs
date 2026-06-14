#![allow(unused_crate_dependencies)]

mod support;

use yam_cache::{BlobRef, ContentHash};

#[test]
fn write_blob_deduplicates_by_hash() {
  let case = support::CacheCase::in_memory();

  let first = case.store().write_blob(b"merged output").unwrap();
  let second = case.store().write_blob(b"merged output").unwrap();

  assert_eq!(first, second);
  assert_eq!(first, BlobRef {
    hash: ContentHash::digest(b"merged output"),
    byte_len: 13,
  });
  assert!(case.store().has_blob(&first.hash).unwrap());
  assert_eq!(
    case.store().read_blob(&first.hash).unwrap(),
    b"merged output"
  );
}
