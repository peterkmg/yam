use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use yam_cache::CacheStore;
use yam_core::{
  BundleLister,
  GameRoot,
  LoadOrder,
  ModOrganizerConfig,
  ResolvedEnvironment,
  ScanOptions,
  ScanReport,
  discover_game_folder,
  discover_mod_organizer,
  scan_environment,
};
use yam_fs::{read_text_file, utf8_path};
use yam_tools::QuickBms;

pub(super) struct ScanRequest<'a> {
  pub(super) environment: &'a ResolvedEnvironment,
  pub(super) cache_root: &'a Path,
  pub(super) quickbms: Option<PathBuf>,
  pub(super) quickbms_script: Option<PathBuf>,
}

pub(super) fn resolve_environment(
  game_root: Option<PathBuf>,
  mo_root: Option<PathBuf>,
  profile: Option<String>,
  output_mod: Option<String>,
) -> Result<ResolvedEnvironment> {
  let resolved = match (game_root, mo_root) {
    (Some(game_root), None) => discover_game_folder(&GameRoot::new(utf8_path(game_root)?))?,
    (None, Some(mo_root)) => {
      let mut config = ModOrganizerConfig::new(utf8_path(mo_root)?);
      if let Some(profile) = profile {
        config = config.with_profile(profile);
      }
      if let Some(output_mod) = output_mod {
        config = config.with_output_mod(output_mod);
      }
      discover_mod_organizer(&config)?
    }
    (None, None) => return Err(anyhow!("provide --game-root or --mo-root")),
    (Some(_), Some(_)) => return Err(anyhow!("choose only one of --game-root or --mo-root")),
  };

  Ok(resolved)
}

pub(super) fn scan_with_cache(request: ScanRequest<'_>) -> Result<ScanReport> {
  let quickbms = quickbms_lister(request.quickbms, request.quickbms_script)?;
  let cache = CacheStore::open(request.cache_root)?;

  Ok(scan_environment(
    request.environment,
    &cache,
    ScanOptions {
      bundle_lister: quickbms.as_ref().map(|tool| tool as &dyn BundleLister),
      ..ScanOptions::default()
    },
  )?)
}

pub(super) fn load_order(path: Option<PathBuf>) -> Result<LoadOrder> {
  let Some(path) = path else {
    return Ok(LoadOrder::empty());
  };

  let text = read_text_file(&path)
    .with_context(|| format!("failed to read load order at {}", path.display()))?
    .text;

  LoadOrder::parse(&text)
    .with_context(|| format!("failed to parse load order at {}", path.display()))
}

fn quickbms_lister(
  executable: Option<PathBuf>,
  script: Option<PathBuf>,
) -> Result<Option<QuickBms>> {
  match (executable, script) {
    (Some(executable), Some(script)) => Ok(Some(QuickBms::new(executable, script))),
    (None, None) => Ok(None),
    (Some(_), None) => Err(anyhow!("provide --quickbms-script when using --quickbms")),
    (None, Some(_)) => Err(anyhow!("provide --quickbms when using --quickbms-script")),
  }
}
