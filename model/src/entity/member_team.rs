use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entity {
    id: i32,
    owner_uid: i32,
    parent_uid: i32,
    uid: i32,
    level: i32,
    created_at: String,
}
