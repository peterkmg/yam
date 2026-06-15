use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use yam_cache::{
  CacheEntryInput,
  CacheEntryKind,
  CacheStore,
  ContentHash,
  ProducerIdentity,
  ProducerKind,
};

use super::{ListedBundleEntry, ScanError};

const BUNDLE_INDEX_COMPATIBILITY_KEY: &[u8] = b"yam-core.bundle-index.v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BundleIndexMetadata {
  entries: Vec<ListedBundleEntry>,
}

pub trait BundleLister: std::fmt::Debug {
  fn list_bundle(&self, bundle_path: &Utf8Path) -> Result<Vec<ListedBundleEntry>, ScanError>;
}

impl<R> BundleLister for yam_tools::QuickBms<R>
where
  R: yam_tools::CommandRunner + std::fmt::Debug,
{
  fn list_bundle(&self, bundle_path: &Utf8Path) -> Result<Vec<ListedBundleEntry>, ScanError> {
    Ok(
      self
        .list_bundle(bundle_path.as_std_path())?
        .into_iter()
        .map(|entry| ListedBundleEntry::new(entry.path, entry.offset, entry.size))
        .collect::<Result<_, _>>()?,
    )
  }
}

pub fn default_bundle_index_compatibility_key() -> ContentHash {
  ContentHash::digest(BUNDLE_INDEX_COMPATIBILITY_KEY)
}

pub fn load_or_build(
  cache: &CacheStore,
  bundle_hash: ContentHash,
  compatibility_key: ContentHash,
  bundle_path: &Utf8Path,
  bundle_lister: Option<&dyn BundleLister>,
) -> Result<Option<Vec<ListedBundleEntry>>, ScanError> {
  let producer = ProducerIdentity {
    kind: ProducerKind::BundleIndexer,
    compatibility_key,
  };

  if let Some(entry) = cache.get_entry(CacheEntryKind::BundleIndex, &bundle_hash, &producer)? {
    let metadata: BundleIndexMetadata = serde_json::from_value(entry.input.metadata)?;
    return Ok(Some(metadata.entries));
  }

  let Some(bundle_lister) = bundle_lister else {
    return Ok(None);
  };

  let entries = bundle_lister.list_bundle(bundle_path)?;
  cache.put_entry(&CacheEntryInput {
    kind: CacheEntryKind::BundleIndex,
    input_key: bundle_hash,
    producer,
    output_hash: None,
    metadata: serde_json::to_value(BundleIndexMetadata {
      entries: entries.clone(),
    })?,
  })?;

  Ok(Some(entries))
}
