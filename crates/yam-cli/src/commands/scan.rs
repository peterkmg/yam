use anyhow::Result;

use super::context::{self, ScanRequest};
use crate::{args::ScanCommand, output};

pub(super) fn run(command: ScanCommand) -> Result<()> {
  let ScanCommand {
    game_root,
    mo_root,
    profile,
    output_mod,
    cache_root,
    log: _,
    quickbms,
    quickbms_script,
  } = command;
  tracing::debug!(cache_root = %cache_root.display(), "scan started");
  let resolved = context::resolve_environment(game_root, mo_root, profile, output_mod)?;
  let report = context::scan_with_cache(ScanRequest {
    environment: &resolved,
    cache_root: &cache_root,
    quickbms,
    quickbms_script,
  })?;
  tracing::debug!(
    file_count = report.files.len(),
    bundle_count = report.bundles.len(),
    bundle_entry_count = report.bundle_entries.len(),
    candidate_count = report.merge_candidates.len(),
    "scan completed"
  );

  output::print_environment(&resolved);
  output::print_scan_summary(&report);
  Ok(())
}
