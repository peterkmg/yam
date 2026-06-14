#![allow(unused_crate_dependencies)]

mod support;

use yam_cache::{ArtifactKind, LogicalPath, ObservationStatus, SourceId};

#[test]
fn observe_file_tracks_new_unchanged_and_changed_artifacts() {
  let case = support::CacheCase::in_memory();
  let file = case.root().join("input.ws");
  support::write_file(&file, "content");

  let first = case
    .store()
    .observe_file(&support::artifact_input(&file, "modAlpha"))
    .unwrap();

  assert_eq!(first.status, ObservationStatus::New);
  assert_eq!(
    first.artifact.input.key.source_id,
    SourceId::new("modAlpha")
  );
  assert_eq!(first.artifact.input.key.kind, ArtifactKind::LooseFile);
  assert_eq!(
    first.artifact.input.key.logical_path,
    LogicalPath::new("content/scripts/input.ws")
  );
  assert_eq!(first.artifact.byte_len, 7);

  let second = case
    .store()
    .observe_file(&support::artifact_input(&file, "modAlpha"))
    .unwrap();
  assert_eq!(second.status, ObservationStatus::Unchanged);
  assert_eq!(first.artifact.hash, second.artifact.hash);

  support::write_file(&file, "changed");
  let third = case
    .store()
    .observe_file(&support::artifact_input(&file, "modAlpha"))
    .unwrap();

  assert_eq!(third.status, ObservationStatus::Changed {
    previous_hash: second.artifact.hash,
  });
  assert_ne!(second.artifact.hash, third.artifact.hash);
}

#[test]
fn observe_file_keeps_sources_independent() {
  let case = support::CacheCase::in_memory();
  let file = case.root().join("input.ws");
  support::write_file(&file, "content");

  let alpha = case
    .store()
    .observe_file(&support::artifact_input(&file, "modAlpha"))
    .unwrap();
  let beta = case
    .store()
    .observe_file(&support::artifact_input(&file, "modBeta"))
    .unwrap();

  assert_eq!(alpha.status, ObservationStatus::New);
  assert_eq!(beta.status, ObservationStatus::New);
  assert_eq!(
    alpha.artifact.input.key.logical_path,
    beta.artifact.input.key.logical_path
  );
  assert_ne!(
    alpha.artifact.input.key.source_id,
    beta.artifact.input.key.source_id
  );
}

#[test]
fn observe_many_records_files_in_one_transaction() {
  let mut case = support::CacheCase::in_memory();
  let alpha = case.root().join("alpha.ws");
  let beta = case.root().join("beta.ws");
  support::write_file(&alpha, "alpha");
  support::write_file(&beta, "beta");

  let inputs = [
    support::artifact_input_with_path(&alpha, "modAlpha", "content/alpha.ws"),
    support::artifact_input_with_path(&beta, "modBeta", "content/beta.ws"),
  ];
  let first = case.store_mut().observe_many(&inputs).unwrap();
  let second = case.store_mut().observe_many(&inputs).unwrap();

  assert_eq!(first.len(), 2);
  assert!(first.iter().all(|result| result.status.is_changed()));
  assert!(second.iter().all(|result| !result.status.is_changed()));
}
