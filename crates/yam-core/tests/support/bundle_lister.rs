use std::{cell::RefCell, collections::BTreeMap};

use camino::Utf8Path;
use yam_core::{BundleLister, ListedBundleEntry};

#[derive(Debug)]
pub struct RecordingBundleLister {
  entries: BTreeMap<String, Vec<ListedBundleEntry>>,
  calls: RefCell<Vec<camino::Utf8PathBuf>>,
}

impl RecordingBundleLister {
  pub fn new<'a>(entries: impl IntoIterator<Item = (&'a str, Vec<ListedBundleEntry>)>) -> Self {
    Self {
      entries: entries
        .into_iter()
        .map(|(path, entries)| (normalize_path(path), entries))
        .collect(),
      calls: RefCell::new(Vec::new()),
    }
  }

  pub fn calls(&self) -> Vec<camino::Utf8PathBuf> {
    self.calls.borrow().clone()
  }
}

impl BundleLister for RecordingBundleLister {
  fn list_bundle(
    &self,
    bundle_path: &Utf8Path,
  ) -> Result<Vec<ListedBundleEntry>, yam_core::ScanError> {
    self.calls.borrow_mut().push(bundle_path.to_path_buf());
    Ok(
      self
        .entries
        .get(&normalize_path(bundle_path.as_str()))
        .cloned()
        .unwrap_or_default(),
    )
  }
}

fn normalize_path(path: &str) -> String {
  path.replace('\\', "/")
}
