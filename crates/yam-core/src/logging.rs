use std::{
  fs::{self, File},
  io,
  path::{Path, PathBuf},
  str::FromStr,
};

use thiserror::Error;
use tracing_subscriber::EnvFilter;

const LOG_DIR: &str = "logs";
const CURRENT_LOG: &str = "yam.log";
const PREVIOUS_LOG: &str = "yam.previous.log";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
  Debug,
  Trace,
}

impl LogLevel {
  #[must_use]
  pub const fn as_filter(self) -> &'static str {
    match self {
      Self::Debug => "debug",
      Self::Trace => "trace",
    }
  }
}

impl FromStr for LogLevel {
  type Err = String;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value {
      "debug" => Ok(Self::Debug),
      "trace" => Ok(Self::Trace),
      _ => Err("expected debug or trace".to_string()),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLogPaths {
  pub current: PathBuf,
  pub previous: PathBuf,
}

impl FileLogPaths {
  #[must_use]
  pub fn for_cache_root(cache_root: &Path) -> Self {
    let log_dir = cache_root.join(LOG_DIR);

    Self {
      current: log_dir.join(CURRENT_LOG),
      previous: log_dir.join(PREVIOUS_LOG),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct FileLoggingOptions<'a> {
  pub cache_root: &'a Path,
  pub level: LogLevel,
}

#[derive(Debug, Error)]
pub enum LoggingError {
  #[error("failed to create log directory at {path}")]
  CreateLogDirectory { path: PathBuf, source: io::Error },

  #[error("failed to remove previous log at {path}")]
  RemovePreviousLog { path: PathBuf, source: io::Error },

  #[error("failed to rotate log from {current} to {previous}")]
  RotateLog {
    current: PathBuf,
    previous: PathBuf,
    source: io::Error,
  },

  #[error("failed to create log file at {path}")]
  CreateLogFile { path: PathBuf, source: io::Error },

  #[error("failed to initialize logging: {0}")]
  Initialize(String),
}

pub fn init_file_logging(options: FileLoggingOptions<'_>) -> Result<PathBuf, LoggingError> {
  let paths = prepare_log_file(options.cache_root)?;
  let file = File::create(&paths.current).map_err(|source| LoggingError::CreateLogFile {
    path: paths.current.clone(),
    source,
  })?;

  tracing_subscriber::fmt()
    .with_env_filter(env_filter(options.level))
    .with_writer(file)
    .with_ansi(false)
    .compact()
    .try_init()
    .map_err(|error| LoggingError::Initialize(error.to_string()))?;

  tracing::debug!(
    level = options.level.as_filter(),
    log_path = %paths.current.display(),
    "logging initialized"
  );

  Ok(paths.current)
}

fn prepare_log_file(cache_root: &Path) -> Result<FileLogPaths, LoggingError> {
  let paths = FileLogPaths::for_cache_root(cache_root);
  let log_dir = paths.current.parent().expect("log path has parent");

  fs::create_dir_all(log_dir).map_err(|source| LoggingError::CreateLogDirectory {
    path: log_dir.to_path_buf(),
    source,
  })?;

  if paths.current.exists() {
    if paths.previous.exists() {
      fs::remove_file(&paths.previous).map_err(|source| LoggingError::RemovePreviousLog {
        path: paths.previous.clone(),
        source,
      })?;
    }
    fs::rename(&paths.current, &paths.previous).map_err(|source| LoggingError::RotateLog {
      current: paths.current.clone(),
      previous: paths.previous.clone(),
      source,
    })?;
  }

  Ok(paths)
}

fn env_filter(level: LogLevel) -> EnvFilter {
  let level = level.as_filter();
  let mut filter = format!(
    "warn,yam={level},yam_cache={level},yam_cli={level},yam_core={level},yam_fs={level},\
     yam_merge={level},yam_tools={level}"
  );

  if let Ok(env_filter) = std::env::var(EnvFilter::DEFAULT_ENV)
    && !env_filter.trim().is_empty()
  {
    filter.push(',');
    filter.push_str(&env_filter);
  }

  EnvFilter::new(filter)
}
