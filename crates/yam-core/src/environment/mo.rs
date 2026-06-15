use camino::Utf8Path;
use yam_fs::read_text_file;

use super::{
  EnvironmentError,
  ensure_dir,
  mo_ini,
  model::{ModEnvironmentKind, ModOrganizerConfig, OutputMod, ResolvedEnvironment, ResolvedMod},
};

pub fn discover_mod_organizer(
  config: &ModOrganizerConfig,
) -> Result<ResolvedEnvironment, EnvironmentError> {
  let instance_root = config.instance_root();
  ensure_dir(instance_root)?;

  let mods_dir = instance_root.join("mods");
  ensure_dir(&mods_dir)?;

  let profile = match config.profile() {
    Some(profile) => profile.to_string(),
    None => mo_ini::read_selected_profile(&instance_root.join("ModOrganizer.ini"))?,
  };

  let output_mod = OutputMod {
    name: config.output_mod().to_string(),
    path: mods_dir.join(config.output_mod()),
  };
  let modlist_path = instance_root
    .join("profiles")
    .join(&profile)
    .join("modlist.txt");
  let mods = read_modlist(&modlist_path, &mods_dir)?;

  Ok(ResolvedEnvironment {
    kind: ModEnvironmentKind::ModOrganizer,
    root: instance_root.to_path_buf(),
    mods_dir,
    output_mod,
    profile: Some(profile),
    mods,
  })
}

fn read_modlist(
  path: &Utf8Path,
  mods_dir: &Utf8Path,
) -> Result<Vec<ResolvedMod>, EnvironmentError> {
  let text = read_text_file(path)?.text;
  let mut mods = Vec::new();
  for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
    let Some(name) = line.strip_prefix('+') else {
      continue;
    };
    let name = name.trim();
    mods.push(ResolvedMod::new(name, mods_dir.join(name)));
  }
  Ok(mods)
}
