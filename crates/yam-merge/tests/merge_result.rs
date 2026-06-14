#![allow(unused_crate_dependencies)]

use yam_merge::MergeResult;

#[test]
fn clean_result_reports_no_conflicts() {
  let result = MergeResult::new("merged".to_owned(), false);

  assert!(result.is_clean());
  assert!(!result.has_conflicts);
  assert_eq!(result.conflict_count(), 0);
}

#[test]
fn conflicted_result_reports_conflicts() {
  let result = MergeResult::new("merged".to_owned(), true);

  assert!(!result.is_clean());
  assert!(result.has_conflicts);
}

#[test]
fn result_counts_conflict_markers_as_conflicts() {
  let result = MergeResult::new(
    "<<<<<<< ours\nours\n=======\ntheirs\n>>>>>>> theirs".to_owned(),
    false,
  );

  assert!(!result.is_clean());
  assert!(result.has_conflicts);
  assert_eq!(result.conflict_count(), 1);
}
