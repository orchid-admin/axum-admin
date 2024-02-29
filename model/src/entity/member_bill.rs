use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entity {
    id: i32,
    user_id: i32,
    r#type: i32,
    pm: i32,
    number: f32,
    created_at: String,
}
