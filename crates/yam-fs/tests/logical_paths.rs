#![allow(unused_crate_dependencies)]

use tempfile::TempDir;
use yam_fs::{FsError, LogicalPath};

#[test]
fn logical_path_normalizes_separators() {
  let path = LogicalPath::new(r"gameplay\items//recipes.xml").unwrap();

  assert_eq!(path.as_str(), "gameplay/items/recipes.xml");
}

#[test]
fn logical_path_rejects_escape_segments() {
  let error = LogicalPath::new("gameplay/../user.settings").unwrap_err();

  assert!(matches!(error, FsError::InvalidLogicalPath { .. }));
}

#[test]
fn logical_path_rejects_drive_paths() {
  let error = LogicalPath::new("C:/mods/file.ws").unwrap_err();

  assert!(matches!(error, FsError::InvalidLogicalPath { .. }));
}

#[test]
fn logical_path_joins_under_output_root() {
  let temp = TempDir::new().unwrap();
  let path = LogicalPath::new(r"gameplay\items\recipes.xml").unwrap();

  let disk_path = path.to_disk_path(temp.path());

  assert_eq!(
    disk_path,
    temp
      .path()
      .join("gameplay")
      .join("items")
      .join("recipes.xml")
  );
}
