#![allow(unused_crate_dependencies)]

use tempfile::TempDir;

mod support;

#[test]
fn conflicts_prints_merge_required_sources_in_load_order() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  let load_order = temp.path().join("mods.settings");

  support::write_file(
    &game_root.join("Mods/modAlpha/content/scripts/game/player.ws"),
    "alpha",
  );
  support::write_file(
    &game_root.join("Mods/modBeta/content/scripts/game/player.ws"),
    "beta",
  );
  support::write_file(
    &load_order,
    "[modBeta]\nEnabled=1\nPriority=0\n\n[modAlpha]\nEnabled=1\nPriority=1\n",
  );

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

  assert!(output.status.success());

  let stdout = support::stdout(output);
  assert!(stdout.contains("conflicts: 1"));
  assert!(stdout.contains("merge required: 1"));
  assert!(stdout.contains("[merge-required] content/scripts/game/player.ws"));
  assert!(stdout.contains("type: witcherscript"));
  assert!(stdout.contains("source: modBeta priority=0 enabled changed"));
  assert!(stdout.contains("source: modAlpha priority=1 enabled changed"));
}

#[test]
fn conflicts_ignores_loose_unsupported_duplicates() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  let load_order = temp.path().join("mods.settings");

  support::write_file(
    &game_root.join("Mods/modAlpha/content/journal/text.txt"),
    "alpha",
  );
  support::write_file(
    &game_root.join("Mods/modBeta/content/journal/text.txt"),
    "beta",
  );
  support::write_file(
    &load_order,
    "[modBeta]\nEnabled=1\nPriority=0\n\n[modAlpha]\nEnabled=1\nPriority=1\n",
  );

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

  assert!(output.status.success());

  let stdout = support::stdout(output);
  assert!(stdout.contains("files: 2"));
  assert!(stdout.contains("merge candidates: 0"));
  assert!(stdout.contains("conflicts: 0"));
  assert!(stdout.contains("load-order resolved: 0"));
  assert!(!stdout.contains("content/journal/text.txt"));
}
