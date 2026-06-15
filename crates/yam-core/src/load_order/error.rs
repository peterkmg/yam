use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadOrderError {
  #[error("failed to parse mods.settings: {0}")]
  ParseIni(#[from] ini::ParseError),
  #[error("load order entry {mod_name} is missing {field}")]
  MissingField {
    mod_name: String,
    field: &'static str,
  },
  #[error("invalid Enabled value for {mod_name}: {value}")]
  InvalidEnabled { mod_name: String, value: String },
  #[error("invalid Priority value for {mod_name}: {value}")]
  InvalidPriority { mod_name: String, value: String },
  #[error("priority for {mod_name} is outside 0..=9999: {priority}")]
  PriorityOutOfRange { mod_name: String, priority: i32 },
}
