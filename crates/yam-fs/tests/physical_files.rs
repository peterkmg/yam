#![allow(unused_crate_dependencies)]

use std::fs;

use camino::Utf8Path;
use tempfile::TempDir;
use yam_fs::{FsError, remove_dir_contents, walk_files, write_bytes};

#[test]
fn walk_files_returns_sorted_relative_paths() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  write_bytes(root.join("zeta.ws"), b"script").unwrap();
  write_bytes(root.join("nested/alpha.xml"), b"<xml />").unwrap();

  let files = walk_files(root).unwrap();

  assert_eq!(
    files
      .iter()
      .map(|file| file.relative_path.as_str())
      .collect::<Vec<_>>(),
    ["nested/alpha.xml", "zeta.ws"]
  );
}

#[test]
fn walk_files_does_not_apply_ignore_rules() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  write_bytes(root.join(".gitignore"), b"*.ws").unwrap();
  write_bytes(root.join(".hidden.xml"), b"<xml />").unwrap();
  write_bytes(root.join("script.ws"), b"script").unwrap();

  let files = walk_files(root).unwrap();

  assert_eq!(
    files
      .iter()
      .map(|file| file.relative_path.as_str())
      .collect::<Vec<_>>(),
    [".gitignore", ".hidden.xml", "script.ws"]
  );
}

#[test]
fn write_bytes_creates_parent_directories() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let path = root.join("content/scripts/player.ws");

  write_bytes(&path, b"content").unwrap();

  assert_eq!(fs::read(path).unwrap(), b"content");
}

#[test]
fn guarded_cleanup_removes_children_but_keeps_directory() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  let target = root.join("output");
  write_bytes(target.join("nested/file.txt"), b"old").unwrap();

  remove_dir_contents(root, &target).unwrap();

  assert!(target.is_dir());
  assert!(!target.join("nested").exists());
}

#[test]
fn guarded_cleanup_rejects_root_directory() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  write_bytes(root.join("file.txt"), b"keep").unwrap();

  let error = remove_dir_contents(root, root).unwrap_err();

  assert!(matches!(error, FsError::RefusingRootDeletion { .. }));
  assert!(root.join("file.txt").is_file());
}
