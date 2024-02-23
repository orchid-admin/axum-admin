-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL UNIQUE,
    nickname VARCHAR NOT NULL DEFAULT '',
    role_id INTEGER,
    dept_id INTEGER,
    phone VARCHAR NOT NULL DEFAULT '',
    email VARCHAR NOT NULL DEFAULT '',
    sex INTEGER NOT NULL DEFAULT 1,
    password VARCHAR NOT NULL DEFAULT '',
    salt VARCHAR NOT NULL DEFAULT '',
    describe VARCHAR NOT NULL DEFAULT '',
    expire_time DATETIME,
    status INTEGER NOT NULL DEFAULT 0,
    last_login_ip VARCHAR NOT NULL DEFAULT '',
    last_login_time DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME,
    CONSTRAINT "system_users_role_id_fkey" FOREIGN KEY ("role_id") REFERENCES "system_roles" ("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "system_users_dept_id_fkey" FOREIGN KEY ("dept_id") REFERENCES "system_depts" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);