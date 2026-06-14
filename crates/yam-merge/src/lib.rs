pub(crate) mod engines;
mod errors;
mod merger;

pub use errors::MergeError;
pub use merger::{MergeInput, MergeResult, MergeableFileType, merge};
