use std::{ffi::OsString, path::PathBuf, process::Command};

use crate::{ToolError, ToolKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
  pub tool: ToolKind,
  pub program: PathBuf,
  pub args: Vec<OsString>,
  pub working_dir: Option<PathBuf>,
  pub env: Vec<(OsString, OsString)>,
}

impl CommandSpec {
  #[must_use]
  pub fn new(
    tool: ToolKind,
    program: impl Into<PathBuf>,
    args: Vec<OsString>,
    working_dir: Option<PathBuf>,
  ) -> Self {
    Self {
      tool,
      program: program.into(),
      args,
      working_dir,
      env: Vec::new(),
    }
  }

  #[must_use]
  pub fn with_env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
    self.env.push((key.into(), value.into()));

    self
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRun {
  pub status_code: Option<i32>,
  pub stdout: String,
  pub stderr: String,
}

pub trait CommandRunner {
  fn run(&self, command: &CommandSpec) -> Result<ToolRun, ToolError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemRunner;

impl CommandRunner for SystemRunner {
  fn run(&self, command: &CommandSpec) -> Result<ToolRun, ToolError> {
    let mut process = Command::new(&command.program);
    process.args(&command.args);

    if let Some(working_dir) = &command.working_dir {
      process.current_dir(working_dir);
    }

    for (key, value) in &command.env {
      process.env(key, value);
    }

    let output = process.output().map_err(|source| ToolError::Run {
      tool: command.tool,
      source,
    })?;

    Ok(ToolRun {
      status_code: output.status.code(),
      stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
      stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
  }
}

pub fn require_exit_code(
  output: ToolRun,
  tool: ToolKind,
  success_exit_codes: &[i32],
) -> Result<ToolRun, ToolError> {
  if output
    .status_code
    .is_some_and(|code| success_exit_codes.contains(&code))
  {
    Ok(output)
  } else {
    Err(ToolError::CommandFailed {
      tool,
      status_code: output.status_code,
      stdout: output.stdout,
      stderr: output.stderr,
    })
  }
}
