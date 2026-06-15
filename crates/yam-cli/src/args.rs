use std::path::PathBuf;

use argh::FromArgs;

/// YAM, Yet Another Merger for Witcher 3 scripts and bundled text.
#[derive(Debug, FromArgs)]
pub struct Cli {
  #[argh(subcommand)]
  pub command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
  InspectEnv(InspectEnvCommand),
  Scan(ScanCommand),
  Conflicts(ConflictsCommand),
}

/// inspect configured game or Mod Organizer environment
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "inspect-env")]
pub struct InspectEnvCommand {
  /// witcher 3 game root for direct or Vortex-style installs
  #[argh(option)]
  pub game_root: Option<PathBuf>,

  /// mod Organizer instance root
  #[argh(option)]
  pub mo_root: Option<PathBuf>,

  /// mod Organizer profile name
  #[argh(option)]
  pub profile: Option<String>,

  /// output mod name
  #[argh(option)]
  pub output_mod: Option<String>,
}

/// scan mods and update the persistent file cache
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "scan")]
pub struct ScanCommand {
  /// witcher 3 game root for direct or Vortex-style installs
  #[argh(option)]
  pub game_root: Option<PathBuf>,

  /// mod Organizer instance root
  #[argh(option)]
  pub mo_root: Option<PathBuf>,

  /// mod Organizer profile name
  #[argh(option)]
  pub profile: Option<String>,

  /// output mod name
  #[argh(option)]
  pub output_mod: Option<String>,

  /// directory where scan cache data is stored
  #[argh(option)]
  pub cache_root: PathBuf,

  /// path to quickbms executable for listing bundle contents
  #[argh(option)]
  pub quickbms: Option<PathBuf>,

  /// path to Witcher 3 quickbms script
  #[argh(option)]
  pub quickbms_script: Option<PathBuf>,
}

/// scan mods and print load-order-aware conflicts
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "conflicts")]
pub struct ConflictsCommand {
  /// witcher 3 game root for direct or Vortex-style installs
  #[argh(option)]
  pub game_root: Option<PathBuf>,

  /// mod Organizer instance root
  #[argh(option)]
  pub mo_root: Option<PathBuf>,

  /// mod Organizer profile name
  #[argh(option)]
  pub profile: Option<String>,

  /// output mod name
  #[argh(option)]
  pub output_mod: Option<String>,

  /// directory where scan cache data is stored
  #[argh(option)]
  pub cache_root: PathBuf,

  /// path to quickbms executable for listing bundle contents
  #[argh(option)]
  pub quickbms: Option<PathBuf>,

  /// path to Witcher 3 quickbms script
  #[argh(option)]
  pub quickbms_script: Option<PathBuf>,

  /// path to mods.settings; omitted means default game mod-name order
  #[argh(option)]
  pub load_order: Option<PathBuf>,
}
