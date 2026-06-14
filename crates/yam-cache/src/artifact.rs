use std::str::FromStr;

use camino::Utf8PathBuf;

use crate::{CacheError, ContentHash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceId(String);

impl SourceId {
  #[must_use]
  pub fn new(value: impl Into<String>) -> Self {
    Self(value.into())
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalPath(String);

impl LogicalPath {
  #[must_use]
  pub fn new(value: impl Into<String>) -> Self {
    Self(value.into())
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceRole {
  Vanilla,
  Mod,
  Generated,
}

impl SourceRole {
  #[must_use]
  pub(crate) const fn as_storage(self) -> &'static str {
    match self {
      Self::Vanilla => "vanilla",
      Self::Mod => "mod",
      Self::Generated => "generated",
    }
  }
}

impl FromStr for SourceRole {
  type Err = CacheError;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value {
      "vanilla" => Ok(Self::Vanilla),
      "mod" => Ok(Self::Mod),
      "generated" => Ok(Self::Generated),
      _ => Err(CacheError::InvalidEnumValue {
        enum_name: "SourceRole",
        value: value.to_string(),
      }),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactKind {
  LooseFile,
  Bundle,
  BundleEntry,
}

impl ArtifactKind {
  #[must_use]
  pub(crate) const fn as_storage(self) -> &'static str {
    match self {
      Self::LooseFile => "loose_file",
      Self::Bundle => "bundle",
      Self::BundleEntry => "bundle_entry",
    }
  }
}

impl FromStr for ArtifactKind {
  type Err = CacheError;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value {
      "loose_file" => Ok(Self::LooseFile),
      "bundle" => Ok(Self::Bundle),
      "bundle_entry" => Ok(Self::BundleEntry),
      _ => Err(CacheError::InvalidEnumValue {
        enum_name: "ArtifactKind",
        value: value.to_string(),
      }),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactKey {
  pub source_id: SourceId,
  pub source_role: SourceRole,
  pub kind: ArtifactKind,
  pub logical_path: LogicalPath,
}

impl ArtifactKey {
  #[must_use]
  pub const fn new(
    source_id: SourceId,
    source_role: SourceRole,
    kind: ArtifactKind,
    logical_path: LogicalPath,
  ) -> Self {
    Self {
      source_id,
      source_role,
      kind,
      logical_path,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactInput {
  pub key: ArtifactKey,
  pub disk_path: Utf8PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRecord {
  pub id: i64,
  pub input: ArtifactInput,
  pub hash: ContentHash,
  pub byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationStatus {
  New,
  Unchanged,
  Changed { previous_hash: ContentHash },
}

impl ObservationStatus {
  #[must_use]
  pub const fn is_changed(&self) -> bool {
    !matches!(self, Self::Unchanged)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationResult {
  pub artifact: ArtifactRecord,
  pub status: ObservationStatus,
}
