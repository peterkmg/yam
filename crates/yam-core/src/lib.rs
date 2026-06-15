#![cfg_attr(test, allow(unused_crate_dependencies))]

pub mod config;
pub mod conflict;
pub mod environment;
pub mod load_order;
pub mod paths;
pub mod scan;

pub use config::{
  AppConfig,
  ConfigError,
  EnvironmentConfig,
  QuickBmsConfig,
  ToolConfig,
  load_config,
  save_config,
};
pub use conflict::{
  ClassifiedConflict,
  ClassifiedConflictAction,
  ClassifiedConflictSource,
  classify_conflicts,
};
pub use environment::{
  EnvironmentError,
  ModEnvironmentKind,
  ModOrganizerConfig,
  ResolvedEnvironment,
  ResolvedMod,
  discover_game_folder,
  discover_mod_organizer,
};
pub use load_order::{
  BOTTOM_PRIORITY,
  LoadOrder,
  LoadOrderEntry,
  LoadOrderError,
  SourceLoadOrder,
  TOP_PRIORITY,
  compare_game_mod_names,
};
pub use paths::GameRoot;
pub use scan::{
  BundleLister,
  ListedBundleEntry,
  MergeCandidate,
  MergeSource,
  MergeSourceLocation,
  ScanError,
  ScanOptions,
  ScanReport,
  ScannedBundle,
  ScannedBundleEntry,
  ScannedFile,
  scan_environment,
};
pub use yam_merge::MergeableFileType;
