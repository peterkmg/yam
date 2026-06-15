#![allow(unused_crate_dependencies)]

#[path = "support/bundle_lister.rs"]
mod bundle_lister;
#[path = "support/candidate.rs"]
mod candidate;
#[path = "support/files.rs"]
mod files;

use bundle_lister::RecordingBundleLister;
use camino::Utf8Path;
use candidate::candidate;
use files::write_file;
use tempfile::TempDir;
use yam_cache::CacheStore;
use yam_core::{
  GameRoot,
  ListedBundleEntry,
  MergeSourceLocation,
  ScanOptions,
  discover_game_folder,
  scan_environment,
};

#[test]
fn scan_environment_tracks_bundles_without_listing_them_when_lister_is_missing() {
  let temp = TempDir::new().unwrap();
  let game_root = Utf8Path::from_path(temp.path()).unwrap();
  write_file(
    &game_root.join("Mods/modAlpha/content/blob0.bundle"),
    "alpha",
  );
  write_file(&game_root.join("Mods/modBeta/content/blob0.bundle"), "beta");

  let env = discover_game_folder(&GameRoot::new(game_root.to_path_buf())).unwrap();
  let cache = CacheStore::open(game_root.join(".yam-cache")).unwrap();
  let report = scan_environment(&env, &cache, ScanOptions::default()).unwrap();

  assert_eq!(report.files.len(), 0);
  assert_eq!(report.bundles.len(), 2);
  assert!(report.bundle_entries.is_empty());
  assert!(report.merge_candidates.is_empty());
}

#[test]
fn scan_environment_indexes_bundle_entries_and_groups_conflicts() {
  let temp = TempDir::new().unwrap();
  let game_root = Utf8Path::from_path(temp.path()).unwrap();
  let alpha_bundle = game_root.join("Mods/modAlpha/content/blob0.bundle");
  let beta_bundle = game_root.join("Mods/modBeta/content/blob0.bundle");
  write_file(&alpha_bundle, "alpha bundle");
  write_file(&beta_bundle, "beta bundle");

  let env = discover_game_folder(&GameRoot::new(game_root.to_path_buf())).unwrap();
  let cache = CacheStore::open(game_root.join(".yam-cache")).unwrap();
  let lister = RecordingBundleLister::new([
    (alpha_bundle.as_str(), vec![
      bundle_entry("gameplay/items/recipes.xml", 16, 200),
      bundle_entry("textures/preview.dds", 216, 400),
    ]),
    (beta_bundle.as_str(), vec![
      bundle_entry("gameplay/items/recipes.xml", 32, 250),
      bundle_entry("textures/preview.dds", 282, 450),
    ]),
  ]);

  let report = scan_environment(&env, &cache, ScanOptions {
    bundle_lister: Some(&lister),
    ..ScanOptions::default()
  })
  .unwrap();

  assert_eq!(lister.calls(), vec![alpha_bundle, beta_bundle]);
  assert_eq!(report.bundles.len(), 2);
  assert_eq!(report.bundle_entries.len(), 4);
  assert_eq!(report.merge_candidates.len(), 2);

  let xml_candidate = candidate(&report, "gameplay/items/recipes.xml");
  assert_eq!(xml_candidate.sources.len(), 2);
  assert!(xml_candidate.merge_file_type.is_some());
  assert!(
    xml_candidate
      .sources
      .iter()
      .all(|source| matches!(source.location, MergeSourceLocation::BundleEntry { .. }))
  );

  let texture_candidate = candidate(&report, "textures/preview.dds");
  assert_eq!(texture_candidate.sources.len(), 2);
  assert_eq!(texture_candidate.merge_file_type, None);
}

#[test]
fn scan_environment_reuses_cached_bundle_index_for_unchanged_bundles() {
  let temp = TempDir::new().unwrap();
  let game_root = Utf8Path::from_path(temp.path()).unwrap();
  let alpha_bundle = game_root.join("Mods/modAlpha/content/blob0.bundle");
  write_file(&alpha_bundle, "alpha bundle");

  let env = discover_game_folder(&GameRoot::new(game_root.to_path_buf())).unwrap();
  let cache = CacheStore::open(game_root.join(".yam-cache")).unwrap();
  let first_lister = RecordingBundleLister::new([(alpha_bundle.as_str(), vec![bundle_entry(
    "gameplay/items/recipes.xml",
    16,
    200,
  )])]);

  let first = scan_environment(&env, &cache, ScanOptions {
    bundle_lister: Some(&first_lister),
    ..ScanOptions::default()
  })
  .unwrap();

  let second_lister = RecordingBundleLister::new([]);
  let second = scan_environment(&env, &cache, ScanOptions {
    bundle_lister: Some(&second_lister),
    ..ScanOptions::default()
  })
  .unwrap();

  assert_eq!(first.bundle_entries.len(), 1);
  assert_eq!(second.bundle_entries.len(), 1);
  assert_eq!(
    first.bundle_entries[0].relative_path,
    second.bundle_entries[0].relative_path
  );
  assert_eq!(
    first.bundle_entries[0].offset,
    second.bundle_entries[0].offset
  );
  assert_eq!(first.bundle_entries[0].len, second.bundle_entries[0].len);
  assert!(first.bundle_entries[0].bundle_changed);
  assert!(!second.bundle_entries[0].bundle_changed);
  assert_eq!(first_lister.calls(), vec![alpha_bundle]);
  assert!(second_lister.calls().is_empty());
  assert_eq!(second.changed_bundle_count(), 0);
  assert_eq!(second.unchanged_bundle_count(), 1);
}

fn bundle_entry(path: &str, offset: u64, len: u64) -> ListedBundleEntry {
  ListedBundleEntry::new(path, offset, len).unwrap()
}
