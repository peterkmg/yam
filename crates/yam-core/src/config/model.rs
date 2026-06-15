use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use yam_tools::{EditorProfile, MergeToolProfile};

use crate::{GameRoot, ModOrganizerConfig};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
  pub environment: EnvironmentConfig,
  #[serde(default)]
  pub tools: ToolConfig,
  pub cache_dir: Option<Utf8PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentConfig {
  GameFolder { game_root: GameRoot },
  ModOrganizer(ModOrganizerConfig),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolConfig {
  pub quickbms: Option<QuickBmsConfig>,
  pub wcc_lite: Option<Utf8PathBuf>,
  #[serde(default)]
  pub manual_merge_tools: Vec<MergeToolProfile>,
  #[serde(default)]
  pub conflict_editors: Vec<EditorProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickBmsConfig {
  pub executable: Utf8PathBuf,
  pub script: Utf8PathBuf,
}

impl AppConfig {
  #[must_use]
  pub fn game_folder(game_root: impl Into<Utf8PathBuf>) -> Self {
    Self {
      environment: EnvironmentConfig::GameFolder {
        game_root: GameRoot::new(game_root),
      },
      tools: ToolConfig::default(),
      cache_dir: None,
    }
  }

  #[must_use]
  pub fn mod_organizer(instance_root: impl Into<Utf8PathBuf>) -> Self {
    Self {
      environment: EnvironmentConfig::ModOrganizer(ModOrganizerConfig::new(instance_root)),
      tools: ToolConfig::default(),
      cache_dir: None,
    }
  }
}
