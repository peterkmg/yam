#![allow(unused_crate_dependencies)]

use std::cmp::Ordering;

use yam_core::{LoadOrder, compare_game_mod_names};

#[test]
fn mods_settings_entries_are_parsed() {
  let order = LoadOrder::parse(
    "[modBeta]
Enabled=1
Priority=12
VK=ignored

[modAlpha]
Enabled=0
Priority=3
",
  )
  .unwrap();

  let beta = order.entry_for("MODBETA").unwrap();
  let alpha = order.entry_for("modalpha").unwrap();

  assert!(beta.enabled);
  assert_eq!(beta.priority, 12);
  assert!(!alpha.enabled);
  assert_eq!(alpha.priority, 3);
}

#[test]
fn invalid_enabled_value_is_rejected() {
  let error = LoadOrder::parse("[modAlpha]\nEnabled=yes\nPriority=1\n").unwrap_err();

  assert!(error.to_string().contains("Enabled"));
}

#[test]
fn game_mod_name_order_is_case_insensitive_ascii() {
  assert_eq!(compare_game_mod_names("modA", "moda"), Ordering::Equal);
  assert_eq!(compare_game_mod_names("mod2", "mod_"), Ordering::Less);
  assert_eq!(compare_game_mod_names("mod_", "moda"), Ordering::Less);
}
