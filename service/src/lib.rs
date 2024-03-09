pub mod cache;
pub mod member;
pub mod member_bill;
pub mod member_team;
pub mod system_action_log;
pub mod system_dept;
pub mod system_dict;
pub mod system_dict_data;
pub mod system_login_log;
pub mod system_menu;
pub mod system_role;
pub mod system_user;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    BuildClient(String),
    QueryError(String),
    RelationNotFetchedError(String),
    DataNotFound,
    SerializeJson(serde_json::Error),
    CacheNotFound,
    Model(model::Error),
}

impl From<serde_json::Error> for ServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeJson(value)
    }
}
impl From<model::Error> for ServiceError {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}
