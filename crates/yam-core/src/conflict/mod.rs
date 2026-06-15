mod classify;
mod model;

pub use classify::classify_conflicts;
pub use model::{ClassifiedConflict, ClassifiedConflictAction, ClassifiedConflictSource};
