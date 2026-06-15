use ini::Properties;

use super::{BOTTOM_PRIORITY, LoadOrder, LoadOrderEntry, LoadOrderError};

impl LoadOrder {
  pub fn parse(text: &str) -> Result<Self, LoadOrderError> {
    tracing::debug!("parsing load order");
    let ini = ini::Ini::load_from_str(text)?;
    let mut entries = Vec::new();

    for (section, properties) in &ini {
      let Some(mod_name) = section else {
        continue;
      };

      let entry = parse_entry(mod_name, properties)?;
      tracing::trace!(
        mod_name = %entry.mod_name,
        enabled = entry.enabled,
        priority = entry.priority,
        "load order entry parsed"
      );
      entries.push(entry);
    }

    tracing::debug!(entry_count = entries.len(), "load order parsed");
    Ok(Self::from_entries(entries))
  }
}

fn parse_entry(mod_name: &str, properties: &Properties) -> Result<LoadOrderEntry, LoadOrderError> {
  let enabled = parse_enabled(mod_name, require_property(mod_name, properties, "Enabled")?)?;
  let priority = parse_priority(
    mod_name,
    require_property(mod_name, properties, "Priority")?,
  )?;

  Ok(LoadOrderEntry {
    mod_name: mod_name.to_string(),
    enabled,
    priority,
  })
}

fn require_property<'a>(
  mod_name: &str,
  properties: &'a Properties,
  field: &'static str,
) -> Result<&'a str, LoadOrderError> {
  properties
    .iter()
    .find(|(key, _)| key.eq_ignore_ascii_case(field))
    .map(|(_, value)| value)
    .ok_or_else(|| LoadOrderError::MissingField {
      mod_name: mod_name.to_string(),
      field,
    })
}

fn parse_enabled(mod_name: &str, value: &str) -> Result<bool, LoadOrderError> {
  match value.trim() {
    "0" => Ok(false),
    "1" => Ok(true),
    value => Err(LoadOrderError::InvalidEnabled {
      mod_name: mod_name.to_string(),
      value: value.to_string(),
    }),
  }
}

fn parse_priority(mod_name: &str, value: &str) -> Result<u16, LoadOrderError> {
  let priority = value
    .trim()
    .parse::<i32>()
    .map_err(|_| LoadOrderError::InvalidPriority {
      mod_name: mod_name.to_string(),
      value: value.to_string(),
    })?;

  if !(0..=i32::from(BOTTOM_PRIORITY)).contains(&priority) {
    return Err(LoadOrderError::PriorityOutOfRange {
      mod_name: mod_name.to_string(),
      priority,
    });
  }

  u16::try_from(priority).map_err(|_| LoadOrderError::PriorityOutOfRange {
    mod_name: mod_name.to_string(),
    priority,
  })
}
