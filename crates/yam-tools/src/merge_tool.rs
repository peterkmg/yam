use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::{
  ToolCommand,
  ToolError,
  ToolKind,
  command::{CommandRunner, SystemRunner, ToolRun, require_exit_code},
  template::TemplateValue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeToolProfile {
  pub name: String,
  pub command: ToolCommand,
}

impl MergeToolProfile {
  #[must_use]
  pub fn new(name: impl Into<String>, command: ToolCommand) -> Self {
    Self {
      name: name.into(),
      command,
    }
  }

  #[must_use]
  pub fn kdiff3(program: impl Into<PathBuf>) -> Self {
    Self::new(
      "KDiff3",
      ToolCommand::new(program, vec![
        "{base}",
        "{left}",
        "{right}",
        "-o",
        "{output}",
        "--L1",
        "{base_label}",
        "--L2",
        "{left_label}",
        "--L3",
        "{right_label}",
      ]),
    )
  }

  #[must_use]
  pub fn beyond_compare(program: impl Into<PathBuf>) -> Self {
    Self::new(
      "Beyond Compare",
      ToolCommand::new(program, vec![
        "{left}",
        "{right}",
        "{base}",
        "/mergeoutput={output}",
        "/automerge",
        "/reviewconflicts",
        "/solo",
        "/lefttitle={left_label}",
        "/centertitle={base_label}",
        "/righttitle={right_label}",
        "/outputtitle={output_label}",
      ]),
    )
  }

  fn validate(&self) -> Result<(), ToolError> {
    for placeholder in ["base", "left", "right", "output"] {
      if !self.command.contains_placeholder(placeholder) {
        return Err(ToolError::MissingRequiredPlaceholder {
          tool: ToolKind::ManualMerge,
          placeholder,
        });
      }
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualMergeInput {
  pub base: PathBuf,
  pub left: PathBuf,
  pub right: PathBuf,
  pub output: PathBuf,
  pub base_label: Option<String>,
  pub left_label: Option<String>,
  pub right_label: Option<String>,
  pub output_label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ManualMergeTool<R = SystemRunner> {
  profile: MergeToolProfile,
  runner: R,
}

impl ManualMergeTool<SystemRunner> {
  #[must_use]
  pub const fn new(profile: MergeToolProfile) -> Self {
    Self {
      profile,
      runner: SystemRunner,
    }
  }
}

impl<R: CommandRunner> ManualMergeTool<R> {
  #[must_use]
  pub const fn with_runner(profile: MergeToolProfile, runner: R) -> Self {
    Self { profile, runner }
  }

  pub fn merge(&self, input: &ManualMergeInput) -> Result<ToolRun, ToolError> {
    self.profile.validate()?;
    remove_existing_output(&input.output)?;

    let base = input.base.to_string_lossy();
    let left = input.left.to_string_lossy();
    let right = input.right.to_string_lossy();
    let output = input.output.to_string_lossy();
    let output_label = input.output_label.as_deref().unwrap_or("Merged");

    let values = [
      TemplateValue {
        key: "base",
        value: &base,
      },
      TemplateValue {
        key: "left",
        value: &left,
      },
      TemplateValue {
        key: "right",
        value: &right,
      },
      TemplateValue {
        key: "output",
        value: &output,
      },
      TemplateValue {
        key: "base_label",
        value: input.base_label.as_deref().unwrap_or("Vanilla"),
      },
      TemplateValue {
        key: "left_label",
        value: input.left_label.as_deref().unwrap_or("Left"),
      },
      TemplateValue {
        key: "right_label",
        value: input.right_label.as_deref().unwrap_or("Right"),
      },
      TemplateValue {
        key: "output_label",
        value: output_label,
      },
    ];

    let command = self
      .profile
      .command
      .render(ToolKind::ManualMerge, &values)?;

    let output = self.runner.run(&command)?;
    let output = require_exit_code(
      output,
      ToolKind::ManualMerge,
      &self.profile.command.success_exit_codes,
    )?;

    if input.output.is_file() {
      Ok(output)
    } else {
      Err(ToolError::MissingToolOutput {
        tool: ToolKind::ManualMerge,
        path: input.output.clone(),
      })
    }
  }
}

fn remove_existing_output(path: &Path) -> Result<(), ToolError> {
  if path.is_file() {
    fs::remove_file(path).map_err(|source| ToolError::Io { source })?;
  }

  Ok(())
}
