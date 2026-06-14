#![allow(unused_crate_dependencies)]

#[path = "support/dirs.rs"]
mod dirs;
#[path = "support/files.rs"]
mod files;
#[path = "support/runner.rs"]
mod runner_support;

use yam_tools::{PackBundleInput, ToolError, ToolKind, WccLite};

#[test]
fn wcc_lite_packs_bundle_content() {
  let files = files::TestDir::new();
  let exe = files.file("bin/wcc_lite.exe");
  let source_dir = dirs::dir(&files, "merged-content");
  let output_dir = dirs::dir(&files, "output");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success("packed", "")]);
  let wcc_lite = WccLite::with_runner(&exe, runner.clone());

  let input = PackBundleInput {
    source_dir,
    output_dir,
  };

  let run = wcc_lite.pack_bundle(&input).expect("pack should run");

  assert_eq!(run.status_code, Some(0));

  let commands = runner.commands();
  assert_eq!(commands.len(), 1);
  assert_eq!(commands[0].program, exe);
  assert_eq!(commands[0].args[0], "pack");
  assert_eq!(
    commands[0].args[1].to_string_lossy(),
    format!("-dir={}", input.source_dir.display())
  );
  assert_eq!(
    commands[0].args[2].to_string_lossy(),
    format!("-outdir={}", input.output_dir.display())
  );
  assert_eq!(commands[0].working_dir, exe.parent().map(Into::into));
}

#[test]
fn wcc_lite_generates_metadata() {
  let files = files::TestDir::new();
  let exe = files.file("bin/wcc_lite.exe");
  let bundle_dir = dirs::dir(&files, "output");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success("metadata", "")]);
  let wcc_lite = WccLite::with_runner(&exe, runner.clone());

  wcc_lite
    .generate_metadata(&bundle_dir)
    .expect("metadata generation should run");

  let commands = runner.commands();
  assert_eq!(commands[0].args[0], "metadatastore");
  assert_eq!(
    commands[0].args[1].to_string_lossy(),
    format!("-path={}", bundle_dir.display())
  );
}

#[test]
fn wcc_lite_reports_stderr_as_tool_failure() {
  let files = files::TestDir::new();
  let exe = files.file("bin/wcc_lite.exe");
  let source_dir = dirs::dir(&files, "merged-content");
  let output_dir = dirs::dir(&files, "output");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success("", "packing failed")]);
  let wcc_lite = WccLite::with_runner(&exe, runner);

  let error = wcc_lite
    .pack_bundle(&PackBundleInput {
      source_dir,
      output_dir,
    })
    .expect_err("stderr should be treated as failure");

  assert!(matches!(error, ToolError::ToolReportedFailure {
    tool: ToolKind::WccLite,
    ..
  }));
}

#[test]
fn wcc_lite_reports_stdout_failure_suffix() {
  let files = files::TestDir::new();
  let exe = files.file("bin/wcc_lite.exe");
  let bundle_dir = dirs::dir(&files, "output");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success(
    "some output\nWcc operation failed",
    "",
  )]);
  let wcc_lite = WccLite::with_runner(&exe, runner);

  let error = wcc_lite
    .generate_metadata(&bundle_dir)
    .expect_err("failure suffix should be treated as failure");

  assert!(matches!(error, ToolError::ToolReportedFailure {
    tool: ToolKind::WccLite,
    ..
  }));
}
