pub(crate) mod engines;
mod error;
mod markers;
mod merger;

pub use error::MergeError;
pub use merger::{MergeInput, MergeResult, MergeableFileType, merge};
