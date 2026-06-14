use std::{
  ffi::OsString,
  path::{Path, PathBuf},
};

use crate::{
  ToolError,
  ToolKind,
  command::CommandSpec,
  template::{TemplateValue, expand_arguments},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCommand {
  pub program: PathBuf,
  pub args: Vec<String>,
  pub launch_mode: LaunchMode,
  pub working_dir: Option<PathBuf>,
  pub success_exit_codes: Vec<i32>,
}

impl ToolCommand {
  #[must_use]
  pub fn new(program: impl Into<PathBuf>, args: Vec<impl Into<String>>) -> Self {
    Self {
      program: program.into(),
      args: args.into_iter().map(Into::into).collect(),
      launch_mode: LaunchMode::Direct,
      working_dir: None,
      success_exit_codes: vec![0],
    }
  }

  #[must_use]
  pub fn with_launch_mode(mut self, launch_mode: LaunchMode) -> Self {
    self.launch_mode = launch_mode;

    self
  }

  #[must_use]
  pub fn with_working_dir(mut self, working_dir: impl Into<PathBuf>) -> Self {
    self.working_dir = Some(working_dir.into());

    self
  }

  #[must_use]
  pub fn with_success_exit_codes(mut self, success_exit_codes: Vec<i32>) -> Self {
    self.success_exit_codes = success_exit_codes;

    self
  }

  pub(crate) fn render(
    &self,
    tool: ToolKind,
    values: &[TemplateValue<'_>],
  ) -> Result<CommandSpec, ToolError> {
    let args = expand_arguments(tool, &self.args, values)?
      .into_iter()
      .map(OsString::from)
      .collect();

    let command = CommandSpec::new(tool, &self.program, args, self.working_dir.clone());

    Ok(self.launch_mode.apply(command))
  }

  pub(crate) fn contains_placeholder(&self, placeholder: &str) -> bool {
    let needle = format!("{{{placeholder}}}");
    self.args.iter().any(|arg| arg.contains(&needle))
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LaunchMode {
  #[default]
  Direct,
  Wine(WineLaunch),
}

impl LaunchMode {
  #[must_use]
  pub const fn direct() -> Self {
    Self::Direct
  }

  #[must_use]
  pub fn wine(executable: impl Into<PathBuf>) -> Self {
    Self::Wine(WineLaunch {
      executable: executable.into(),
      prefix: None,
      path_mappings: Vec::new(),
    })
  }

  #[must_use]
  pub fn with_wine_prefix(mut self, prefix: impl Into<PathBuf>) -> Self {
    if let Self::Wine(wine) = &mut self {
      wine.prefix = Some(prefix.into());
    }

    self
  }

  #[must_use]
  pub fn with_path_mapping(mut self, mapping: PathMapping) -> Self {
    if let Self::Wine(wine) = &mut self {
      wine.path_mappings.push(mapping);
    }

    self
  }

  pub(crate) fn apply(&self, command: CommandSpec) -> CommandSpec {
    match self {
      Self::Direct => command,
      Self::Wine(wine) => wine.apply(command),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WineLaunch {
  pub executable: PathBuf,
  pub prefix: Option<PathBuf>,
  pub path_mappings: Vec<PathMapping>,
}

impl WineLaunch {
  fn apply(&self, command: CommandSpec) -> CommandSpec {
    let mut args = Vec::with_capacity(command.args.len() + 1);

    args.push(OsString::from(self.map_path_text(&command.program)));
    args.extend(
      command
        .args
        .iter()
        .map(|arg| OsString::from(self.map_arg_text(&arg.to_string_lossy()))),
    );

    let mut wrapped = CommandSpec::new(command.tool, &self.executable, args, command.working_dir);

    if let Some(prefix) = &self.prefix {
      wrapped = wrapped.with_env("WINEPREFIX", prefix.as_os_str());
    }

    wrapped
  }

  fn map_path_text(&self, path: &Path) -> String {
    self.map_arg_text(&path.to_string_lossy())
  }

  fn map_arg_text(&self, value: &str) -> String {
    self
      .path_mappings
      .iter()
      .find_map(|mapping| mapping.map(value))
      .unwrap_or_else(|| value.to_string())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathMapping {
  pub host_prefix: PathBuf,
  pub tool_prefix: String,
}

impl PathMapping {
  #[must_use]
  pub fn new(host_prefix: impl Into<PathBuf>, tool_prefix: impl Into<String>) -> Self {
    Self {
      host_prefix: host_prefix.into(),
      tool_prefix: tool_prefix.into(),
    }
  }

  fn map(&self, value: &str) -> Option<String> {
    let raw_host_prefix = self.host_prefix.to_string_lossy();
    let trimmed_host_prefix = raw_host_prefix.trim_end_matches(['\\', '/']);

    let host_prefix = if trimmed_host_prefix.is_empty() {
      raw_host_prefix.as_ref()
    } else {
      trimmed_host_prefix
    };

    let value_matches = if cfg!(windows) {
      value
        .to_ascii_lowercase()
        .starts_with(&host_prefix.to_ascii_lowercase())
    } else {
      value.starts_with(host_prefix)
    };

    if !value_matches {
      return None;
    }

    let rest = &value[host_prefix.len()..];
    if !rest.is_empty() && !rest.starts_with(['\\', '/']) {
      return None;
    }

    let rest = rest.trim_start_matches(['\\', '/']).replace('/', "\\");
    if rest.is_empty() {
      Some(self.tool_prefix.clone())
    } else {
      Some(format!(
        "{}\\{rest}",
        self.tool_prefix.trim_end_matches('\\')
      ))
    }
  }
}
