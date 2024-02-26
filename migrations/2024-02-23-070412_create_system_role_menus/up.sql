-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_role_menus (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role_id INTEGER NOT NULL,
    menu_id INTEGER NOT NULL,
    deleted_at DATETIME,
    CONSTRAINT "system_role_menus_role_id_fkey" FOREIGN KEY ("role_id") REFERENCES "system_roles" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "system_role_menus_menu_id_fkey" FOREIGN KEY ("menu_id") REFERENCES "system_menus" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE UNIQUE INDEX "system_role_menus_role_id_menu_id_key" ON "system_role_menus"("role_id", "menu_id");