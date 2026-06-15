use camino::Utf8Path;
use ini::{Ini, Properties};
use yam_fs::read_text_file;

use super::EnvironmentError;

pub(super) fn read_selected_profile(path: &Utf8Path) -> Result<String, EnvironmentError> {
  let text = read_text_file(path)?.text;

  let selected_profile = Ini::load_from_str(&text).map_or_else(
    |_| find_selected_profile_in_text(&text),
    |ini| find_selected_profile(&ini),
  );

  selected_profile.ok_or(EnvironmentError::MissingProfile)
}

fn find_selected_profile(ini: &Ini) -> Option<String> {
  ini
    .section(Some("General"))
    .and_then(selected_profile_from_section)
    .or_else(|| selected_profile_from_section(ini.general_section()))
}

fn selected_profile_from_section(section: &Properties) -> Option<String> {
  section
    .get("selected_profile")
    .or_else(|| section.get("selectedProfile"))
    .map(normalize_ini_value)
    .map(str::to_string)
}

fn normalize_ini_value(value: &str) -> &str {
  value
    .strip_prefix("@ByteArray(")
    .and_then(|value| value.strip_suffix(')'))
    .unwrap_or(value)
}

fn find_selected_profile_in_text(text: &str) -> Option<String> {
  let mut is_general_section = false;
  let mut global_value = None;

  for raw_line in text.lines() {
    let line = raw_line.trim();
    if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
      continue;
    }

    if let Some(section) = line
      .strip_prefix('[')
      .and_then(|line| line.strip_suffix(']'))
    {
      is_general_section = section == "General";
      continue;
    }

    let Some((key, value)) = line.split_once('=') else {
      continue;
    };
    let key = key.trim();
    if key != "selected_profile" && key != "selectedProfile" {
      continue;
    }

    let value = normalize_ini_value(value.trim()).to_string();
    if is_general_section {
      return Some(value);
    }

    if global_value.is_none() {
      global_value = Some(value);
    }
  }

  global_value
}
