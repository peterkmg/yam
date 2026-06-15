use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameRoot {
  path: Utf8PathBuf,
}

impl GameRoot {
  #[must_use]
  pub fn new(path: impl Into<Utf8PathBuf>) -> Self {
    Self { path: path.into() }
  }

  #[must_use]
  pub const fn path(&self) -> &Utf8PathBuf {
    &self.path
  }

  #[must_use]
  pub fn mods_dir(&self) -> Utf8PathBuf {
    self.path.join("Mods")
  }
}
