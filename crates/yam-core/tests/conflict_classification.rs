#![allow(unused_crate_dependencies)]

use camino::Utf8PathBuf;
use yam_cache::ContentHash;
use yam_core::{
  ClassifiedConflictAction,
  LoadOrder,
  MergeCandidate,
  MergeSource,
  MergeSourceLocation,
  ScanReport,
  classify_conflicts,
};
use yam_merge::MergeableFileType;

#[test]
fn mergeable_conflict_keeps_ordered_active_sources() {
  let report = report_with(candidate(
    "content/scripts/game/player.ws",
    Some(MergeableFileType::WitcherScript),
    ["modGamma", "modAlpha"],
  ));

  let conflicts = classify_conflicts(&report, &LoadOrder::empty());

  assert_eq!(conflicts[0].action, ClassifiedConflictAction::MergeRequired);
  assert_eq!(source_names(&conflicts[0]), ["modAlpha", "modGamma"]);
  assert!(conflicts[0].sources.iter().all(|source| !source.is_winner));
}

#[test]
fn unmergeable_conflict_uses_load_order_winner() {
  let report = report_with(candidate("textures/preview.dds", None, [
    "modBeta", "modAlpha",
  ]));
  let order = LoadOrder::parse(
    "[modBeta]
Enabled=1
Priority=4

[modAlpha]
Enabled=1
Priority=1
",
  )
  .unwrap();

  let conflicts = classify_conflicts(&report, &order);

  assert_eq!(
    conflicts[0].action,
    ClassifiedConflictAction::LoadOrderResolved
  );
  assert_eq!(source_names(&conflicts[0]), ["modAlpha", "modBeta"]);
  assert!(conflicts[0].sources[0].is_winner);
  assert!(!conflicts[0].sources[1].is_winner);
}

#[test]
fn disabled_sources_do_not_force_merge_work() {
  let report = report_with(candidate(
    "content/scripts/game/player.ws",
    Some(MergeableFileType::WitcherScript),
    ["modAlpha", "modBeta"],
  ));
  let order = LoadOrder::parse(
    "[modAlpha]
Enabled=0
Priority=1

[modBeta]
Enabled=1
Priority=2
",
  )
  .unwrap();

  let conflicts = classify_conflicts(&report, &order);

  assert_eq!(
    conflicts[0].action,
    ClassifiedConflictAction::LoadOrderResolved
  );
  assert_eq!(source_names(&conflicts[0]), ["modBeta", "modAlpha"]);
  assert!(conflicts[0].sources[0].is_winner);
  assert!(!conflicts[0].sources[1].load_order.enabled);
}

fn report_with(candidate: MergeCandidate) -> ScanReport {
  ScanReport {
    files: Vec::new(),
    bundles: Vec::new(),
    bundle_entries: Vec::new(),
    merge_candidates: vec![candidate],
  }
}

fn candidate(
  relative_path: &str,
  merge_file_type: Option<MergeableFileType>,
  names: impl IntoIterator<Item = &'static str>,
) -> MergeCandidate {
  MergeCandidate {
    relative_path: relative_path.to_string(),
    merge_file_type,
    sources: names.into_iter().map(source).collect(),
  }
}

fn source(mod_name: &str) -> MergeSource {
  MergeSource {
    mod_name: mod_name.to_string(),
    location: MergeSourceLocation::LooseFile {
      path: Utf8PathBuf::from(format!("Mods/{mod_name}/input")),
    },
    hash: ContentHash::digest(mod_name.as_bytes()),
    len: 1,
    changed: true,
  }
}

fn source_names(conflict: &yam_core::ClassifiedConflict) -> Vec<&str> {
  conflict
    .sources
    .iter()
    .map(|source| source.source.mod_name.as_str())
    .collect()
}
