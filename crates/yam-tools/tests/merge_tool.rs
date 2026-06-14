#![allow(unused_crate_dependencies)]

#[path = "support/files.rs"]
mod files;

use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

use yam_tools::{
  CommandRunner,
  CommandSpec,
  ManualMergeInput,
  ManualMergeTool,
  MergeToolProfile,
  ToolCommand,
  ToolError,
  ToolRun,
};

#[test]
fn kdiff3_profile_runs_three_way_merge_and_requires_output_file() {
  let files = files::TestDir::new();
  let exe = files.file("KDiff3/kdiff3.exe");
  let base = files.file("base.ws");
  let left = files.file("left.ws");
  let right = files.file("right.ws");
  let output = files.path("merged.ws");
  let runner = WritingRunner::new(output.clone());
  let tool = ManualMergeTool::with_runner(MergeToolProfile::kdiff3(&exe), runner.clone());

  tool
    .merge(&ManualMergeInput {
      base,
      left,
      right,
      output,
      base_label: Some("Vanilla".to_string()),
      left_label: Some("modA".to_string()),
      right_label: Some("modB".to_string()),
      output_label: Some("Merged".to_string()),
    })
    .expect("merge tool should run");

  let commands = runner.commands();
  assert_eq!(commands.len(), 1);
  assert_eq!(commands[0].program, exe);
  assert_eq!(commands[0].args[3], "-o");
  assert_eq!(commands[0].args[5], "--L1");
  assert_eq!(commands[0].args[6], "Vanilla");
  assert_eq!(commands[0].args[7], "--L2");
  assert_eq!(commands[0].args[8], "modA");
  assert_eq!(commands[0].args[9], "--L3");
  assert_eq!(commands[0].args[10], "modB");
}

#[test]
fn custom_merge_profile_must_use_explicit_output_placeholder() {
  let files = files::TestDir::new();
  let exe = files.file("merge.exe");
  let runner = WritingRunner::new(files.path("merged.ws"));
  let tool = ManualMergeTool::with_runner(
    MergeToolProfile::new(
      "broken",
      ToolCommand::new(&exe, vec!["{base}", "{left}", "{right}"]),
    ),
    runner,
  );

  let error = tool
    .merge(&ManualMergeInput {
      base: files.file("base.ws"),
      left: files.file("left.ws"),
      right: files.file("right.ws"),
      output: files.path("merged.ws"),
      base_label: None,
      left_label: None,
      right_label: None,
      output_label: None,
    })
    .expect_err("profile without output should be rejected");

  assert!(matches!(
    error,
    ToolError::MissingRequiredPlaceholder { .. }
  ));
}

#[test]
fn beyond_compare_profile_uses_explicit_merge_output_argument() {
  let files = files::TestDir::new();
  let exe = files.file("BComp.exe");
  let output = files.path("merged.ws");
  let runner = WritingRunner::new(output.clone());
  let tool = ManualMergeTool::with_runner(MergeToolProfile::beyond_compare(&exe), runner.clone());

  tool
    .merge(&ManualMergeInput {
      base: files.file("base.ws"),
      left: files.file("left.ws"),
      right: files.file("right.ws"),
      output: output.clone(),
      base_label: None,
      left_label: None,
      right_label: None,
      output_label: Some("Merged output".to_string()),
    })
    .expect("merge tool should run");

  let commands = runner.commands();
  assert_eq!(commands[0].program, exe);
  assert_eq!(
    commands[0].args[3].to_string_lossy(),
    format!("/mergeoutput={}", output.display())
  );
  assert_eq!(commands[0].args[10], "/outputtitle=Merged output");
}

#[test]
fn merge_tool_reports_missing_output_file_after_successful_exit() {
  let files = files::TestDir::new();
  let exe = files.file("merge.exe");
  let tool = ManualMergeTool::with_runner(MergeToolProfile::kdiff3(&exe), SucceedingRunner);

  let error = tool
    .merge(&ManualMergeInput {
      base: files.file("base.ws"),
      left: files.file("left.ws"),
      right: files.file("right.ws"),
      output: files.path("merged.ws"),
      base_label: None,
      left_label: None,
      right_label: None,
      output_label: None,
    })
    .expect_err("missing output should be reported");

  assert!(matches!(error, ToolError::MissingToolOutput { .. }));
}

#[derive(Debug, Clone)]
struct WritingRunner {
  inner: Rc<WritingRunnerState>,
}

#[derive(Debug)]
struct WritingRunnerState {
  output: PathBuf,
  commands: RefCell<Vec<CommandSpec>>,
}

impl WritingRunner {
  fn new(output: PathBuf) -> Self {
    Self {
      inner: Rc::new(WritingRunnerState {
        output,
        commands: RefCell::new(Vec::new()),
      }),
    }
  }

  fn commands(&self) -> Vec<CommandSpec> {
    self.inner.commands.borrow().clone()
  }
}

impl CommandRunner for WritingRunner {
  fn run(&self, command: &CommandSpec) -> Result<ToolRun, ToolError> {
    self.inner.commands.borrow_mut().push(command.clone());
    fs::write(&self.inner.output, "merged").unwrap();
    Ok(ToolRun {
      status_code: Some(0),
      stdout: String::new(),
      stderr: String::new(),
    })
  }
}

#[derive(Debug, Clone, Copy)]
struct SucceedingRunner;

impl CommandRunner for SucceedingRunner {
  fn run(&self, _command: &CommandSpec) -> Result<ToolRun, ToolError> {
    Ok(ToolRun {
      status_code: Some(0),
      stdout: String::new(),
      stderr: String::new(),
    })
  }
}
