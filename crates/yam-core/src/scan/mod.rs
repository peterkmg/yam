mod bundle_index;
mod candidates;
mod error;
mod model;
mod observe;
mod options;
mod walker;

pub use bundle_index::BundleLister;
pub use error::ScanError;
pub use model::{
  ListedBundleEntry,
  MergeCandidate,
  MergeSource,
  MergeSourceLocation,
  ScanReport,
  ScannedBundle,
  ScannedBundleEntry,
  ScannedFile,
};
pub use options::ScanOptions;
use yam_cache::CacheStore;

use crate::ResolvedEnvironment;

pub fn scan_environment(
  environment: &ResolvedEnvironment,
  cache: &CacheStore,
  options: ScanOptions<'_>,
) -> Result<ScanReport, ScanError> {
  let mut files = Vec::new();
  let mut bundles = Vec::new();
  let mut bundle_entries = Vec::new();

  for mod_source in &environment.mods {
    for discovered in walker::discover_paths(mod_source.path())? {
      let walker::DiscoveredPath {
        relative_path,
        path,
        kind,
      } = discovered;

      match kind {
        walker::DiscoveredPathKind::LooseFile => {
          files.push(observe::observe_loose_file(
            mod_source,
            &relative_path,
            path,
            cache,
          )?);
        }
        walker::DiscoveredPathKind::Bundle => {
          let (bundle, entries) =
            observe::observe_bundle(mod_source, &relative_path, path, cache, options)?;
          bundles.push(bundle);
          bundle_entries.extend(entries);
        }
      }
    }
  }

  let merge_candidates = candidates::merge_candidates_from_sources(&files, &bundle_entries);
  Ok(ScanReport {
    files,
    bundles,
    bundle_entries,
    merge_candidates,
  })
}
