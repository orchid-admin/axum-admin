use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entity {
    id: i32,
    dict_id: i32,
    label: String,
    value: i32,
    status: i32,
    sort: i32,
    remark: String,
    created_at: String,
}
