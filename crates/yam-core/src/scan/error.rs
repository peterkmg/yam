use thiserror::Error;
use yam_cache::CacheError;

#[derive(Debug, Error)]
pub enum ScanError {
  #[error("failed to update cache: {0}")]
  Cache(#[from] CacheError),
  #[error("external tool failed: {0}")]
  Tool(#[from] yam_tools::ToolError),
  #[error("filesystem operation failed: {0}")]
  FileSystem(#[from] yam_fs::FsError),
  #[error("failed to serialize cache metadata: {0}")]
  CacheMetadata(#[from] serde_json::Error),
}
