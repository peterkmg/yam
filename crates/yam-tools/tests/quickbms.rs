#![allow(unused_crate_dependencies)]

#[path = "support/args.rs"]
mod args;
#[path = "support/dirs.rs"]
mod dirs;
#[path = "support/files.rs"]
mod files;
#[path = "support/runner.rs"]
mod runner_support;

use yam_tools::{ExtractFileInput, QuickBms, ToolError, ToolKind};

#[test]
fn quickbms_lists_bundle_entries_from_mixed_output() {
  let files = files::TestDir::new();
  let exe = files.file("quickbms.exe");
  let script = files.file("witcher3.bms");
  let bundle = files.file("blob0.bundle");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success(
    "  00001000 1159       environment\\definitions\\test.env\n",
    "- 1 files found in 0 seconds\n",
  )]);
  let quickbms = QuickBms::with_runner(&exe, &script, runner.clone());

  let entries = quickbms.list_bundle(&bundle).expect("list should run");

  assert_eq!(entries.len(), 1);
  assert_eq!(entries[0].path.as_str(), "environment/definitions/test.env");
  assert_eq!(entries[0].offset, 0x1000);
  assert_eq!(entries[0].size, 1159);

  let commands = runner.commands();
  assert_eq!(commands.len(), 1);
  assert_eq!(commands[0].program, exe);
  assert_eq!(commands[0].args[0], "-l");
  args::assert_arg(&commands[0], 1, &script);
  args::assert_arg(&commands[0], 2, &bundle);
}

#[test]
fn quickbms_extracts_single_entry_to_output_path() {
  let files = files::TestDir::new();
  let exe = files.file("quickbms.exe");
  let script = files.file("witcher3.bms");
  let bundle = files.file("blob0.bundle");
  let output_dir = dirs::dir(&files, "out");
  let extracted = files.file("out/gameplay/items/test.xml");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success("", "extracted ok")]);
  let quickbms = QuickBms::with_runner(&exe, &script, runner.clone());

  let input = ExtractFileInput {
    bundle_path: bundle,
    entry_path: "gameplay/items/test.xml".to_string(),
    output_dir,
  };

  let extracted_path = quickbms.extract_file(&input).expect("extract should run");

  assert_eq!(extracted_path, extracted);

  let commands = runner.commands();
  assert_eq!(commands[0].args[0], "-Y");
  assert_eq!(commands[0].args[1], "-f");
  assert_eq!(commands[0].args[2], "gameplay/items/test.xml");
  args::assert_arg(&commands[0], 3, &script);
  args::assert_arg(&commands[0], 4, &input.bundle_path);
  args::assert_arg(&commands[0], 5, &input.output_dir);
}

#[test]
fn quickbms_extract_reports_missing_entry() {
  let files = files::TestDir::new();
  let exe = files.file("quickbms.exe");
  let script = files.file("witcher3.bms");
  let bundle = files.file("blob0.bundle");
  let output_dir = dirs::dir(&files, "out");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success(
    "",
    "- filter string: gameplay/items/missing.xml\n- 0 files found\n",
  )]);
  let quickbms = QuickBms::with_runner(&exe, &script, runner);

  let error = quickbms
    .extract_file(&ExtractFileInput {
      bundle_path: bundle,
      entry_path: "gameplay/items/missing.xml".to_string(),
      output_dir,
    })
    .expect_err("missing entry should be reported");

  assert!(matches!(error, ToolError::MissingBundleEntry {
    tool: ToolKind::QuickBms,
    ..
  }));
}

#[test]
fn quickbms_list_reports_malformed_entry_line() {
  let files = files::TestDir::new();
  let exe = files.file("quickbms.exe");
  let script = files.file("witcher3.bms");
  let bundle = files.file("blob0.bundle");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success(
    "  00001000 not-a-size gameplay\\items\\bad.xml\n",
    "",
  )]);
  let quickbms = QuickBms::with_runner(&exe, &script, runner);

  let error = quickbms
    .list_bundle(&bundle)
    .expect_err("malformed entry line should be reported");

  assert!(matches!(error, ToolError::Parse {
    tool: ToolKind::QuickBms,
    ..
  }));
}
