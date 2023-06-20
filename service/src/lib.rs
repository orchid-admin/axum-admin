#[allow(unused, warnings)]
mod generate_prisma;

pub mod member_bill_service;
pub mod member_service;
pub mod member_team_service;
pub mod system_action_log_service;
pub mod system_dept_service;
pub mod system_dict_data_service;
pub mod system_dict_service;
pub mod system_login_log_server;
pub mod system_menu_service;
pub mod system_role_menu_service;
pub mod system_role_service;
pub mod system_user_service;

use generate_prisma as prisma;
pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    BuildClient(String),
    QueryError(String),
    RelationNotFetchedError(String),
    DataNotFound,
}

impl From<prisma_client_rust::NewClientError> for ServiceError {
    fn from(value: prisma_client_rust::NewClientError) -> Self {
        Self::BuildClient(value.to_string())
    }
}

impl From<prisma_client_rust::QueryError> for ServiceError {
    fn from(value: prisma_client_rust::QueryError) -> Self {
        Self::QueryError(value.to_string())
    }
}

impl From<prisma_client_rust::RelationNotFetchedError> for ServiceError {
    fn from(value: prisma_client_rust::RelationNotFetchedError) -> Self {
        Self::RelationNotFetchedError(value.to_string())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DataPower<T: serde::Serialize> {
    _can_edit: bool,
    _can_delete: bool,
    #[serde(flatten)]
    data: T,
}

/// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    database_url: Option<String>,
    admin_username: String,
    admin_role_sign: String,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: Default::default(),
            admin_username: "admin".to_owned(),
            admin_role_sign: "admin".to_owned(),
        }
    }
}
impl DatabaseConfig {
    pub fn get_admin_username(&self) -> String {
        self.admin_username.clone()
    }
    pub fn get_admin_role_sign(&self) -> String {
        self.admin_role_sign.clone()
    }
}
/// 外部使用的数据库链接结构
pub struct Database {
    config: DatabaseConfig,
    client: prisma::PrismaClient,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let client = match config.database_url.clone() {
            Some(database_url) => {
                prisma::PrismaClient::_builder()
                    .with_url(database_url)
                    .build()
                    .await?
            }
            None => prisma::PrismaClient::_builder().build().await?,
        };
        Ok(Self { config, client })
    }

    pub fn config(&self) -> DatabaseConfig {
        self.config.clone()
    }
}
