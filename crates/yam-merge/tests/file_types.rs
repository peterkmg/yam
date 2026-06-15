#![allow(unused_crate_dependencies)]

use yam_merge::MergeableFileType;

#[test]
fn merge_file_type_matches_supported_extensions() {
  assert_eq!(
    MergeableFileType::from_path("content/scripts/game/player.ws"),
    Some(MergeableFileType::WitcherScript)
  );
  assert_eq!(
    MergeableFileType::from_path("content/items/recipes.XML"),
    Some(MergeableFileType::Xml)
  );
  assert_eq!(
    MergeableFileType::from_path("content/items/prices.csv"),
    Some(MergeableFileType::Csv)
  );
}

#[test]
fn merge_file_type_rejects_unsupported_extensions() {
  assert_eq!(MergeableFileType::from_path("content/blob0.bundle"), None);
  assert_eq!(
    MergeableFileType::from_path("content/journal/text.txt"),
    None
  );
  assert_eq!(
    MergeableFileType::from_path("content/scripts/game/player"),
    None
  );
}
