use yam_core::ScanReport;

pub fn print_scan_summary(report: &ScanReport) {
  println!("files: {}", report.files.len());
  println!("changed files: {}", report.changed_file_count());
  println!("unchanged files: {}", report.unchanged_file_count());
  println!("bundles: {}", report.bundles.len());
  println!("changed bundles: {}", report.changed_bundle_count());
  println!("unchanged bundles: {}", report.unchanged_bundle_count());
  println!("bundle entries: {}", report.bundle_entries.len());
  println!("merge candidates: {}", report.merge_candidates.len());
}
