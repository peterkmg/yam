use crate::CacheError;

pub fn sqlite_len(value: u64, name: &'static str) -> Result<i64, CacheError> {
  value
    .try_into()
    .map_err(|_| CacheError::ValueTooLarge(name))
}
