#![allow(unused_crate_dependencies)]

use tempfile::TempDir;

mod support;

#[test]
fn missing_load_order_path_is_reported() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  let load_order = temp.path().join("missing-mods.settings");

  std::fs::create_dir_all(game_root.join("Mods")).unwrap();

  let output = support::yam_command()
    .arg("conflicts")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--load-order")
    .arg(&load_order)
    .output()
    .unwrap();

  assert!(!output.status.success());

  let stderr = support::stderr(output);

  assert!(stderr.contains("failed to read load order"));
  assert!(stderr.contains("missing-mods.settings"));
}
