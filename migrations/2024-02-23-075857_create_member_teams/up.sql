-- Your SQL goes here
CREATE TABLE IF NOT EXISTS member_teams (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    owner_uid INTEGER NOT NULL,
    parent_uid INTEGER NOT NULL,
    member_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME,
    CONSTRAINT "member_teams_owner_uid_fkey" FOREIGN KEY ("owner_uid") REFERENCES "members" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "member_teams_parent_uid_fkey" FOREIGN KEY ("parent_uid") REFERENCES "members" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "member_teams_member_id_fkey" FOREIGN KEY ("member_id") REFERENCES "members" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);