use yam_core::{ClassifiedConflict, ClassifiedConflictAction, MergeableFileType};

pub fn print_conflicts(conflicts: &[ClassifiedConflict]) {
  println!("conflicts: {}", conflicts.len());
  println!(
    "merge required: {}",
    action_count(conflicts, ClassifiedConflictAction::MergeRequired)
  );
  println!(
    "load-order resolved: {}",
    action_count(conflicts, ClassifiedConflictAction::LoadOrderResolved)
  );
  println!(
    "no active sources: {}",
    action_count(conflicts, ClassifiedConflictAction::NoActiveSources)
  );

  for conflict in conflicts {
    println!();
    println!(
      "[{}] {}",
      action_label(conflict.action),
      conflict.relative_path
    );
    println!("type: {}", merge_type_label(conflict.merge_file_type));
    if let Some(winner) = conflict.sources.iter().find(|source| source.is_winner) {
      println!("winner: {}", winner.source.mod_name);
    }

    for source in &conflict.sources {
      println!(
        "source: {} priority={} {} {}",
        source.source.mod_name,
        priority_label(source.load_order.priority),
        enabled_label(source.load_order.enabled),
        changed_label(source.source.changed)
      );
    }
  }
}

fn action_count(conflicts: &[ClassifiedConflict], action: ClassifiedConflictAction) -> usize {
  conflicts
    .iter()
    .filter(|conflict| conflict.action == action)
    .count()
}

const fn action_label(action: ClassifiedConflictAction) -> &'static str {
  match action {
    ClassifiedConflictAction::MergeRequired => "merge-required",
    ClassifiedConflictAction::LoadOrderResolved => "load-order-resolved",
    ClassifiedConflictAction::NoActiveSources => "no-active-sources",
  }
}

const fn merge_type_label(file_type: Option<MergeableFileType>) -> &'static str {
  match file_type {
    Some(MergeableFileType::WitcherScript) => "witcherscript",
    Some(MergeableFileType::Xml) => "xml",
    Some(MergeableFileType::Csv) => "csv",
    None => "unsupported",
  }
}

fn priority_label(priority: Option<u16>) -> String {
  priority.map_or_else(|| "none".to_string(), |priority| priority.to_string())
}

const fn enabled_label(enabled: bool) -> &'static str {
  if enabled { "enabled" } else { "disabled" }
}

const fn changed_label(changed: bool) -> &'static str {
  if changed { "changed" } else { "unchanged" }
}
