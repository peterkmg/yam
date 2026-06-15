#![cfg_attr(test, allow(unused_crate_dependencies))]

mod app;
mod args;
mod commands;
mod output;

fn main() {
  let exit_code = app::run_from_env();
  if exit_code != 0 {
    std::process::exit(exit_code);
  }
}
