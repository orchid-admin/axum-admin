use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entity {
    id: i32,
    name: String,
    sign: String,
    status: i32,
    remark: String,
    created_at: String,
}
