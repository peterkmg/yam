use std::collections::BTreeMap;

use yam_merge::MergeableFileType;

use super::{MergeCandidate, MergeSource, MergeSourceLocation, ScannedBundleEntry, ScannedFile};

#[derive(Clone, Copy)]
enum SourceRef<'a> {
  LooseFile(&'a ScannedFile),
  BundleEntry(&'a ScannedBundleEntry),
}

pub fn merge_candidates_from_sources(
  files: &[ScannedFile],
  bundle_entries: &[ScannedBundleEntry],
) -> Vec<MergeCandidate> {
  let mut grouped = BTreeMap::<&str, Vec<SourceRef<'_>>>::new();

  for file in files {
    if file.merge_file_type.is_none() {
      continue;
    }

    grouped
      .entry(&file.relative_path)
      .or_default()
      .push(SourceRef::LooseFile(file));
  }

  for entry in bundle_entries {
    grouped
      .entry(&entry.relative_path)
      .or_default()
      .push(SourceRef::BundleEntry(entry));
  }

  grouped
    .into_iter()
    .filter(|(_, sources)| sources.len() > 1)
    .map(|(relative_path, sources)| MergeCandidate {
      merge_file_type: MergeableFileType::from_path(relative_path),
      relative_path: relative_path.to_string(),
      sources: sources.into_iter().map(merge_source).collect(),
    })
    .collect()
}

fn merge_source(source: SourceRef<'_>) -> MergeSource {
  match source {
    SourceRef::LooseFile(file) => MergeSource {
      mod_name: file.mod_name.clone(),
      location: MergeSourceLocation::LooseFile {
        path: file.path.clone(),
      },
      hash: file.hash,
      len: file.len,
      changed: file.changed,
    },
    SourceRef::BundleEntry(entry) => MergeSource {
      mod_name: entry.mod_name.clone(),
      location: MergeSourceLocation::BundleEntry {
        bundle_path: entry.bundle_path.clone(),
        bundle_relative_path: entry.bundle_relative_path.clone(),
        entry_path: entry.relative_path.clone(),
        offset: entry.offset,
      },
      hash: entry.bundle_hash,
      len: entry.len,
      changed: entry.bundle_changed,
    },
  }
}
