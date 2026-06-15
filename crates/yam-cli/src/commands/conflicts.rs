use anyhow::Result;
use yam_core::classify_conflicts;

use super::context::{self, ScanRequest};
use crate::{args::ConflictsCommand, output};

pub(super) fn run(command: ConflictsCommand) -> Result<()> {
  let ConflictsCommand {
    game_root,
    mo_root,
    profile,
    output_mod,
    cache_root,
    log: _,
    quickbms,
    quickbms_script,
    load_order,
  } = command;
  tracing::debug!(cache_root = %cache_root.display(), "conflict scan started");
  let resolved = context::resolve_environment(game_root, mo_root, profile, output_mod)?;
  let report = context::scan_with_cache(ScanRequest {
    environment: &resolved,
    cache_root: &cache_root,
    quickbms,
    quickbms_script,
  })?;
  let load_order = context::load_order(load_order)?;
  let conflicts = classify_conflicts(&report, &load_order);
  tracing::debug!(
    file_count = report.files.len(),
    candidate_count = report.merge_candidates.len(),
    conflict_count = conflicts.len(),
    "conflict scan completed"
  );

  output::print_environment(&resolved);
  output::print_scan_summary(&report);
  output::print_conflicts(&conflicts);
  Ok(())
}
