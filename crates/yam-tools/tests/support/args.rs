use std::path::Path;

use yam_tools::CommandSpec;

pub fn assert_arg(command: &CommandSpec, index: usize, expected: &Path) {
  assert_eq!(command.args[index], expected.as_os_str());
}
