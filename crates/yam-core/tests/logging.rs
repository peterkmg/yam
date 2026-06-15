#![allow(unused_crate_dependencies)]

use std::str::FromStr;

use tempfile::TempDir;
use yam_core::logging::{FileLogPaths, LogLevel};

#[test]
fn log_level_accepts_cli_values() {
  assert_eq!(LogLevel::from_str("debug").unwrap(), LogLevel::Debug);
  assert_eq!(LogLevel::from_str("trace").unwrap(), LogLevel::Trace);
  assert!(LogLevel::from_str("info").is_err());
}

#[test]
fn file_log_paths_live_under_cache_logs_directory() {
  let temp = TempDir::new().unwrap();
  let paths = FileLogPaths::for_cache_root(temp.path());

  assert_eq!(paths.current, temp.path().join("logs/yam.log"));
  assert_eq!(paths.previous, temp.path().join("logs/yam.previous.log"));
}
