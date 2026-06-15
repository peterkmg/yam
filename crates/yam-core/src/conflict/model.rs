use yam_merge::MergeableFileType;

use crate::{MergeSource, SourceLoadOrder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassifiedConflictAction {
  MergeRequired,
  LoadOrderResolved,
  NoActiveSources,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifiedConflict {
  pub relative_path: String,
  pub merge_file_type: Option<MergeableFileType>,
  pub action: ClassifiedConflictAction,
  pub sources: Vec<ClassifiedConflictSource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifiedConflictSource {
  pub source: MergeSource,
  pub load_order: SourceLoadOrder,
  pub is_winner: bool,
}
