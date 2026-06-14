use std::{
  fmt,
  path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
  QuickBms,
  WccLite,
  ManualMerge,
  ConflictEditor,
}

impl ToolKind {
  #[must_use]
  pub const fn display_name(self) -> &'static str {
    match self {
      Self::QuickBms => "QuickBMS",
      Self::WccLite => "wcc_lite",
      Self::ManualMerge => "manual merge tool",
      Self::ConflictEditor => "conflict editor",
    }
  }
}

impl fmt::Display for ToolKind {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.display_name())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolComponent {
  Executable,
  Script,
  Bundle,
  Directory,
  File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolComponentReport {
  pub component: ToolComponent,
  pub path: PathBuf,
  pub available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolReport {
  pub kind: ToolKind,
  pub components: Vec<ToolComponentReport>,
}

impl ToolReport {
  #[must_use]
  pub fn is_available(&self) -> bool {
    self.components.iter().all(|component| component.available)
  }
}

pub fn component_report(component: ToolComponent, path: impl AsRef<Path>) -> ToolComponentReport {
  let path = path.as_ref().to_path_buf();
  let available = path.is_file();

  ToolComponentReport {
    component,
    path,
    available,
  }
}
