#![allow(unused_crate_dependencies)]

#[path = "support/files.rs"]
mod files;

use yam_tools::{QuickBms, ToolComponent, ToolKind, WccLite};

#[test]
fn quickbms_report_requires_executable_and_script() {
  let files = files::TestDir::new();
  let exe = files.file("quickbms.exe");
  let script = files.path("missing/witcher3.bms");

  let report = QuickBms::new(&exe, &script).inspect();

  assert_eq!(report.kind, ToolKind::QuickBms);
  assert!(!report.is_available());
  assert_eq!(report.components.len(), 2);
  assert_eq!(report.components[0].component, ToolComponent::Executable);
  assert!(report.components[0].available);
  assert_eq!(report.components[1].component, ToolComponent::Script);
  assert!(!report.components[1].available);
}

#[test]
fn wcc_lite_report_tracks_executable() {
  let files = files::TestDir::new();
  let exe = files.file("wcc_lite.exe");

  let report = WccLite::new(&exe).inspect();

  assert_eq!(report.kind, ToolKind::WccLite);
  assert!(report.is_available());
  assert_eq!(report.components.len(), 1);
  assert_eq!(report.components[0].component, ToolComponent::Executable);
}

#[test]
fn tool_display_names_are_user_facing() {
  assert_eq!(ToolKind::QuickBms.display_name(), "QuickBMS");
  assert_eq!(ToolKind::WccLite.display_name(), "wcc_lite");
}
