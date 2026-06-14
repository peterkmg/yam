use std::str::FromStr;

use rusqlite::{OptionalExtension, params};

use crate::{
  CacheEntry,
  CacheEntryInput,
  CacheEntryKind,
  CacheError,
  CacheStore,
  ContentHash,
  ProducerIdentity,
  ProducerKind,
};

impl CacheStore {
  pub fn put_entry(&self, input: &CacheEntryInput) -> Result<CacheEntry, CacheError> {
    let metadata_json = serde_json::to_string(&input.metadata)?;
    let output_hash = input.output_hash.map(|hash| hash.as_bytes().to_vec());

    let mut statement = self.connection.prepare_cached(
      r"
        INSERT INTO cache_entries
          (entry_kind, input_key, producer_kind, producer_compatibility_key, output_hash, metadata_json)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT(entry_kind, input_key, producer_kind, producer_compatibility_key)
        DO UPDATE SET
          output_hash = excluded.output_hash,
          metadata_json = excluded.metadata_json,
          created_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        RETURNING id
      ",
    )?;

    let id = statement.query_row(
      params![
        input.kind.as_storage(),
        input.input_key.as_bytes().as_slice(),
        input.producer.kind.as_storage(),
        input.producer.compatibility_key.as_bytes().as_slice(),
        output_hash,
        metadata_json,
      ],
      |row| row.get(0),
    )?;

    Ok(CacheEntry {
      id,
      input: input.clone(),
    })
  }

  pub fn get_entry(
    &self,
    kind: CacheEntryKind,
    input_key: &ContentHash,
    producer: &ProducerIdentity,
  ) -> Result<Option<CacheEntry>, CacheError> {
    let mut statement = self.connection.prepare_cached(
      r"
          SELECT id, entry_kind, input_key, producer_kind, producer_compatibility_key, output_hash, metadata_json
          FROM cache_entries
          WHERE entry_kind = ?1
            AND input_key = ?2
            AND producer_kind = ?3
            AND producer_compatibility_key = ?4
        ",
    )?;

    statement
      .query_row(
        params![
          kind.as_storage(),
          input_key.as_bytes().as_slice(),
          producer.kind.as_storage(),
          producer.compatibility_key.as_bytes().as_slice(),
        ],
        |row| {
          Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Vec<u8>>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Vec<u8>>(4)?,
            row.get::<_, Option<Vec<u8>>>(5)?,
            row.get::<_, String>(6)?,
          ))
        },
      )
      .optional()?
      .map(
        |(
          id,
          entry_kind,
          input_key,
          producer_kind,
          producer_compatibility_key,
          output_hash,
          metadata_json,
        )| {
          Ok(CacheEntry {
            id,
            input: CacheEntryInput {
              kind: CacheEntryKind::from_str(&entry_kind)?,
              input_key: ContentHash::from_slice(&input_key)?,
              producer: ProducerIdentity {
                kind: ProducerKind::from_str(&producer_kind)?,
                compatibility_key: ContentHash::from_slice(&producer_compatibility_key)?,
              },
              output_hash: output_hash
                .as_deref()
                .map(ContentHash::from_slice)
                .transpose()?,
              metadata: serde_json::from_str(&metadata_json)?,
            },
          })
        },
      )
      .transpose()
  }
}
