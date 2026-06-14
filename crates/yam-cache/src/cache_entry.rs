use std::str::FromStr;

use serde_json::Value;

use crate::{CacheError, ContentHash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProducerKind {
  BundleIndexer,
  ScriptMerger,
  XmlMerger,
  CsvMerger,
  BundlePacker,
}

impl ProducerKind {
  #[must_use]
  pub(crate) const fn as_storage(self) -> &'static str {
    match self {
      Self::BundleIndexer => "bundle_indexer",
      Self::ScriptMerger => "script_merger",
      Self::XmlMerger => "xml_merger",
      Self::CsvMerger => "csv_merger",
      Self::BundlePacker => "bundle_packer",
    }
  }
}

impl FromStr for ProducerKind {
  type Err = CacheError;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value {
      "bundle_indexer" => Ok(Self::BundleIndexer),
      "script_merger" => Ok(Self::ScriptMerger),
      "xml_merger" => Ok(Self::XmlMerger),
      "csv_merger" => Ok(Self::CsvMerger),
      "bundle_packer" => Ok(Self::BundlePacker),
      _ => Err(CacheError::InvalidEnumValue {
        enum_name: "ProducerKind",
        value: value.to_string(),
      }),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProducerIdentity {
  pub kind: ProducerKind,
  pub compatibility_key: ContentHash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheEntryKind {
  BundleIndex,
  MergeResult,
  PackedBundle,
}

impl CacheEntryKind {
  #[must_use]
  pub(crate) const fn as_storage(self) -> &'static str {
    match self {
      Self::BundleIndex => "bundle_index",
      Self::MergeResult => "merge_result",
      Self::PackedBundle => "packed_bundle",
    }
  }
}

impl FromStr for CacheEntryKind {
  type Err = CacheError;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value {
      "bundle_index" => Ok(Self::BundleIndex),
      "merge_result" => Ok(Self::MergeResult),
      "packed_bundle" => Ok(Self::PackedBundle),
      _ => Err(CacheError::InvalidEnumValue {
        enum_name: "CacheEntryKind",
        value: value.to_string(),
      }),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntryInput {
  pub kind: CacheEntryKind,
  pub input_key: ContentHash,
  pub producer: ProducerIdentity,
  pub output_hash: Option<ContentHash>,
  pub metadata: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
  pub id: i64,
  pub input: CacheEntryInput,
}
