use std::{
  io,
  path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{ToolComponent, ToolKind};

#[derive(Debug, Error)]
pub enum ToolError {
  #[error("{tool} {component:?} path does not exist: {path}")]
  MissingPath {
    tool: ToolKind,
    component: ToolComponent,
    path: PathBuf,
  },
  #[error("{tool} command failed with exit code {status_code:?}")]
  CommandFailed {
    tool: ToolKind,
    status_code: Option<i32>,
    stdout: String,
    stderr: String,
  },
  #[error("{tool} reported failure: {message}")]
  ToolReportedFailure { tool: ToolKind, message: String },
  #[error("{tool} output could not be parsed: {line}")]
  Parse { tool: ToolKind, line: String },
  #[error("{tool} profile is missing required placeholder {{{placeholder}}}")]
  MissingRequiredPlaceholder {
    tool: ToolKind,
    placeholder: &'static str,
  },
  #[error("{tool} template argument contains an unknown placeholder: {argument}")]
  UnknownTemplatePlaceholder { tool: ToolKind, argument: String },
  #[error("{tool} could not find bundle entry {entry_path}")]
  MissingBundleEntry {
    tool: ToolKind,
    entry_path: String,
    output: String,
  },
  #[error("{tool} did not create output file: {path}")]
  MissingToolOutput { tool: ToolKind, path: PathBuf },
  #[error("extracted file was not created: {path}")]
  MissingExtractedFile { path: PathBuf },
  #[error("{tool} path is not a valid logical path: {path}")]
  InvalidLogicalPath {
    tool: ToolKind,
    path: String,
    #[source]
    source: yam_fs::FsError,
  },
  #[error("tool filesystem operation failed: {0}")]
  FileSystem(#[from] yam_fs::FsError),
  #[error("tool file operation failed: {source}")]
  Io {
    #[source]
    source: io::Error,
  },
  #[error("failed to run {tool}: {source}")]
  Run {
    tool: ToolKind,
    #[source]
    source: io::Error,
  },
}

pub fn require_file(
  tool: ToolKind,
  component: ToolComponent,
  path: &Path,
) -> Result<(), ToolError> {
  if path.is_file() {
    Ok(())
  } else {
    Err(ToolError::MissingPath {
      tool,
      component,
      path: path.to_path_buf(),
    })
  }
}

pub fn require_dir(tool: ToolKind, component: ToolComponent, path: &Path) -> Result<(), ToolError> {
  if path.is_dir() {
    Ok(())
  } else {
    Err(ToolError::MissingPath {
      tool,
      component,
      path: path.to_path_buf(),
    })
  }
}
