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
  tracing::debug!(
    mode = %environment.kind,
    mod_count = environment.mods.len(),
    "scan environment started"
  );
  let mut files = Vec::new();
  let mut bundles = Vec::new();
  let mut bundle_entries = Vec::new();

  for mod_source in &environment.mods {
    let discovered_paths = walker::discover_paths(mod_source.path())?;
    tracing::debug!(
      mod_name = mod_source.name(),
      path = %mod_source.path(),
      discovered_count = discovered_paths.len(),
      "mod paths discovered"
    );

    for discovered in discovered_paths {
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
  tracing::debug!(
    file_count = files.len(),
    bundle_count = bundles.len(),
    bundle_entry_count = bundle_entries.len(),
    candidate_count = merge_candidates.len(),
    "scan environment completed"
  );
  Ok(ScanReport {
    files,
    bundles,
    bundle_entries,
    merge_candidates,
  })
}
