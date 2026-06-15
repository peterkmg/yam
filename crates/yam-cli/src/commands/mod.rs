mod conflicts;
mod context;
mod inspect_env;
mod scan;

use anyhow::Result;

use crate::args::Command;

pub fn run(command: Command) -> Result<()> {
  match command {
    Command::InspectEnv(command) => inspect_env::run(command),
    Command::Scan(command) => scan::run(command),
    Command::Conflicts(command) => conflicts::run(command),
  }
}
