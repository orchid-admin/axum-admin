-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_login_logs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    type INTEGER NOT NULL DEFAULT 1,
    user_id INTEGER NOT NULL,
    ip_address VARCHAR NOT NULL,
    ip_address_name VARCHAR NOT NULL DEFAULT '',
    browser_agent VARCHAR NOT NULL DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "system_login_logs_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "system_users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);