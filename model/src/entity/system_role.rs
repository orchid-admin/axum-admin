use getset::Getters;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Getters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    name: String,
    #[getset(get = "pub")]
    sign: String,
    describe: String,
    status: i32,
    sort: i32,
    created_at: String,
    menu_ids: Vec<i32>,
}
