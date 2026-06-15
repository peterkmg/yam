use yam_core::ScanReport;

pub fn candidate<'a>(report: &'a ScanReport, relative_path: &str) -> &'a yam_core::MergeCandidate {
  report
    .merge_candidates
    .iter()
    .find(|candidate| candidate.relative_path == relative_path)
    .unwrap()
}
