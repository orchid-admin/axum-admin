// pub mod cache_service;
// pub mod member_bill_service;
// pub mod member_service;
// pub mod member_team_service;
// pub mod system_action_log_service;
// pub mod system_dept_service;
// pub mod system_dict_data_service;
// pub mod system_dict_service;
// pub mod system_login_log_server;
// pub mod system_menu_service;
// pub mod system_role_menu_service;
// pub mod system_role_service;
// pub mod system_user_service;
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
#[derive(Debug, serde::Serialize)]
pub struct DataPower<T: serde::Serialize> {
    _can_edit: bool,
    _can_delete: bool,
    #[serde(flatten)]
    data: T,
}
