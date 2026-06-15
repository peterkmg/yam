#![allow(unused_crate_dependencies)]

use tempfile::TempDir;

mod support;

#[test]
fn scan_writes_debug_log_file() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  support::write_file(
    &game_root.join("Mods/modAlpha/content/scripts/game/player.ws"),
    "alpha",
  );

  let output = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--log")
    .arg("debug")
    .output()
    .unwrap();

  assert!(output.status.success());

  let stdout = support::stdout(output);
  assert!(stdout.contains("log: "));
  assert!(stdout.contains("yam.log"));

  let log = std::fs::read_to_string(cache_root.join("logs/yam.log")).unwrap();
  assert!(log.contains("scan started"));
  assert!(log.contains("scan completed"));
}

#[test]
fn scan_rotates_previous_log_file() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  support::write_file(
    &game_root.join("Mods/modAlpha/content/scripts/game/player.ws"),
    "alpha",
  );

  let first = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--log")
    .arg("debug")
    .output()
    .unwrap();
  assert!(first.status.success());

  support::write_file(
    &game_root.join("Mods/modBeta/content/scripts/game/player.ws"),
    "beta",
  );

  let second = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--log")
    .arg("debug")
    .output()
    .unwrap();
  assert!(second.status.success());

  let previous = std::fs::read_to_string(cache_root.join("logs/yam.previous.log")).unwrap();
  let current = std::fs::read_to_string(cache_root.join("logs/yam.log")).unwrap();

  assert!(previous.contains("file_count=1"));
  assert!(current.contains("file_count=2"));
}

#[test]
fn trace_log_includes_per_file_scan_detail() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  support::write_file(
    &game_root.join("Mods/modAlpha/content/scripts/game/player.ws"),
    "alpha",
  );

  let output = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--log")
    .arg("trace")
    .output()
    .unwrap();

  assert!(output.status.success());

  let log = std::fs::read_to_string(cache_root.join("logs/yam.log")).unwrap();
  assert!(log.contains("observed loose file"));
  assert!(log.contains("content/scripts/game/player.ws"));
}
