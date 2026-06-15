#![allow(unused_crate_dependencies)]

use yam_core::{AppConfig, EnvironmentConfig};

#[test]
fn game_folder_config_uses_game_root_environment() {
  let config = AppConfig::game_folder("D:/Steam/W3");

  let EnvironmentConfig::GameFolder { game_root } = config.environment else {
    panic!("expected game folder config");
  };
  assert_eq!(game_root.mods_dir().file_name(), Some("Mods"));
}

#[test]
fn mod_organizer_config_stores_instance_root() {
  let config = AppConfig::mod_organizer("D:/w3-mo");

  let EnvironmentConfig::ModOrganizer(mo) = config.environment else {
    panic!("expected mo config");
  };
  assert_eq!(mo.instance_root(), "D:/w3-mo");
}
