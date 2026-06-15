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
    quickbms,
    quickbms_script,
  } = command;
  let resolved = context::resolve_environment(game_root, mo_root, profile, output_mod)?;
  let report = context::scan_with_cache(ScanRequest {
    environment: &resolved,
    cache_root: &cache_root,
    quickbms,
    quickbms_script,
  })?;

  output::print_environment(&resolved);
  output::print_scan_summary(&report);
  Ok(())
}
