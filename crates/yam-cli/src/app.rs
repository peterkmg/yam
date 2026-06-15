use crate::{args, commands};

pub fn run_from_env() -> i32 {
  init_tracing();

  let cli: args::Cli = argh::from_env();
  let result = commands::run(cli.command);
  if let Err(error) = &result {
    eprintln!("error: {error:#}");
  }

  i32::from(result.is_err())
}

fn init_tracing() {
  let _ = tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .try_init();
}
