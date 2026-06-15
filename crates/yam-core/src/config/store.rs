use camino::Utf8Path;
use yam_fs::{TextEncoding, read_text_file, write_text_file};

use super::{AppConfig, ConfigError};

pub fn load_config(path: &Utf8Path) -> Result<AppConfig, ConfigError> {
  tracing::debug!(path = %path, "loading config");
  let text = read_text_file(path)?.text;

  let config = toml_edit::de::from_str(&text).map_err(|source| ConfigError::Parse {
    path: path.to_path_buf(),
    source,
  })?;
  tracing::debug!(path = %path, "config loaded");
  Ok(config)
}

pub fn save_config(path: &Utf8Path, config: &AppConfig) -> Result<(), ConfigError> {
  tracing::debug!(path = %path, "saving config");
  let text =
    toml_edit::ser::to_string_pretty(config).map_err(|source| ConfigError::Serialize { source })?;

  write_text_file(path, &text, TextEncoding::Utf8)?;
  tracing::debug!(path = %path, "config saved");

  Ok(())
}
