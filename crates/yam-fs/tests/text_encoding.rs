#![allow(unused_crate_dependencies)]

use tempfile::TempDir;
use yam_fs::{TextEncoding, decode_text, encode_text, read_text_file, write_text_file};

#[test]
fn decode_text_detects_utf8_bom() {
  let decoded = decode_text(b"\xEF\xBB\xBFalpha").unwrap();

  assert_eq!(decoded.encoding, TextEncoding::Utf8Bom);
  assert_eq!(decoded.text, "alpha");
}

#[test]
fn decode_text_detects_utf16le_bom() {
  let decoded = decode_text(&[0xFF, 0xFE, b'a', 0, b'b', 0]).unwrap();

  assert_eq!(decoded.encoding, TextEncoding::Utf16LeBom);
  assert_eq!(decoded.text, "ab");
}

#[test]
fn encode_text_preserves_selected_encoding() {
  let bytes = encode_text("ab", TextEncoding::Utf16LeBom);

  assert_eq!(bytes, [0xFF, 0xFE, b'a', 0, b'b', 0]);
}

#[test]
fn text_file_round_trips_with_selected_encoding() {
  let temp = TempDir::new().unwrap();
  let path = temp.path().join("config").join("input.txt");

  write_text_file(&path, "alpha", TextEncoding::Utf16LeBom).unwrap();

  let decoded = read_text_file(&path).unwrap();
  assert_eq!(decoded.encoding, TextEncoding::Utf16LeBom);
  assert_eq!(decoded.text, "alpha");
}
