use std::{cmp::Ordering, collections::BTreeMap};

pub const TOP_PRIORITY: u16 = 0;
pub const BOTTOM_PRIORITY: u16 = 9999;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOrder {
  entries: Vec<LoadOrderEntry>,
  entry_indices: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOrderEntry {
  pub mod_name: String,
  pub enabled: bool,
  pub priority: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLoadOrder {
  pub configured: bool,
  pub enabled: bool,
  pub priority: Option<u16>,
}

impl LoadOrder {
  #[must_use]
  pub fn empty() -> Self {
    Self::from_entries(Vec::new())
  }

  #[must_use]
  pub(crate) fn from_entries(mut entries: Vec<LoadOrderEntry>) -> Self {
    entries.sort_by(compare_entries);

    let entry_indices = entries
      .iter()
      .enumerate()
      .map(|(index, entry)| (normalize_mod_name(&entry.mod_name), index))
      .collect();

    Self {
      entries,
      entry_indices,
    }
  }

  #[must_use]
  pub fn entries(&self) -> &[LoadOrderEntry] {
    &self.entries
  }

  #[must_use]
  pub fn entry_for(&self, mod_name: &str) -> Option<&LoadOrderEntry> {
    self
      .entry_indices
      .get(&normalize_mod_name(mod_name))
      .map(|index| &self.entries[*index])
  }

  #[must_use]
  pub fn state_for(&self, mod_name: &str) -> SourceLoadOrder {
    self.entry_for(mod_name).map_or(
      SourceLoadOrder {
        configured: false,
        enabled: true,
        priority: None,
      },
      |entry| SourceLoadOrder {
        configured: true,
        enabled: entry.enabled,
        priority: Some(entry.priority),
      },
    )
  }

  #[must_use]
  pub fn compare_sources(&self, left: &str, right: &str) -> Ordering {
    let left_state = self.state_for(left);
    let right_state = self.state_for(right);

    match (left_state.enabled, right_state.enabled) {
      (true, false) => return Ordering::Less,
      (false, true) => return Ordering::Greater,
      _ => {}
    }

    match (left_state.priority, right_state.priority) {
      (Some(left), Some(right)) => left.cmp(&right),
      (Some(_), None) => Ordering::Less,
      (None, Some(_)) => Ordering::Greater,
      (None, None) => Ordering::Equal,
    }
    .then_with(|| compare_game_mod_names(left, right))
  }
}

#[must_use]
pub fn compare_game_mod_names(left: &str, right: &str) -> Ordering {
  left.to_ascii_lowercase().cmp(&right.to_ascii_lowercase())
}

fn compare_entries(left: &LoadOrderEntry, right: &LoadOrderEntry) -> Ordering {
  match (left.enabled, right.enabled) {
    (true, false) => Ordering::Less,
    (false, true) => Ordering::Greater,
    _ => left
      .priority
      .cmp(&right.priority)
      .then_with(|| compare_game_mod_names(&left.mod_name, &right.mod_name)),
  }
}

fn normalize_mod_name(mod_name: &str) -> String {
  mod_name.to_ascii_lowercase()
}
