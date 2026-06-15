use camino::{Utf8Path, Utf8PathBuf};
use yam_fs::walk_files;

use super::ScanError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredPath {
  pub relative_path: String,
  pub path: Utf8PathBuf,
  pub kind: DiscoveredPathKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveredPathKind {
  LooseFile,
  Bundle,
}

pub fn discover_paths(root: &Utf8Path) -> Result<Vec<DiscoveredPath>, ScanError> {
  Ok(
    walk_files(root)?
      .into_iter()
      .map(|file| DiscoveredPath {
        relative_path: file.relative_path.into_string(),
        kind: file_kind(&file.path),
        path: file.path,
      })
      .collect(),
  )
}

fn file_kind(path: &Utf8Path) -> DiscoveredPathKind {
  if is_bundle_path(path) {
    DiscoveredPathKind::Bundle
  } else {
    DiscoveredPathKind::LooseFile
  }
}

fn is_bundle_path(path: &Utf8Path) -> bool {
  path
    .extension()
    .is_some_and(|extension| extension.eq_ignore_ascii_case("bundle"))
}
