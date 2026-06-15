use super::{ClassifiedConflict, ClassifiedConflictAction, ClassifiedConflictSource};
use crate::{LoadOrder, MergeCandidate, ScanReport};

#[must_use]
pub fn classify_conflicts(report: &ScanReport, load_order: &LoadOrder) -> Vec<ClassifiedConflict> {
  let conflicts = report
    .merge_candidates
    .iter()
    .map(|candidate| classify_candidate(candidate, load_order))
    .collect::<Vec<_>>();

  tracing::debug!(
    candidate_count = report.merge_candidates.len(),
    conflict_count = conflicts.len(),
    "conflicts classified"
  );
  conflicts
}

fn classify_candidate(candidate: &MergeCandidate, load_order: &LoadOrder) -> ClassifiedConflict {
  let mut sources = candidate
    .sources
    .iter()
    .cloned()
    .map(|source| ClassifiedConflictSource {
      load_order: load_order.state_for(&source.mod_name),
      source,
      is_winner: false,
    })
    .collect::<Vec<_>>();

  sources.sort_by(|left, right| {
    load_order.compare_sources(&left.source.mod_name, &right.source.mod_name)
  });

  let active_count = sources
    .iter()
    .filter(|source| source.load_order.enabled)
    .count();

  let action = conflict_action(candidate, active_count);

  if action == ClassifiedConflictAction::LoadOrderResolved
    && let Some(winner) = sources.iter_mut().find(|source| source.load_order.enabled)
  {
    winner.is_winner = true;
  }
  tracing::trace!(
    relative_path = %candidate.relative_path,
    action = ?action,
    active_count,
    source_count = sources.len(),
    "conflict candidate classified"
  );

  ClassifiedConflict {
    relative_path: candidate.relative_path.clone(),
    merge_file_type: candidate.merge_file_type,
    action,
    sources,
  }
}

const fn conflict_action(
  candidate: &MergeCandidate,
  active_count: usize,
) -> ClassifiedConflictAction {
  if active_count == 0 {
    return ClassifiedConflictAction::NoActiveSources;
  }

  if candidate.merge_file_type.is_some() && active_count > 1 {
    ClassifiedConflictAction::MergeRequired
  } else {
    ClassifiedConflictAction::LoadOrderResolved
  }
}
