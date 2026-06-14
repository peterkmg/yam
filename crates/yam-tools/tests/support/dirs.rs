use std::{fs, path::PathBuf};

use crate::files::TestDir;

pub fn dir(files: &TestDir, relative: &str) -> PathBuf {
  let path = files.path(relative);
  fs::create_dir_all(&path).unwrap();
  path
}
