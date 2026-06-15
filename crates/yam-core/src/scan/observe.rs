use camino::{Utf8Path, Utf8PathBuf};
use yam_cache::{
  ArtifactInput,
  ArtifactKey,
  ArtifactKind,
  CacheStore,
  ContentHash,
  LogicalPath,
  SourceId,
  SourceRole,
};
use yam_merge::MergeableFileType;

use super::{
  ListedBundleEntry,
  ScanError,
  ScannedBundle,
  ScannedBundleEntry,
  ScannedFile,
  bundle_index,
};
use crate::ResolvedMod;

pub fn observe_loose_file(
  mod_source: &ResolvedMod,
  relative_path: &str,
  path: Utf8PathBuf,
  cache: &CacheStore,
) -> Result<ScannedFile, ScanError> {
  let input = artifact_input(mod_source, ArtifactKind::LooseFile, relative_path, path)?;
  let result = cache.observe_file(&input)?;
  let artifact = result.artifact;
  let source_id = artifact.input.key.source_id.as_str().to_string();
  let logical_path = artifact.input.key.logical_path.as_str().to_string();
  let merge_file_type = MergeableFileType::from_path(&logical_path);

  Ok(ScannedFile {
    mod_name: source_id,
    relative_path: logical_path,
    path: artifact.input.disk_path,
    merge_file_type,
    hash: artifact.hash,
    len: artifact.byte_len,
    changed: result.status.is_changed(),
  })
}

pub fn observe_bundle(
  mod_source: &ResolvedMod,
  relative_path: &str,
  path: Utf8PathBuf,
  cache: &CacheStore,
  options: super::ScanOptions<'_>,
) -> Result<(ScannedBundle, Vec<ScannedBundleEntry>), ScanError> {
  let input = artifact_input(mod_source, ArtifactKind::Bundle, relative_path, path)?;
  let result = cache.observe_file(&input)?;
  let artifact = result.artifact;
  let source_id = artifact.input.key.source_id.as_str().to_string();
  let logical_path = artifact.input.key.logical_path.as_str().to_string();
  let disk_path = artifact.input.disk_path;
  let bundle_changed = result.status.is_changed();

  let listed_entries = bundle_index::load_or_build(
    cache,
    artifact.hash,
    options.bundle_index_compatibility_key,
    &disk_path,
    options.bundle_lister,
  )?;

  let entries = listed_entries.map_or_else(Vec::new, |entries| {
    scanned_bundle_entries(
      entries,
      &source_id,
      &logical_path,
      &disk_path,
      artifact.hash,
      bundle_changed,
    )
  });

  let bundle = ScannedBundle {
    mod_name: source_id,
    relative_path: logical_path,
    path: disk_path,
    hash: artifact.hash,
    len: artifact.byte_len,
    changed: bundle_changed,
  };

  Ok((bundle, entries))
}

fn artifact_input(
  mod_source: &ResolvedMod,
  kind: ArtifactKind,
  relative_path: &str,
  path: Utf8PathBuf,
) -> Result<ArtifactInput, ScanError> {
  Ok(ArtifactInput {
    key: ArtifactKey::new(
      SourceId::new(mod_source.name()),
      SourceRole::Mod,
      kind,
      LogicalPath::new(relative_path)?,
    ),
    disk_path: path,
  })
}

fn scanned_bundle_entries(
  entries: Vec<ListedBundleEntry>,
  mod_name: &str,
  bundle_relative_path: &str,
  bundle_path: &Utf8Path,
  bundle_hash: ContentHash,
  bundle_changed: bool,
) -> Vec<ScannedBundleEntry> {
  entries
    .into_iter()
    .map(|entry| ScannedBundleEntry {
      mod_name: mod_name.to_string(),
      merge_file_type: MergeableFileType::from_path(&entry.path),
      relative_path: entry.path,
      bundle_relative_path: bundle_relative_path.to_string(),
      bundle_path: bundle_path.to_path_buf(),
      offset: entry.offset,
      len: entry.len,
      bundle_hash,
      bundle_changed,
    })
    .collect()
}
