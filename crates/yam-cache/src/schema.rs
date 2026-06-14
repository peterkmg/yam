use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

use crate::CacheError;

pub const CACHE_SCHEMA_VERSION: u32 = 1;

const MIGRATION_ITEMS: &[M<'_>] = &[M::up(include_str!("schema/v001_initial.sql"))];
const MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATION_ITEMS);

#[must_use]
pub const fn schema_version() -> u32 {
  CACHE_SCHEMA_VERSION
}

pub fn migrate(connection: &mut Connection) -> Result<(), CacheError> {
  MIGRATIONS.to_latest(connection)?;
  Ok(())
}
