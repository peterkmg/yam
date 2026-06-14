use std::path::Path;

use crate::{MergeError, engines};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeableFileType {
  WitcherScript,
  Xml,
  Csv,
}

impl MergeableFileType {
  #[must_use]
  pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
    match path
      .as_ref()
      .extension()
      .and_then(|extension| extension.to_str())
      .map(str::to_ascii_lowercase)
      .as_deref()
    {
      Some("ws") => Some(Self::WitcherScript),
      Some("xml") => Some(Self::Xml),
      Some("csv") => Some(Self::Csv),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergeInput<'a> {
  pub(crate) base: &'a str,
  pub(crate) ours: &'a str,
  pub(crate) theirs: &'a str,
}

impl<'a> MergeInput<'a> {
  #[must_use]
  pub const fn new(base: &'a str, ours: &'a str, theirs: &'a str) -> Self {
    Self { base, ours, theirs }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeResult {
  pub text: String,
  pub has_conflicts: bool,
}

impl MergeResult {
  #[must_use]
  pub fn new(text: String, has_conflicts: bool) -> Self {
    Self {
      has_conflicts: has_conflicts || text.contains("<<<<<<<"),
      text,
    }
  }

  #[must_use]
  pub const fn is_clean(&self) -> bool {
    !self.has_conflicts
  }

  #[must_use]
  pub fn conflict_count(&self) -> usize {
    self.text.matches("<<<<<<<").count()
  }
}

pub fn merge(
  file_type: MergeableFileType,
  input: MergeInput<'_>,
) -> Result<MergeResult, MergeError> {
  match file_type {
    MergeableFileType::WitcherScript => engines::mergiraf::merge(input),
    MergeableFileType::Xml => engines::xml_3dm::merge(input),
    MergeableFileType::Csv => Ok(engines::mergiraf::merge_by_line(input)),
  }
}
