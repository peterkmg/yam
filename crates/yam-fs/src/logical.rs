use std::{
  fmt,
  path::{Path, PathBuf},
};

use relative_path::{Component, RelativePath, RelativePathBuf};

use crate::FsError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalPath(RelativePathBuf);

impl LogicalPath {
  pub fn new(path: impl AsRef<str>) -> Result<Self, FsError> {
    let raw = path.as_ref();
    let normalized = raw.replace('\\', "/");

    if normalized.trim().is_empty() {
      return invalid(raw, "path is empty");
    }

    if normalized.starts_with('/') {
      return invalid(raw, "absolute paths are not allowed");
    }

    let mut path = RelativePathBuf::new();
    for component in RelativePath::new(&normalized).components() {
      let Component::Normal(segment) = component else {
        return invalid(raw, "relative traversal is not allowed");
      };

      if segment.contains(':') {
        return invalid(raw, "drive or scheme separators are not allowed");
      }

      path.push(segment);
    }

    if path.as_relative_path().as_str().is_empty() {
      return invalid(raw, "path has no segments");
    }

    Ok(Self(path))
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    self.0.as_relative_path().as_str()
  }

  #[must_use]
  pub fn into_string(self) -> String {
    self.0.into_string()
  }

  #[must_use]
  pub fn to_disk_path(&self, root: impl AsRef<Path>) -> PathBuf {
    self.0.as_relative_path().to_path(root)
  }
}

impl fmt::Display for LogicalPath {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.as_str())
  }
}

fn invalid<T>(path: &str, reason: &'static str) -> Result<T, FsError> {
  Err(FsError::InvalidLogicalPath {
    path: path.to_string(),
    reason,
  })
}
