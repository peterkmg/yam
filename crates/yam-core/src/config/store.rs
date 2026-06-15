use camino::Utf8Path;
use yam_fs::{TextEncoding, read_text_file, write_text_file};

use super::{AppConfig, ConfigError};

pub fn load_config(path: &Utf8Path) -> Result<AppConfig, ConfigError> {
  let text = read_text_file(path)?.text;

  toml_edit::de::from_str(&text).map_err(|source| ConfigError::Parse {
    path: path.to_path_buf(),
    source,
  })
}

pub fn save_config(path: &Utf8Path, config: &AppConfig) -> Result<(), ConfigError> {
  let text =
    toml_edit::ser::to_string_pretty(config).map_err(|source| ConfigError::Serialize { source })?;

  write_text_file(path, &text, TextEncoding::Utf8)?;

  Ok(())
}
