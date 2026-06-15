use camino::Utf8PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("config filesystem operation failed: {0}")]
  FileSystem(#[from] yam_fs::FsError),
  #[error("failed to parse config {path}: {source}")]
  Parse {
    path: Utf8PathBuf,
    source: toml_edit::de::Error,
  },
  #[error("failed to serialize config: {source}")]
  Serialize { source: toml_edit::ser::Error },
}
