use thiserror::Error;

#[derive(Debug, Error)]
pub enum MergeError {
  #[error("XML merge failed: {0}")]
  Xml(#[from] xml_3dm::Error),
  #[error("Mergiraf language profile is unavailable: {0}")]
  MergirafLanguage(&'static str),
  #[error("Mergiraf reported merge issues without conflict markers")]
  UnmarkedMergeIssues,
  #[error("merge output was not valid UTF-8: {0}")]
  Utf8(#[from] std::string::FromUtf8Error),
  #[error("merge writer failed: {0}")]
  Write(#[from] std::io::Error),
}
