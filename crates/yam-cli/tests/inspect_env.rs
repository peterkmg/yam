#![allow(unused_crate_dependencies)]

use tempfile::TempDir;

mod support;

#[test]
fn inspect_env_prints_direct_game_folder_mods() {
  let temp = TempDir::new().unwrap();
  std::fs::create_dir_all(temp.path().join("Mods/modAlpha")).unwrap();

  let output = support::yam_command()
    .arg("inspect-env")
    .arg("--game-root")
    .arg(temp.path())
    .output()
    .unwrap();

  assert!(output.status.success());

  let stdout = support::stdout(output);
  assert!(stdout.contains("mode: game-folder"));
  assert!(stdout.contains("mod: modAlpha"));
}

#[test]
fn scan_prints_cache_counts_for_direct_game_folder() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  support::write_file(
    &game_root.join("Mods/modAlpha/content/scripts/game/player.ws"),
    "alpha",
  );
  support::write_file(
    &game_root.join("Mods/modBeta/content/scripts/game/player.ws"),
    "beta",
  );

  let first = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .output()
    .unwrap();

  assert!(first.status.success());
  let first_stdout = support::stdout(first);
  assert!(first_stdout.contains("files: 2"));
  assert!(first_stdout.contains("changed files: 2"));
  assert!(first_stdout.contains("unchanged files: 0"));
  assert!(first_stdout.contains("merge candidates: 1"));

  let second = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .output()
    .unwrap();

  assert!(second.status.success());
  let second_stdout = support::stdout(second);
  assert!(second_stdout.contains("changed files: 0"));
  assert!(second_stdout.contains("unchanged files: 2"));
}
