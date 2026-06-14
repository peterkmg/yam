#![allow(unused_crate_dependencies)]

#[path = "support/files.rs"]
mod files;
#[path = "support/runner.rs"]
mod runner_support;

use yam_tools::{LaunchMode, PathMapping, QuickBms};

#[test]
fn wine_launch_wraps_quickbms_command_and_maps_paths() {
  let files = files::TestDir::new();
  let wine = files.file("wine");
  let exe = files.file("tools/quickbms.exe");
  let script = files.file("tools/witcher3.bms");
  let bundle = files.file("game/content/blob0.bundle");
  let runner = runner_support::RunnerSpy::new(vec![runner_support::success(
    "  00000001 1 gameplay\\items\\item.xml\n",
    "",
  )]);
  let launch = LaunchMode::wine(wine.clone())
    .with_path_mapping(PathMapping::new(files.path(""), r"Z:\yam-test"));
  let quickbms = QuickBms::with_runner(&exe, &script, runner.clone()).with_launch_mode(launch);

  quickbms.list_bundle(&bundle).expect("listing should run");

  let commands = runner.commands();

  assert_eq!(commands.len(), 1);
  assert_eq!(commands[0].program, wine);
  assert_eq!(
    commands[0].args[0].to_string_lossy(),
    r"Z:\yam-test\tools\quickbms.exe"
  );
  assert_eq!(commands[0].args[1], "-l");
  assert_eq!(
    commands[0].args[2].to_string_lossy(),
    r"Z:\yam-test\tools\witcher3.bms"
  );
  assert_eq!(
    commands[0].args[3].to_string_lossy(),
    r"Z:\yam-test\game\content\blob0.bundle"
  );
}
