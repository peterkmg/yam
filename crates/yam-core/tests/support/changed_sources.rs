use yam_core::ScanReport;

pub fn changed_source_names(report: &ScanReport) -> Vec<&str> {
  report.merge_candidates[0]
    .sources
    .iter()
    .filter(|source| source.changed)
    .map(|source| source.mod_name.as_str())
    .collect()
}
