use std::io;

use camino::Utf8PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvironmentError {
  #[error("required path does not exist: {0}")]
  MissingPath(Utf8PathBuf),
  #[error("Mod Organizer profile is not configured")]
  MissingProfile,
  #[error("failed to read {path}: {source}")]
  Read {
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[error("environment filesystem operation failed: {0}")]
  FileSystem(#[from] yam_fs::FsError),
  #[error("failed to parse INI file {path}: {source}")]
  ParseIni {
    path: Utf8PathBuf,
    source: ini::ParseError,
  },
}
