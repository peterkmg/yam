#![allow(unused_crate_dependencies)]

#[path = "support/candidate.rs"]
mod candidate;
#[path = "support/changed_sources.rs"]
mod changed_sources;
#[path = "support/files.rs"]
mod files;

use camino::Utf8Path;
use candidate::candidate;
use changed_sources::changed_source_names;
use files::write_file;
use tempfile::TempDir;
use yam_cache::CacheStore;
use yam_core::{GameRoot, MergeableFileType, ScanOptions, discover_game_folder, scan_environment};

#[test]
fn scan_environment_reports_cache_state_for_merge_sources() {
  let temp = TempDir::new().unwrap();
  let game_root = Utf8Path::from_path(temp.path()).unwrap();
  write_file(
    &game_root.join("Mods/modAlpha/content/scripts/game/player.ws"),
    "alpha",
  );
  write_file(
    &game_root.join("Mods/modBeta/content/scripts/game/player.ws"),
    "beta",
  );
  let env = discover_game_folder(&GameRoot::new(game_root.to_path_buf())).unwrap();
  let cache_root = game_root.join(".yam-cache");

  let cache = CacheStore::open(&cache_root).unwrap();
  let first = scan_environment(&env, &cache, ScanOptions::default()).unwrap();

  assert_eq!(first.files.len(), 2);
  assert_eq!(first.changed_file_count(), 2);
  assert_eq!(first.unchanged_file_count(), 0);
  assert_eq!(first.merge_candidates.len(), 1);
  assert!(
    first.merge_candidates[0]
      .sources
      .iter()
      .all(|item| item.changed)
  );

  let cache = CacheStore::open(&cache_root).unwrap();
  let second = scan_environment(&env, &cache, ScanOptions::default()).unwrap();

  assert_eq!(second.changed_file_count(), 0);
  assert_eq!(second.unchanged_file_count(), 2);
  assert!(
    second.merge_candidates[0]
      .sources
      .iter()
      .all(|item| !item.changed)
  );

  write_file(
    &game_root.join("Mods/modBeta/content/scripts/game/player.ws"),
    "beta changed",
  );
  let third = scan_environment(&env, &cache, ScanOptions::default()).unwrap();

  assert_eq!(third.changed_file_count(), 1);
  assert_eq!(third.unchanged_file_count(), 1);
  assert_eq!(changed_source_names(&third), ["modBeta"]);
}

#[test]
fn scan_environment_ignores_loose_unsupported_duplicates_as_merge_candidates() {
  let temp = TempDir::new().unwrap();
  let game_root = Utf8Path::from_path(temp.path()).unwrap();
  write_file(
    &game_root.join("Mods/modAlpha/content/en.w3strings"),
    "alpha",
  );
  write_file(&game_root.join("Mods/modBeta/content/en.w3strings"), "beta");

  let env = discover_game_folder(&GameRoot::new(game_root.to_path_buf())).unwrap();
  let cache = CacheStore::open(game_root.join(".yam-cache")).unwrap();
  let report = scan_environment(&env, &cache, ScanOptions::default()).unwrap();

  assert_eq!(report.files.len(), 2);
  assert!(report.merge_candidates.is_empty());
}

#[test]
fn scan_environment_keeps_loose_csv_duplicates_as_merge_candidates() {
  let temp = TempDir::new().unwrap();
  let game_root = Utf8Path::from_path(temp.path()).unwrap();
  write_file(
    &game_root.join("Mods/modAlpha/localization/en.csv"),
    "id,text\n1,a",
  );
  write_file(
    &game_root.join("Mods/modBeta/localization/en.csv"),
    "id,text\n1,b",
  );

  let env = discover_game_folder(&GameRoot::new(game_root.to_path_buf())).unwrap();
  let cache = CacheStore::open(game_root.join(".yam-cache")).unwrap();
  let report = scan_environment(&env, &cache, ScanOptions::default()).unwrap();

  let item = candidate(&report, "localization/en.csv");
  assert_eq!(item.merge_file_type, Some(MergeableFileType::Csv));
  assert_eq!(item.sources.len(), 2);
}
