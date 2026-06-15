use yam_cache::ContentHash;

use super::{BundleLister, bundle_index};

#[derive(Debug, Clone, Copy)]
pub struct ScanOptions<'a> {
  pub bundle_lister: Option<&'a dyn BundleLister>,
  pub bundle_index_compatibility_key: ContentHash,
}

impl Default for ScanOptions<'_> {
  fn default() -> Self {
    Self {
      bundle_lister: None,
      bundle_index_compatibility_key: bundle_index::default_bundle_index_compatibility_key(),
    }
  }
}
