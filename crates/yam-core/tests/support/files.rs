use std::fs;

use camino::Utf8Path;

pub fn write_file(path: &Utf8Path, contents: &str) {
  fs::create_dir_all(path.parent().unwrap()).unwrap();
  fs::write(path, contents).unwrap();
}
