use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub person_name: String,
    pub person_phone: String,
    pub person_email: String,
    pub describe: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InfoForUserInfo {
    pub id: i32,
    pub name: String,
}

impl From<Info> for InfoForUserInfo {
    fn from(value: Info) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
