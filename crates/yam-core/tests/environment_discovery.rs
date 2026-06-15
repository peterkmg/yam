#![allow(unused_crate_dependencies)]

use std::fs;

use camino::Utf8Path;
use tempfile::TempDir;
use yam_core::{
  GameRoot,
  ModEnvironmentKind,
  ModOrganizerConfig,
  ResolvedMod,
  discover_game_folder,
  discover_mod_organizer,
};

#[test]
fn game_folder_discovers_mod_dirs_and_default_output() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  fs::create_dir_all(root.join("Mods/modBeta")).unwrap();
  fs::create_dir_all(root.join("Mods/modAlpha")).unwrap();
  fs::create_dir_all(root.join("Mods/mod0000_MergedFiles")).unwrap();
  fs::write(root.join("Mods/readme.txt"), "ignored").unwrap();

  let resolved = discover_game_folder(&GameRoot::new(root.to_path_buf())).unwrap();

  assert_eq!(resolved.kind, ModEnvironmentKind::GameFolder);
  assert_eq!(
    resolved
      .mods
      .iter()
      .map(ResolvedMod::name)
      .collect::<Vec<_>>(),
    ["modAlpha", "modBeta"]
  );
  assert_eq!(resolved.output_mod.name, "mod0000_MergedFiles");
  assert_eq!(
    resolved.output_mod.path,
    root.join("Mods/mod0000_MergedFiles")
  );
}

#[test]
fn mod_organizer_discovers_selected_profile_and_active_mods() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  fs::create_dir_all(root.join("mods/Enabled A")).unwrap();
  fs::create_dir_all(root.join("mods/Enabled B")).unwrap();
  fs::create_dir_all(root.join("mods/Disabled")).unwrap();
  fs::create_dir_all(root.join("profiles/Default")).unwrap();
  fs::write(
    root.join("ModOrganizer.ini"),
    "[General]\nselected_profile=Default\n",
  )
  .unwrap();
  fs::write(
    root.join("profiles/Default/modlist.txt"),
    "-Disabled\n+Enabled A\n+Enabled B\n",
  )
  .unwrap();

  let config = ModOrganizerConfig::new(root.to_path_buf()).with_output_mod("Merge Output");
  let resolved = discover_mod_organizer(&config).unwrap();

  assert_eq!(resolved.kind, ModEnvironmentKind::ModOrganizer);
  assert_eq!(resolved.profile.as_deref(), Some("Default"));
  assert_eq!(
    resolved
      .mods
      .iter()
      .map(ResolvedMod::name)
      .collect::<Vec<_>>(),
    ["Enabled A", "Enabled B"]
  );
  assert_eq!(resolved.output_mod.path, root.join("mods/Merge Output"));
}

#[test]
fn mod_organizer_accepts_qt_byte_array_profile_value() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  fs::create_dir_all(root.join("mods/Enabled")).unwrap();
  fs::create_dir_all(root.join("profiles/Profile Name")).unwrap();
  fs::write(
    root.join("ModOrganizer.ini"),
    "[General]\nselected_profile=@ByteArray(Profile Name)\n",
  )
  .unwrap();
  fs::write(root.join("profiles/Profile Name/modlist.txt"), "+Enabled\n").unwrap();

  let resolved = discover_mod_organizer(&ModOrganizerConfig::new(root.to_path_buf())).unwrap();

  assert_eq!(resolved.profile.as_deref(), Some("Profile Name"));
}

#[test]
fn mod_organizer_ignores_qt_variant_values_when_reading_selected_profile() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  fs::create_dir_all(root.join("mods/Enabled")).unwrap();
  fs::create_dir_all(root.join("profiles/Default")).unwrap();
  fs::write(
    root.join("ModOrganizer.ini"),
    "[General]\nselected_profile=Default\noverwrittenLooseFilesColor=@Variant(\\0\\0\\0\\x43\\x1@@\
     \\0\\0\\xff\\xff\\0\\0\\0\\0)\n",
  )
  .unwrap();
  fs::write(root.join("profiles/Default/modlist.txt"), "+Enabled\n").unwrap();

  let resolved = discover_mod_organizer(&ModOrganizerConfig::new(root.to_path_buf())).unwrap();

  assert_eq!(resolved.profile.as_deref(), Some("Default"));
}

#[test]
fn mod_organizer_prefers_general_selected_profile() {
  let temp = TempDir::new().unwrap();
  let root = Utf8Path::from_path(temp.path()).unwrap();
  fs::create_dir_all(root.join("mods/Enabled")).unwrap();
  fs::create_dir_all(root.join("profiles/Default")).unwrap();
  fs::create_dir_all(root.join("profiles/Wrong")).unwrap();
  fs::write(
    root.join("ModOrganizer.ini"),
    "[General]\nselected_profile=Default\n[Other]\nselected_profile=Wrong\n",
  )
  .unwrap();
  fs::write(root.join("profiles/Default/modlist.txt"), "+Enabled\n").unwrap();
  fs::write(root.join("profiles/Wrong/modlist.txt"), "").unwrap();

  let resolved = discover_mod_organizer(&ModOrganizerConfig::new(root.to_path_buf())).unwrap();

  assert_eq!(resolved.profile.as_deref(), Some("Default"));
}
