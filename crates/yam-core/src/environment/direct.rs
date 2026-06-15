use std::fs;

use camino::Utf8Path;
use yam_fs::utf8_path;

use super::{
  EnvironmentError,
  ensure_dir,
  model::{
    DEFAULT_OUTPUT_MOD_NAME,
    ModEnvironmentKind,
    OutputMod,
    ResolvedEnvironment,
    ResolvedMod,
  },
};
use crate::GameRoot;

pub fn discover_game_folder(root: &GameRoot) -> Result<ResolvedEnvironment, EnvironmentError> {
  let mods_dir = root.mods_dir();
  ensure_dir(&mods_dir)?;

  let output_mod = OutputMod {
    name: DEFAULT_OUTPUT_MOD_NAME.to_string(),
    path: mods_dir.join(DEFAULT_OUTPUT_MOD_NAME),
  };
  let mods = read_mod_dirs(&mods_dir, Some(&output_mod.name))?;

  Ok(ResolvedEnvironment {
    kind: ModEnvironmentKind::GameFolder,
    root: root.path().clone(),
    mods_dir,
    output_mod,
    profile: None,
    mods,
  })
}

fn read_mod_dirs(
  mods_dir: &Utf8Path,
  exclude_name: Option<&str>,
) -> Result<Vec<ResolvedMod>, EnvironmentError> {
  let mut mods = Vec::new();
  for entry in read_dir(mods_dir)? {
    let entry = entry.map_err(|source| EnvironmentError::Read {
      path: mods_dir.to_path_buf(),
      source,
    })?;
    let path = utf8_path(entry.path())?;
    if !path.is_dir() {
      continue;
    }
    let Some(name) = path.file_name() else {
      continue;
    };
    if exclude_name.is_some_and(|excluded| excluded.eq_ignore_ascii_case(name)) {
      continue;
    }
    if name.to_ascii_lowercase().starts_with("mod") {
      mods.push(ResolvedMod::new(name.to_string(), path));
    }
  }
  mods.sort_by_key(|item| item.name().to_ascii_lowercase());
  Ok(mods)
}

fn read_dir(path: &Utf8Path) -> Result<fs::ReadDir, EnvironmentError> {
  fs::read_dir(path).map_err(|source| EnvironmentError::Read {
    path: path.to_path_buf(),
    source,
  })
}
