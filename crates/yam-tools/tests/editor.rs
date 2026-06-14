#![allow(unused_crate_dependencies)]

#[path = "support/files.rs"]
mod files;
#[path = "support/runner.rs"]
mod runner_support;

use yam_tools::{ConflictEditor, EditorProfile, OpenConflictFileInput, ToolCommand, ToolError};

#[test]
fn vscode_profile_opens_conflict_file_at_line_and_column() {
  let files = files::TestDir::new();
  let code = files.file("bin/code");
  let conflict = files.file("merged.ws");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success("", "")]);
  let editor = ConflictEditor::with_runner(EditorProfile::vscode(&code), runner.clone());

  editor
    .open(&OpenConflictFileInput {
      file: conflict.clone(),
      line: Some(12),
      column: Some(3),
    })
    .expect("editor should run");

  let commands = runner.commands();

  assert_eq!(commands.len(), 1);
  assert_eq!(commands[0].program, code);
  assert_eq!(commands[0].args[0], "--goto");
  assert_eq!(
    commands[0].args[1].to_string_lossy(),
    format!("{}:12:3", conflict.display())
  );
}

#[test]
fn custom_editor_profile_must_use_file_placeholder() {
  let files = files::TestDir::new();
  let editor_exe = files.file("editor.exe");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success("", "")]);
  let editor = ConflictEditor::with_runner(
    EditorProfile::new("broken", ToolCommand::new(editor_exe, vec!["--wait"])),
    runner,
  );

  let error = editor
    .open(&OpenConflictFileInput {
      file: files.file("merged.ws"),
      line: None,
      column: None,
    })
    .expect_err("profile without file placeholder should be rejected");

  assert!(matches!(
    error,
    ToolError::MissingRequiredPlaceholder { .. }
  ));
}
