use std::{
  ffi::OsString,
  path::{Path, PathBuf},
};

use yam_fs::LogicalPath;

use crate::{
  command::{CommandRunner, CommandSpec, SystemRunner, ToolRun, require_exit_code},
  error::{ToolError, require_file},
  launch::LaunchMode,
  report::{ToolComponent, ToolKind, ToolReport, component_report},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleEntry {
  pub path: String,
  pub offset: u64,
  pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractFileInput {
  pub bundle_path: PathBuf,
  pub entry_path: String,
  pub output_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct QuickBms<R = SystemRunner> {
  executable: PathBuf,
  script: PathBuf,
  launch_mode: LaunchMode,
  runner: R,
}

impl QuickBms<SystemRunner> {
  #[must_use]
  pub fn new(executable: impl Into<PathBuf>, script: impl Into<PathBuf>) -> Self {
    Self {
      executable: executable.into(),
      script: script.into(),
      launch_mode: LaunchMode::Direct,
      runner: SystemRunner,
    }
  }
}

impl<R: CommandRunner> QuickBms<R> {
  #[must_use]
  pub fn with_runner(
    executable: impl Into<PathBuf>,
    script: impl Into<PathBuf>,
    runner: R,
  ) -> Self {
    Self {
      executable: executable.into(),
      script: script.into(),
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
      kind: ToolKind::QuickBms,
      components: vec![
        component_report(ToolComponent::Executable, &self.executable),
        component_report(ToolComponent::Script, &self.script),
      ],
    }
  }

  pub fn list_bundle(&self, bundle_path: impl AsRef<Path>) -> Result<Vec<BundleEntry>, ToolError> {
    let bundle_path = bundle_path.as_ref();
    self.validate_bundle_inputs(bundle_path)?;

    let command = self.launch_mode.apply(CommandSpec::new(
      ToolKind::QuickBms,
      &self.executable,
      vec![
        OsString::from("-l"),
        self.script.as_os_str().to_os_string(),
        bundle_path.as_os_str().to_os_string(),
      ],
      None,
    ));

    let output = self.runner.run(&command)?;
    let output = require_exit_code(output, ToolKind::QuickBms, &[0])?;

    parse_bundle_list(&output)
  }

  pub fn extract_file(&self, input: &ExtractFileInput) -> Result<PathBuf, ToolError> {
    self.validate_bundle_inputs(&input.bundle_path)?;

    let command = self.launch_mode.apply(CommandSpec::new(
      ToolKind::QuickBms,
      &self.executable,
      vec![
        OsString::from("-Y"),
        OsString::from("-f"),
        OsString::from(&input.entry_path),
        self.script.as_os_str().to_os_string(),
        input.bundle_path.as_os_str().to_os_string(),
        input.output_dir.as_os_str().to_os_string(),
      ],
      None,
    ));

    let output = self.runner.run(&command)?;
    if output.stdout.contains("- 0 files found") || output.stderr.contains("- 0 files found") {
      return Err(ToolError::MissingBundleEntry {
        tool: ToolKind::QuickBms,
        entry_path: input.entry_path.clone(),
        output: format!("{}\n{}", output.stdout, output.stderr),
      });
    }

    require_exit_code(output, ToolKind::QuickBms, &[0])?;

    let path = extracted_path(&input.output_dir, &input.entry_path)?;
    if path.is_file() {
      Ok(path)
    } else {
      Err(ToolError::MissingExtractedFile { path })
    }
  }

  fn validate_bundle_inputs(&self, bundle_path: &Path) -> Result<(), ToolError> {
    require_file(
      ToolKind::QuickBms,
      ToolComponent::Executable,
      &self.executable,
    )?;

    require_file(ToolKind::QuickBms, ToolComponent::Script, &self.script)?;
    require_file(ToolKind::QuickBms, ToolComponent::Bundle, bundle_path)
  }
}

fn parse_bundle_list(output: &ToolRun) -> Result<Vec<BundleEntry>, ToolError> {
  let mut entries = Vec::new();

  parse_bundle_lines(&output.stdout, &mut entries)?;
  parse_bundle_lines(&output.stderr, &mut entries)?;

  Ok(entries)
}

fn parse_bundle_lines(output: &str, entries: &mut Vec<BundleEntry>) -> Result<(), ToolError> {
  for line in output.lines() {
    if let Some(entry) = parse_entry_line(line)? {
      entries.push(entry);
    }
  }

  Ok(())
}

fn parse_entry_line(line: &str) -> Result<Option<BundleEntry>, ToolError> {
  let mut parts = line.split_whitespace();
  let Some(offset) = parts.next() else {
    return Ok(None);
  };

  if !offset
    .chars()
    .all(|character| character.is_ascii_hexdigit())
  {
    return Ok(None);
  }

  let Some(size) = parts.next() else {
    return parse_error(line);
  };

  let Ok(offset) = u64::from_str_radix(offset, 16) else {
    return parse_error(line);
  };

  let Ok(size) = size.parse() else {
    return parse_error(line);
  };

  let path = parts.collect::<Vec<_>>().join(" ");
  if path.is_empty() {
    return parse_error(line);
  }

  let Ok(path) = LogicalPath::new(&path) else {
    return parse_error(line);
  };

  Ok(Some(BundleEntry {
    path: path.into_string(),
    offset,
    size,
  }))
}

fn parse_error<T>(line: &str) -> Result<T, ToolError> {
  Err(ToolError::Parse {
    tool: ToolKind::QuickBms,
    line: line.to_string(),
  })
}

fn extracted_path(output_dir: &Path, entry_path: &str) -> Result<PathBuf, ToolError> {
  let path = LogicalPath::new(entry_path).map_err(|source| ToolError::InvalidLogicalPath {
    tool: ToolKind::QuickBms,
    path: entry_path.to_string(),
    source,
  })?;

  Ok(path.to_disk_path(output_dir))
}
