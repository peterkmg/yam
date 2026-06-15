use std::{io, path::PathBuf, str::Utf8Error, string::FromUtf16Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
  #[error("failed to {operation} {path}: {source}")]
  Io {
    operation: &'static str,
    path: PathBuf,
    source: io::Error,
  },
  #[error("failed while walking files: {source}")]
  Walk { source: ignore::Error },
  #[error("path is not valid UTF-8: {path}")]
  NonUtf8Path { path: PathBuf },
  #[error("path {path} is not under root {root}")]
  PathOutsideRoot { root: PathBuf, path: PathBuf },
  #[error("refusing to remove root directory contents: {root}")]
  RefusingRootDeletion { root: PathBuf },
  #[error("invalid logical path {path}: {reason}")]
  InvalidLogicalPath { path: String, reason: &'static str },
  #[error("invalid UTF-8 text: {source}")]
  InvalidUtf8 { source: Utf8Error },
  #[error("invalid UTF-16 text: {source}")]
  InvalidUtf16 { source: FromUtf16Error },
  #[error("UTF-16 text has an odd byte length")]
  OddUtf16Length,
}

impl FsError {
  pub(crate) fn io(operation: &'static str, path: impl Into<PathBuf>, source: io::Error) -> Self {
    Self::Io {
      operation,
      path: path.into(),
      source,
    }
  }
}
