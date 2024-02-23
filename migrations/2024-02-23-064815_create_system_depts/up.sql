-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_depts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    parent_id INTEGER NOT NULL DEFAULT 0,
    name VARCHAR NOT NULL,
    person_name VARCHAR NOT NULL DEFAULT '',
    person_phone VARCHAR NOT NULL DEFAULT '',
    person_email VARCHAR NOT NULL DEFAULT '',
    describe VARCHAR NOT NULL DEFAULT '',
    status INTEGER NOT NULL DEFAULT 1,
    sort INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME
);