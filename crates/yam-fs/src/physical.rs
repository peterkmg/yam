use std::{
  fs,
  path::{Path, PathBuf},
  sync::mpsc,
};

use camino::{Utf8Path, Utf8PathBuf};
use ignore::{DirEntry, WalkBuilder, WalkState};

use crate::{FsError, LogicalPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalkedFile {
  pub relative_path: LogicalPath,
  pub path: Utf8PathBuf,
}

pub fn utf8_path(path: PathBuf) -> Result<Utf8PathBuf, FsError> {
  Utf8PathBuf::from_path_buf(path).map_err(|path| FsError::NonUtf8Path { path })
}

pub fn relative_path(root: &Utf8Path, path: &Utf8Path) -> Result<LogicalPath, FsError> {
  let relative = path
    .strip_prefix(root)
    .map_err(|_| FsError::PathOutsideRoot {
      root: root.as_std_path().to_path_buf(),
      path: path.as_std_path().to_path_buf(),
    })?;

  LogicalPath::new(relative.as_str())
}

pub fn walk_files(root: &Utf8Path) -> Result<Vec<WalkedFile>, FsError> {
  tracing::debug!(root = %root, "walking files");
  let root = root.to_path_buf();
  let (sender, receiver) = mpsc::channel();
  let walker = walk_builder(&root).build_parallel();

  walker.run(|| {
    let root = root.clone();
    let sender = sender.clone();

    Box::new(move |entry| {
      if let Some(result) = walked_file(&root, entry)
        && sender.send(result).is_err()
      {
        return WalkState::Quit;
      }

      WalkState::Continue
    })
  });
  drop(sender);

  let mut files = Vec::new();
  for file in receiver {
    files.push(file?);
  }

  files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
  tracing::debug!(root = %root, file_count = files.len(), "walk completed");

  Ok(files)
}

pub fn write_bytes(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> Result<(), FsError> {
  let path = path.as_ref();

  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent).map_err(|source| FsError::io("create directory", parent, source))?;
  }

  fs::write(path, bytes).map_err(|source| FsError::io("write file", path, source))
}

pub fn read_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>, FsError> {
  let path = path.as_ref();
  fs::read(path).map_err(|source| FsError::io("read file", path, source))
}

pub fn remove_file_if_exists(path: impl AsRef<Path>) -> Result<(), FsError> {
  let path = path.as_ref();

  if path.is_file() {
    fs::remove_file(path).map_err(|source| FsError::io("remove file", path, source))?;
  }

  Ok(())
}

pub fn remove_dir_contents(root: &Utf8Path, target: &Utf8Path) -> Result<(), FsError> {
  if !target.exists() {
    return Ok(());
  }

  let root = canonicalize(root)?;
  let target = canonicalize(target)?;

  if root == target {
    return Err(FsError::RefusingRootDeletion { root });
  }

  if !target.starts_with(&root) {
    return Err(FsError::PathOutsideRoot { root, path: target });
  }

  let entries =
    fs::read_dir(&target).map_err(|source| FsError::io("read directory", &target, source))?;

  for entry in entries {
    let entry = entry.map_err(|source| FsError::io("read directory entry", &target, source))?;
    let path = entry.path();
    let file_type = entry
      .file_type()
      .map_err(|source| FsError::io("read file type", &path, source))?;

    if file_type.is_dir() {
      fs::remove_dir_all(&path).map_err(|source| FsError::io("remove directory", &path, source))?;
    } else {
      fs::remove_file(&path).map_err(|source| FsError::io("remove file", &path, source))?;
    }
  }

  Ok(())
}

fn walk_builder(root: &Utf8Path) -> WalkBuilder {
  let mut builder = WalkBuilder::new(root.as_std_path());
  builder.standard_filters(false).threads(0);

  builder
}

fn walked_file(
  root: &Utf8Path,
  entry: Result<DirEntry, ignore::Error>,
) -> Option<Result<WalkedFile, FsError>> {
  let entry = match entry {
    Ok(entry) => entry,
    Err(source) => return Some(Err(FsError::Walk { source })),
  };

  let file_type = entry.file_type()?;
  if !file_type.is_file() {
    return None;
  }

  let path = match utf8_path(entry.into_path()) {
    Ok(path) => path,
    Err(error) => return Some(Err(error)),
  };

  Some(relative_path(root, &path).map(|relative_path| {
    tracing::trace!(path = %path, relative_path = relative_path.as_str(), "walked file");
    WalkedFile {
      relative_path,
      path,
    }
  }))
}

fn canonicalize(path: &Utf8Path) -> Result<PathBuf, FsError> {
  fs::canonicalize(path.as_std_path()).map_err(|source| {
    FsError::io(
      "canonicalize path",
      path.as_std_path().to_path_buf(),
      source,
    )
  })
}
