use anyhow::Result;

use super::context;
use crate::{args::InspectEnvCommand, output};

pub(super) fn run(command: InspectEnvCommand) -> Result<()> {
  let InspectEnvCommand {
    game_root,
    mo_root,
    profile,
    output_mod,
  } = command;
  let resolved = context::resolve_environment(game_root, mo_root, profile, output_mod)?;

  output::print_environment(&resolved);
  Ok(())
}
