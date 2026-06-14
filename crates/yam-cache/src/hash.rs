use std::{
  fmt::{self, Write as _},
  fs::File,
  io::Read,
  path::Path,
};

use crate::CacheError;

const HASH_BUFFER_SIZE: usize = 64 * 1024;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentHash([u8; 32]);

impl ContentHash {
  #[must_use]
  pub fn digest(bytes: &[u8]) -> Self {
    Self(*blake3::hash(bytes).as_bytes())
  }

  pub fn from_slice(bytes: &[u8]) -> Result<Self, CacheError> {
    let bytes: [u8; 32] = bytes
      .try_into()
      .map_err(|_| CacheError::InvalidContentHash("expected 32 bytes".to_string()))?;
    Ok(Self(bytes))
  }

  #[must_use]
  pub const fn as_bytes(&self) -> &[u8; 32] {
    &self.0
  }

  #[must_use]
  pub fn to_hex(self) -> String {
    let mut hex = String::with_capacity(64);

    for byte in self.0 {
      write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }

    hex
  }
}

impl fmt::Debug for ContentHash {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("ContentHash").field(&self.to_hex()).finish()
  }
}

pub fn digest_file(path: impl AsRef<Path>) -> Result<(ContentHash, u64), CacheError> {
  let mut file = File::open(path)?;
  let mut hasher = blake3::Hasher::new();
  let mut byte_len = 0_u64;
  let mut buffer = vec![0_u8; HASH_BUFFER_SIZE];

  loop {
    let read = file.read(&mut buffer)?;

    if read == 0 {
      break;
    }

    let read_len =
      u64::try_from(read).map_err(|_| CacheError::ValueTooLarge("artifact byte length"))?;

    byte_len = byte_len
      .checked_add(read_len)
      .ok_or(CacheError::ValueTooLarge("artifact byte length"))?;

    hasher.update(&buffer[..read]);
  }

  Ok((ContentHash(*hasher.finalize().as_bytes()), byte_len))
}
