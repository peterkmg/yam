use std::{fs, path::Path, process::Command};

pub fn yam_command() -> Command {
  Command::new(env!("CARGO_BIN_EXE_yam"))
}

#[allow(dead_code)]
pub fn stdout(output: std::process::Output) -> String {
  String::from_utf8(output.stdout).unwrap()
}

#[allow(dead_code)]
pub fn stderr(output: std::process::Output) -> String {
  String::from_utf8(output.stderr).unwrap()
}

#[allow(dead_code)]
pub fn write_file(path: &Path, contents: &str) {
  fs::create_dir_all(path.parent().unwrap()).unwrap();
  fs::write(path, contents).unwrap();
}

#[allow(dead_code)]
pub fn write_quickbms_list_tool(path: &Path, entries: &[&str]) {
  fs::create_dir_all(path.parent().unwrap()).unwrap();

  #[cfg(windows)]
  {
    let mut contents = String::from("@echo off\r\n");
    for entry in entries {
      contents.push_str("echo ");
      contents.push_str(entry);
      contents.push_str("\r\n");
    }
    contents.push_str("exit /B 0\r\n");
    fs::write(path, contents).unwrap();
  }

  #[cfg(not(windows))]
  {
    use std::os::unix::fs::PermissionsExt;

    let mut contents = String::from("#!/bin/sh\n");
    for entry in entries {
      contents.push_str("printf '%s\\n' '");
      contents.push_str(entry);
      contents.push_str("'\n");
    }
    fs::write(path, contents).unwrap();

    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
  }
}
