-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_caches (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    key VARCHAR NOT NULL UNIQUE,
    type INTEGER NOT NULL,
    value TEXT NOT NULL,
    attach TEXT NOT NULL DEFAULT '',
    valid_time_length INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME
);

CREATE UNIQUE INDEX "system_caches_key_type_key" ON "system_caches"("key", "type");