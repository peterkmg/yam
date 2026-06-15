#![allow(unused_crate_dependencies)]

use std::fs;

use camino::Utf8Path;
use tempfile::TempDir;
use yam_core::{AppConfig, EnvironmentConfig, QuickBmsConfig, load_config, save_config};
use yam_tools::{EditorProfile, LaunchMode, MergeToolProfile, PathMapping, ToolCommand};

#[test]
fn app_config_round_trips_toml_with_tool_profiles() {
  let temp = TempDir::new().unwrap();
  let path = Utf8Path::from_path(temp.path()).unwrap().join("yam.toml");
  let config = config_with_tool_profiles();

  save_config(&path, &config).unwrap();

  let loaded = load_config(&path).unwrap();
  assert_eq!(loaded, config);

  let text = fs::read_to_string(path).unwrap();
  assert!(text.contains("[[tools.manual_merge_tools]]"));
  assert!(text.contains("[[tools.conflict_editors]]"));
  assert!(text.contains("[tools.quickbms]"));
  assert!(text.contains("script = "));
  assert!(text.contains("args = ["));
}

fn config_with_tool_profiles() -> AppConfig {
  let mut config = AppConfig::mod_organizer("D:/w3-mo");
  config.cache_dir = Some("D:/yam/cache".into());
  config.tools.quickbms = Some(QuickBmsConfig {
    executable: "D:/tools/quickbms.exe".into(),
    script: "D:/tools/witcher3.bms".into(),
  });
  config.tools.wcc_lite = Some("D:/Witcher 3/bin/x64/wcc_lite.exe".into());
  config
    .tools
    .manual_merge_tools
    .push(beyond_compare_profile());
  config.tools.conflict_editors.push(vs_code_profile());

  config
}

fn beyond_compare_profile() -> MergeToolProfile {
  MergeToolProfile::new(
    "Beyond Compare",
    ToolCommand::new("C:/Program Files/Beyond Compare 5/BComp.exe", vec![
      "{left}",
      "{right}",
      "{base}",
      "/mergeoutput={output}",
      "/reviewconflicts",
    ])
    .with_success_exit_codes(vec![0, 1]),
  )
}

fn vs_code_profile() -> EditorProfile {
  let launch_mode = LaunchMode::wine("wine").with_path_mapping(PathMapping::new("D:/", "Z:\\"));

  EditorProfile::new(
    "VS Code",
    ToolCommand::new(
      "C:/Users/pk/AppData/Local/Programs/Microsoft VS Code/Code.exe",
      vec!["--goto", "{file}:{line}:{column}"],
    )
    .with_launch_mode(launch_mode),
  )
}

#[test]
fn config_store_keeps_environment_variant() {
  let temp = TempDir::new().unwrap();
  let path = Utf8Path::from_path(temp.path()).unwrap().join("yam.toml");
  let config = AppConfig::game_folder("D:/Steam/W3");

  save_config(&path, &config).unwrap();

  let loaded = load_config(&path).unwrap();
  let EnvironmentConfig::GameFolder { game_root } = loaded.environment else {
    panic!("expected game folder environment");
  };
  assert_eq!(game_root.path(), "D:/Steam/W3");
}
