use std::{
  fs,
  path::{Path, PathBuf},
};

use tempfile::TempDir;

pub struct TestDir {
  temp: TempDir,
}

impl TestDir {
  pub fn new() -> Self {
    Self {
      temp: TempDir::new().unwrap(),
    }
  }

  pub fn path(&self, relative: impl AsRef<Path>) -> PathBuf {
    self.temp.path().join(relative)
  }

  pub fn file(&self, relative: impl AsRef<Path>) -> PathBuf {
    let path = self.path(relative);
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, "").unwrap();
    path
  }
}
