use getset::Getters;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Getters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    parent_id: i32,
    name: String,
    person_name: String,
    person_phone: String,
    person_email: String,
    describe: String,
    status: i32,
    sort: i32,
    created_at: String,
}
