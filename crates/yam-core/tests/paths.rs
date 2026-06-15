#![allow(unused_crate_dependencies)]

use yam_core::GameRoot;

#[test]
fn game_root_resolves_standard_mods_dir() {
  let root = GameRoot::new("D:/Games/The Witcher 3");
  let mods_dir = root.mods_dir();

  assert_eq!(mods_dir.parent(), Some(root.path().as_path()));
  assert_eq!(mods_dir.file_name(), Some("Mods"));
}
