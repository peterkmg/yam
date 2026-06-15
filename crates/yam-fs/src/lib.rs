#![cfg_attr(test, allow(unused_crate_dependencies))]

mod error;
mod logical;
mod physical;
mod text;

pub use error::FsError;
pub use logical::LogicalPath;
pub use physical::{
  WalkedFile,
  read_bytes,
  relative_path,
  remove_dir_contents,
  remove_file_if_exists,
  utf8_path,
  walk_files,
  write_bytes,
};
pub use text::{
  DecodedText,
  TextEncoding,
  decode_text,
  encode_text,
  read_text_file,
  write_text_file,
};
