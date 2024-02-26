-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_action_logs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    menu_id INTEGER NOT NULL,
    menu_names VARCHAR NOT NULL DEFAULT '',
    ip_address VARCHAR NOT NULL,
    ip_address_name VARCHAR NOT NULL DEFAULT '',
    browser_agent VARCHAR NOT NULL DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "system_action_logs_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "system_users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "system_action_logs_menu_id_fkey" FOREIGN KEY ("menu_id") REFERENCES "system_menus" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);