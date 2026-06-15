use camino::Utf8PathBuf;
use yam_cache::ContentHash;
use yam_fs::{FsError, LogicalPath};
use yam_merge::MergeableFileType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedFile {
  pub mod_name: String,
  pub relative_path: String,
  pub path: Utf8PathBuf,
  pub merge_file_type: Option<MergeableFileType>,
  pub hash: ContentHash,
  pub len: u64,
  pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedBundle {
  pub mod_name: String,
  pub relative_path: String,
  pub path: Utf8PathBuf,
  pub hash: ContentHash,
  pub len: u64,
  pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ListedBundleEntry {
  pub path: String,
  pub offset: u64,
  pub len: u64,
}

impl ListedBundleEntry {
  pub fn new(path: impl AsRef<str>, offset: u64, len: u64) -> Result<Self, FsError> {
    Ok(Self {
      path: LogicalPath::new(path)?.into_string(),
      offset,
      len,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedBundleEntry {
  pub mod_name: String,
  pub relative_path: String,
  pub bundle_relative_path: String,
  pub bundle_path: Utf8PathBuf,
  pub offset: u64,
  pub len: u64,
  pub merge_file_type: Option<MergeableFileType>,
  pub bundle_hash: ContentHash,
  pub bundle_changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeSourceLocation {
  LooseFile {
    path: Utf8PathBuf,
  },
  BundleEntry {
    bundle_path: Utf8PathBuf,
    bundle_relative_path: String,
    entry_path: String,
    offset: u64,
  },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeSource {
  pub mod_name: String,
  pub location: MergeSourceLocation,
  pub hash: ContentHash,
  pub len: u64,
  pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeCandidate {
  pub relative_path: String,
  pub merge_file_type: Option<MergeableFileType>,
  pub sources: Vec<MergeSource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanReport {
  pub files: Vec<ScannedFile>,
  pub bundles: Vec<ScannedBundle>,
  pub bundle_entries: Vec<ScannedBundleEntry>,
  pub merge_candidates: Vec<MergeCandidate>,
}

impl ScanReport {
  #[must_use]
  pub fn changed_file_count(&self) -> usize {
    self.files.iter().filter(|file| file.changed).count()
  }

  #[must_use]
  pub fn unchanged_file_count(&self) -> usize {
    self.files.len() - self.changed_file_count()
  }

  #[must_use]
  pub fn changed_bundle_count(&self) -> usize {
    self.bundles.iter().filter(|bundle| bundle.changed).count()
  }

  #[must_use]
  pub fn unchanged_bundle_count(&self) -> usize {
    self.bundles.len() - self.changed_bundle_count()
  }
}
