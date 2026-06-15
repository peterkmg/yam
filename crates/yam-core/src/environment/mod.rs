mod direct;
mod error;
mod mo;
mod mo_ini;
mod model;

use camino::Utf8Path;
pub use direct::discover_game_folder;
pub use error::EnvironmentError;
pub use mo::discover_mod_organizer;
pub use model::{
  DEFAULT_OUTPUT_MOD_NAME,
  ModEnvironmentKind,
  ModOrganizerConfig,
  OutputMod,
  ResolvedEnvironment,
  ResolvedMod,
};

fn ensure_dir(path: &Utf8Path) -> Result<(), EnvironmentError> {
  if path.is_dir() {
    Ok(())
  } else {
    Err(EnvironmentError::MissingPath(path.to_path_buf()))
  }
}
