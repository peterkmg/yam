#![cfg_attr(test, allow(unused_crate_dependencies))]

mod artifact;
mod blob;
mod cache_entry;
mod entry;
mod error;
mod hash;
mod observed;
mod schema;
mod store;
mod util;

pub use artifact::{
  ArtifactInput,
  ArtifactKey,
  ArtifactKind,
  ArtifactRecord,
  LogicalPath,
  ObservationResult,
  ObservationStatus,
  SourceId,
  SourceRole,
};
pub use blob::BlobRef;
pub use cache_entry::{
  CacheEntry,
  CacheEntryInput,
  CacheEntryKind,
  ProducerIdentity,
  ProducerKind,
};
pub use error::CacheError;
pub use hash::ContentHash;
pub use schema::{CACHE_SCHEMA_VERSION, schema_version};
pub use store::CacheStore;
