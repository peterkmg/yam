use rusqlite::{Connection, OptionalExtension, params};

use crate::{
  ArtifactInput,
  ArtifactKey,
  CacheError,
  CacheStore,
  ContentHash,
  ObservationResult,
  ObservationStatus,
  artifact::ArtifactRecord,
  hash::digest_file,
  util::sqlite_len,
};

struct StoredArtifact {
  hash: ContentHash,
  byte_len: u64,
}

impl CacheStore {
  pub fn observe_file(&self, input: &ArtifactInput) -> Result<ObservationResult, CacheError> {
    observe_file_on(&self.connection, input)
  }

  pub fn observe_many(
    &mut self,
    inputs: &[ArtifactInput],
  ) -> Result<Vec<ObservationResult>, CacheError> {
    let transaction = self.connection.transaction()?;
    let mut results = Vec::with_capacity(inputs.len());

    for input in inputs {
      results.push(observe_file_on(&transaction, input)?);
    }

    transaction.commit()?;
    Ok(results)
  }
}

fn observe_file_on(
  connection: &Connection,
  input: &ArtifactInput,
) -> Result<ObservationResult, CacheError> {
  let (hash, byte_len) = digest_file(&input.disk_path)?;
  let sqlite_byte_len = sqlite_len(byte_len, "artifact byte length")?;
  let existing = select_stored_artifact(connection, &input.key)?;

  let status = match existing {
    None => ObservationStatus::New,
    Some(artifact) if artifact.hash == hash && artifact.byte_len == byte_len => {
      ObservationStatus::Unchanged
    }
    Some(artifact) => ObservationStatus::Changed {
      previous_hash: artifact.hash,
    },
  };
  tracing::trace!(
    source_id = input.key.source_id.as_str(),
    source_role = input.key.source_role.as_storage(),
    artifact_kind = input.key.kind.as_storage(),
    logical_path = input.key.logical_path.as_str(),
    hash = ?hash,
    byte_len,
    status = ?status,
    "observed artifact"
  );

  let id = upsert_observed_artifact(connection, input, &hash, sqlite_byte_len)?;
  let artifact = ArtifactRecord {
    id,
    input: input.clone(),
    hash,
    byte_len,
  };

  Ok(ObservationResult { artifact, status })
}

fn upsert_observed_artifact(
  connection: &Connection,
  input: &ArtifactInput,
  hash: &ContentHash,
  byte_len: i64,
) -> Result<i64, CacheError> {
  let mut statement = connection.prepare_cached(
    "INSERT INTO observed_artifacts
       (source_id, source_role, artifact_kind, logical_path, hash, byte_len, observed_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
     ON CONFLICT(source_id, source_role, artifact_kind, logical_path)
     DO UPDATE SET
       hash = excluded.hash,
       byte_len = excluded.byte_len,
       observed_at = excluded.observed_at
     RETURNING id",
  )?;

  Ok(statement.query_row(
    params![
      input.key.source_id.as_str(),
      input.key.source_role.as_storage(),
      input.key.kind.as_storage(),
      input.key.logical_path.as_str(),
      hash.as_bytes().as_slice(),
      byte_len,
    ],
    |row| row.get(0),
  )?)
}

fn select_stored_artifact(
  connection: &Connection,
  key: &ArtifactKey,
) -> Result<Option<StoredArtifact>, CacheError> {
  let mut statement = connection.prepare_cached(
    "SELECT hash, byte_len
       FROM observed_artifacts
       WHERE source_id = ?1
         AND source_role = ?2
         AND artifact_kind = ?3
         AND logical_path = ?4",
  )?;

  statement
    .query_row(
      params![
        key.source_id.as_str(),
        key.source_role.as_storage(),
        key.kind.as_storage(),
        key.logical_path.as_str(),
      ],
      |row| Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, i64>(1)?)),
    )
    .optional()
    .map_err(CacheError::from)?
    .map(|(hash, byte_len)| {
      Ok(StoredArtifact {
        hash: ContentHash::from_slice(&hash)?,
        byte_len: byte_len
          .try_into()
          .map_err(|_| CacheError::ValueTooLarge("artifact byte length"))?,
      })
    })
    .transpose()
}
