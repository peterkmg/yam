use anyhow::Result;
use yam_core::logging::init_file_logging;

use crate::{args, commands};

pub fn run_from_env() -> i32 {
  let cli: args::Cli = argh::from_env();
  let result = run(cli);

  if let Err(error) = &result {
    eprintln!("error: {error:#}");
  }

  i32::from(result.is_err())
}

fn run(cli: args::Cli) -> Result<()> {
  if let Some(options) = cli.command.file_logging_options() {
    let path = init_file_logging(options)?;
    println!("log: {}", path.display());
  }

  commands::run(cli.command)
}
