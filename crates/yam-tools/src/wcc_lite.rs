use std::{
  ffi::OsString,
  path::{Path, PathBuf},
};

use crate::{
  command::{CommandRunner, CommandSpec, SystemRunner, ToolRun, require_exit_code},
  error::{ToolError, require_dir, require_file},
  launch::LaunchMode,
  report::{ToolComponent, ToolKind, ToolReport, component_report},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackBundleInput {
  pub source_dir: PathBuf,
  pub output_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct WccLite<R = SystemRunner> {
  executable: PathBuf,
  launch_mode: LaunchMode,
  runner: R,
}

impl WccLite<SystemRunner> {
  #[must_use]
  pub fn new(executable: impl Into<PathBuf>) -> Self {
    Self {
      executable: executable.into(),
      launch_mode: LaunchMode::Direct,
      runner: SystemRunner,
    }
  }
}

impl<R: CommandRunner> WccLite<R> {
  #[must_use]
  pub fn with_runner(executable: impl Into<PathBuf>, runner: R) -> Self {
    Self {
      executable: executable.into(),
      launch_mode: LaunchMode::Direct,
      runner,
    }
  }

  #[must_use]
  pub fn with_launch_mode(mut self, launch_mode: LaunchMode) -> Self {
    self.launch_mode = launch_mode;
    self
  }

  #[must_use]
  pub fn inspect(&self) -> ToolReport {
    ToolReport {
      kind: ToolKind::WccLite,
      components: vec![component_report(
        ToolComponent::Executable,
        &self.executable,
      )],
    }
  }

  pub fn pack_bundle(&self, input: &PackBundleInput) -> Result<ToolRun, ToolError> {
    require_file(
      ToolKind::WccLite,
      ToolComponent::Executable,
      &self.executable,
    )?;
    require_dir(
      ToolKind::WccLite,
      ToolComponent::Directory,
      &input.source_dir,
    )?;
    require_dir(
      ToolKind::WccLite,
      ToolComponent::Directory,
      &input.output_dir,
    )?;

    self.run_checked(vec![
      OsString::from("pack"),
      OsString::from(format!("-dir={}", input.source_dir.display())),
      OsString::from(format!("-outdir={}", input.output_dir.display())),
    ])
  }

  pub fn generate_metadata(&self, bundle_dir: impl AsRef<Path>) -> Result<ToolRun, ToolError> {
    let bundle_dir = bundle_dir.as_ref();
    require_file(
      ToolKind::WccLite,
      ToolComponent::Executable,
      &self.executable,
    )?;
    require_dir(ToolKind::WccLite, ToolComponent::Directory, bundle_dir)?;

    self.run_checked(vec![
      OsString::from("metadatastore"),
      OsString::from(format!("-path={}", bundle_dir.display())),
    ])
  }

  fn run_checked(&self, args: Vec<OsString>) -> Result<ToolRun, ToolError> {
    let command = self.launch_mode.apply(CommandSpec::new(
      ToolKind::WccLite,
      &self.executable,
      args,
      self.executable.parent().map(Path::to_path_buf),
    ));
    let output = self.runner.run(&command)?;

    let output = require_exit_code(output, ToolKind::WccLite, &[0])?;
    reject_reported_failure(&output)?;
    Ok(output)
  }
}

fn reject_reported_failure(output: &ToolRun) -> Result<(), ToolError> {
  let stderr = output.stderr.trim();
  if !stderr.is_empty() {
    return Err(ToolError::ToolReportedFailure {
      tool: ToolKind::WccLite,
      message: stderr.to_string(),
    });
  }

  let stdout = output.stdout.trim_end();
  if stdout.ends_with("Wcc operation failed") {
    return Err(ToolError::ToolReportedFailure {
      tool: ToolKind::WccLite,
      message: stdout.to_string(),
    });
  }

  Ok(())
}
