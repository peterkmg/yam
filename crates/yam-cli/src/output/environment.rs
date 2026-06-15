use yam_core::ResolvedEnvironment;

pub fn print_environment(resolved: &ResolvedEnvironment) {
  println!("mode: {}", resolved.kind);
  println!("root: {}", resolved.root);
  if let Some(profile) = &resolved.profile {
    println!("profile: {profile}");
  }
  println!(
    "output: {} -> {}",
    resolved.output_mod.name, resolved.output_mod.path
  );
  println!("mods: {}", resolved.mods.len());
  for mod_source in &resolved.mods {
    println!("mod: {} -> {}", mod_source.name(), mod_source.path());
  }
}
