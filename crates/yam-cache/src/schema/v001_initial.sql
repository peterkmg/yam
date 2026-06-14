CREATE TABLE observed_artifacts (
  id INTEGER PRIMARY KEY,
  source_id TEXT NOT NULL,
  source_role TEXT NOT NULL CHECK (source_role IN ('vanilla', 'mod', 'generated')),
  artifact_kind TEXT NOT NULL CHECK (artifact_kind IN ('loose_file', 'bundle', 'bundle_entry')),
  logical_path TEXT NOT NULL,
  hash BLOB NOT NULL CHECK (length(hash) = 32),
  byte_len INTEGER NOT NULL CHECK (byte_len >= 0),
  observed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  UNIQUE (source_id, source_role, artifact_kind, logical_path)
);

CREATE INDEX observed_artifacts_hash_idx
ON observed_artifacts (hash);

CREATE TABLE blobs (
  hash BLOB PRIMARY KEY CHECK (length(hash) = 32),
  byte_len INTEGER NOT NULL CHECK (byte_len >= 0),
  stored_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE cache_entries (
  id INTEGER PRIMARY KEY,
  entry_kind TEXT NOT NULL CHECK (entry_kind IN ('bundle_index', 'merge_result', 'packed_bundle')),
  input_key BLOB NOT NULL CHECK (length(input_key) = 32),
  producer_kind TEXT NOT NULL CHECK (
    producer_kind IN (
      'bundle_indexer',
      'script_merger',
      'xml_merger',
      'csv_merger',
      'bundle_packer'
    )
  ),
  producer_compatibility_key BLOB NOT NULL CHECK (length(producer_compatibility_key) = 32),
  output_hash BLOB REFERENCES blobs(hash) ON DELETE RESTRICT,
  metadata_json TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  UNIQUE (entry_kind, input_key, producer_kind, producer_compatibility_key)
);

CREATE INDEX cache_entries_output_hash_idx
ON cache_entries (output_hash);
