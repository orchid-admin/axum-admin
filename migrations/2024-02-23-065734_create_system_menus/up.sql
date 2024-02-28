-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_menus (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    parent_id INTEGER NOT NULL DEFAULT 0,
    type INTEGER NOT NULL DEFAULT 1,
    title VARCHAR NOT NULL,
    icon VARCHAR NOT NULL DEFAULT '',
    router_name VARCHAR NOT NULL DEFAULT '',
    router_component VARCHAR NOT NULL DEFAULT '',
    router_path VARCHAR NOT NULL DEFAULT '',
    redirect VARCHAR NOT NULL DEFAULT '',
    link VARCHAR NOT NULL DEFAULT '',
    iframe VARCHAR NOT NULL DEFAULT '',
    btn_auth VARCHAR NOT NULL DEFAULT '',
    api_url VARCHAR NOT NULL DEFAULT '',
    api_method VARCHAR NOT NULL DEFAULT '',
    is_hide INTEGER NOT NULL DEFAULT 0,
    is_keep_alive INTEGER NOT NULL DEFAULT 1,
    is_affix INTEGER NOT NULL DEFAULT 0,
    sort INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME
);