use std::path::Path;

use crate::{FsError, read_bytes, write_bytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
  Utf8,
  Utf8Bom,
  Utf16LeBom,
  Utf16BeBom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedText {
  pub text: String,
  pub encoding: TextEncoding,
}

pub fn decode_text(bytes: &[u8]) -> Result<DecodedText, FsError> {
  if let Some(bytes) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
    return Ok(DecodedText {
      text: decode_utf8(bytes)?,
      encoding: TextEncoding::Utf8Bom,
    });
  }

  if let Some(bytes) = bytes.strip_prefix(&[0xFF, 0xFE]) {
    return Ok(DecodedText {
      text: decode_utf16(bytes, u16::from_le_bytes)?,
      encoding: TextEncoding::Utf16LeBom,
    });
  }

  if let Some(bytes) = bytes.strip_prefix(&[0xFE, 0xFF]) {
    return Ok(DecodedText {
      text: decode_utf16(bytes, u16::from_be_bytes)?,
      encoding: TextEncoding::Utf16BeBom,
    });
  }

  Ok(DecodedText {
    text: decode_utf8(bytes)?,
    encoding: TextEncoding::Utf8,
  })
}

pub fn read_text_file(path: impl AsRef<Path>) -> Result<DecodedText, FsError> {
  decode_text(&read_bytes(path)?)
}

pub fn write_text_file(
  path: impl AsRef<Path>,
  text: &str,
  encoding: TextEncoding,
) -> Result<(), FsError> {
  write_bytes(path, encode_text(text, encoding))
}

#[must_use]
pub fn encode_text(text: &str, encoding: TextEncoding) -> Vec<u8> {
  match encoding {
    TextEncoding::Utf8 => text.as_bytes().to_vec(),
    TextEncoding::Utf8Bom => {
      let mut bytes = Vec::with_capacity(text.len() + 3);
      bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
      bytes.extend_from_slice(text.as_bytes());
      bytes
    }
    TextEncoding::Utf16LeBom => encode_utf16(text, [0xFF, 0xFE], u16::to_le_bytes),
    TextEncoding::Utf16BeBom => encode_utf16(text, [0xFE, 0xFF], u16::to_be_bytes),
  }
}

fn decode_utf8(bytes: &[u8]) -> Result<String, FsError> {
  std::str::from_utf8(bytes)
    .map(str::to_string)
    .map_err(|source| FsError::InvalidUtf8 { source })
}

fn decode_utf16(bytes: &[u8], convert: fn([u8; 2]) -> u16) -> Result<String, FsError> {
  if !bytes.len().is_multiple_of(2) {
    return Err(FsError::OddUtf16Length);
  }

  let words = bytes
    .chunks_exact(2)
    .map(|chunk| convert([chunk[0], chunk[1]]))
    .collect::<Vec<_>>();

  String::from_utf16(&words).map_err(|source| FsError::InvalidUtf16 { source })
}

fn encode_utf16(text: &str, bom: [u8; 2], convert: fn(u16) -> [u8; 2]) -> Vec<u8> {
  let mut bytes = Vec::with_capacity(2 + text.len() * 2);
  bytes.extend_from_slice(&bom);

  for word in text.encode_utf16() {
    bytes.extend_from_slice(&convert(word));
  }

  bytes
}
