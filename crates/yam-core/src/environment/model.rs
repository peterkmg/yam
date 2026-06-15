use std::fmt;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

pub const DEFAULT_OUTPUT_MOD_NAME: &str = "mod0000_MergedFiles";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModEnvironmentKind {
  GameFolder,
  ModOrganizer,
}

impl fmt::Display for ModEnvironmentKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::GameFolder => f.write_str("game-folder"),
      Self::ModOrganizer => f.write_str("mod-organizer"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModOrganizerConfig {
  instance_root: Utf8PathBuf,
  profile: Option<String>,
  output_mod: String,
}

impl ModOrganizerConfig {
  #[must_use]
  pub fn new(instance_root: impl Into<Utf8PathBuf>) -> Self {
    Self {
      instance_root: instance_root.into(),
      profile: None,
      output_mod: DEFAULT_OUTPUT_MOD_NAME.to_string(),
    }
  }

  #[must_use]
  pub fn with_profile(mut self, profile: impl Into<String>) -> Self {
    self.profile = Some(profile.into());
    self
  }

  #[must_use]
  pub fn with_output_mod(mut self, output_mod: impl Into<String>) -> Self {
    self.output_mod = output_mod.into();
    self
  }

  #[must_use]
  pub fn instance_root(&self) -> &Utf8Path {
    &self.instance_root
  }

  #[must_use]
  pub fn profile(&self) -> Option<&str> {
    self.profile.as_deref()
  }

  #[must_use]
  pub fn output_mod(&self) -> &str {
    &self.output_mod
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputMod {
  pub name: String,
  pub path: Utf8PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedMod {
  name: String,
  path: Utf8PathBuf,
}

impl ResolvedMod {
  #[must_use]
  pub fn new(name: impl Into<String>, path: impl Into<Utf8PathBuf>) -> Self {
    Self {
      name: name.into(),
      path: path.into(),
    }
  }

  #[must_use]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[must_use]
  pub const fn path(&self) -> &Utf8PathBuf {
    &self.path
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedEnvironment {
  pub kind: ModEnvironmentKind,
  pub root: Utf8PathBuf,
  pub mods_dir: Utf8PathBuf,
  pub output_mod: OutputMod,
  pub profile: Option<String>,
  pub mods: Vec<ResolvedMod>,
}
