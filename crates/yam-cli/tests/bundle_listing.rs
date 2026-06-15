#![allow(unused_crate_dependencies)]

use tempfile::TempDir;

mod support;

#[test]
fn scan_lists_bundle_entries_with_quickbms_options() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  let quickbms = temp.path().join(quickbms_tool_name());
  let script = temp.path().join("witcher3.bms");

  support::write_file(
    &game_root.join("Mods/modAlpha/content/blob0.bundle"),
    "alpha",
  );
  support::write_file(&game_root.join("Mods/modBeta/content/blob0.bundle"), "beta");
  support::write_file(&script, "script body");
  support::write_quickbms_list_tool(&quickbms, &[
    "00000010 200 gameplay/items/recipes.xml",
    "00000020 300 textures/preview.dds",
  ]);

  let output = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--quickbms")
    .arg(&quickbms)
    .arg("--quickbms-script")
    .arg(&script)
    .output()
    .unwrap();

  assert!(output.status.success());

  let stdout = support::stdout(output);
  assert!(stdout.contains("bundles: 2"));
  assert!(stdout.contains("bundle entries: 4"));
  assert!(stdout.contains("merge candidates: 2"));
}

#[test]
fn conflicts_reports_bundle_entries_with_quickbms_options() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  let quickbms = temp.path().join(quickbms_tool_name());
  let script = temp.path().join("witcher3.bms");

  support::write_file(
    &game_root.join("Mods/modAlpha/content/blob0.bundle"),
    "alpha",
  );
  support::write_file(&game_root.join("Mods/modBeta/content/blob0.bundle"), "beta");
  support::write_file(&script, "script body");
  support::write_quickbms_list_tool(&quickbms, &["00000010 200 gameplay/items/recipes.xml"]);

  let output = support::yam_command()
    .arg("conflicts")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--quickbms")
    .arg(&quickbms)
    .arg("--quickbms-script")
    .arg(&script)
    .output()
    .unwrap();

  assert!(output.status.success());

  let stdout = support::stdout(output);
  assert!(stdout.contains("bundle entries: 2"));
  assert!(stdout.contains("conflicts: 1"));
  assert!(stdout.contains("merge required: 1"));
  assert!(stdout.contains("[merge-required] gameplay/items/recipes.xml"));
  assert!(stdout.contains("type: xml"));
}

#[test]
fn quickbms_options_must_be_provided_as_a_pair() {
  let temp = TempDir::new().unwrap();
  let game_root = temp.path().join("game");
  let cache_root = temp.path().join("cache");
  let quickbms = temp.path().join(quickbms_tool_name());

  std::fs::create_dir_all(game_root.join("Mods")).unwrap();
  support::write_quickbms_list_tool(&quickbms, &["00000010 200 gameplay/items/recipes.xml"]);

  let output = support::yam_command()
    .arg("scan")
    .arg("--game-root")
    .arg(&game_root)
    .arg("--cache-root")
    .arg(&cache_root)
    .arg("--quickbms")
    .arg(&quickbms)
    .output()
    .unwrap();

  assert!(!output.status.success());
  assert!(support::stderr(output).contains("provide --quickbms-script when using --quickbms"));
}

#[cfg(windows)]
const fn quickbms_tool_name() -> &'static str {
  "quickbms.cmd"
}

#[cfg(not(windows))]
const fn quickbms_tool_name() -> &'static str {
  "quickbms"
}
