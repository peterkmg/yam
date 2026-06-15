#![allow(dead_code)]

use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use tempfile::TempDir;
use yam_cache::{
  ArtifactInput,
  ArtifactKey,
  ArtifactKind,
  CacheStore,
  ContentHash,
  LogicalPath,
  ProducerIdentity,
  ProducerKind,
  SourceId,
  SourceRole,
};

pub struct CacheCase {
  _temp: TempDir,
  root: Utf8PathBuf,
  store: CacheStore,
}

impl CacheCase {
  pub fn in_memory() -> Self {
    let temp = TempDir::new().unwrap();
    let root = Utf8Path::from_path(temp.path()).unwrap().to_path_buf();
    let store = CacheStore::open_in_memory(&root).unwrap();
    Self {
      _temp: temp,
      root,
      store,
    }
  }

  pub const fn store(&self) -> &CacheStore {
    &self.store
  }

  pub const fn store_mut(&mut self) -> &mut CacheStore {
    &mut self.store
  }

  pub fn root(&self) -> &Utf8Path {
    &self.root
  }
}

pub fn artifact_input(path: &Utf8Path, source_id: &str) -> ArtifactInput {
  artifact_input_with_path(path, source_id, "content/scripts/input.ws")
}

pub fn artifact_input_with_path(
  disk_path: &Utf8Path,
  source_id: &str,
  logical_path: &str,
) -> ArtifactInput {
  ArtifactInput {
    key: ArtifactKey::new(
      SourceId::new(source_id),
      SourceRole::Mod,
      ArtifactKind::LooseFile,
      LogicalPath::new(logical_path).unwrap(),
    ),
    disk_path: disk_path.to_path_buf(),
  }
}

pub fn write_file(path: &Utf8Path, contents: &str) {
  fs::write(path, contents).unwrap();
}

pub fn producer(kind: ProducerKind, compatibility_key: &[u8]) -> ProducerIdentity {
  ProducerIdentity {
    kind,
    compatibility_key: ContentHash::digest(compatibility_key),
  }
}
