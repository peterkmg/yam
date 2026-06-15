use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
  ToolCommand,
  ToolError,
  ToolKind,
  command::{CommandRunner, SystemRunner, ToolRun, require_exit_code},
  error::require_file,
  report::ToolComponent,
  template::TemplateValue,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorProfile {
  pub name: String,
  pub command: ToolCommand,
}

impl EditorProfile {
  #[must_use]
  pub fn new(name: impl Into<String>, command: ToolCommand) -> Self {
    Self {
      name: name.into(),
      command,
    }
  }

  #[must_use]
  pub fn vscode(program: impl Into<PathBuf>) -> Self {
    Self::new(
      "VS Code",
      ToolCommand::new(program, vec!["--goto", "{file}:{line}:{column}"]),
    )
  }

  fn validate(&self) -> Result<(), ToolError> {
    if self.command.contains_placeholder("file") {
      Ok(())
    } else {
      Err(ToolError::MissingRequiredPlaceholder {
        tool: ToolKind::ConflictEditor,
        placeholder: "file",
      })
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenConflictFileInput {
  pub file: PathBuf,
  pub line: Option<u32>,
  pub column: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ConflictEditor<R = SystemRunner> {
  profile: EditorProfile,
  runner: R,
}

impl ConflictEditor<SystemRunner> {
  #[must_use]
  pub const fn new(profile: EditorProfile) -> Self {
    Self {
      profile,
      runner: SystemRunner,
    }
  }
}

impl<R: CommandRunner> ConflictEditor<R> {
  #[must_use]
  pub const fn with_runner(profile: EditorProfile, runner: R) -> Self {
    Self { profile, runner }
  }

  pub fn open(&self, input: &OpenConflictFileInput) -> Result<ToolRun, ToolError> {
    self.profile.validate()?;
    require_file(ToolKind::ConflictEditor, ToolComponent::File, &input.file)?;

    let file = input.file.to_string_lossy();
    let line = input.line.unwrap_or(1).to_string();
    let column = input.column.unwrap_or(1).to_string();

    let values = [
      TemplateValue {
        key: "file",
        value: &file,
      },
      TemplateValue {
        key: "line",
        value: &line,
      },
      TemplateValue {
        key: "column",
        value: &column,
      },
    ];

    let command = self
      .profile
      .command
      .render(ToolKind::ConflictEditor, &values)?;

    let output = self.runner.run(&command)?;

    require_exit_code(
      output,
      ToolKind::ConflictEditor,
      &self.profile.command.success_exit_codes,
    )
  }
}
