use prisma_client_rust::chrono::{DateTime, FixedOffset, Utc};
use serde_with::{serde_as, DisplayFromStr};

#[allow(unused, warnings)]
mod prisma;
pub mod sys_menu;
pub mod sys_role;
#[allow(unused)]
mod sys_role_menu;
pub mod sys_user;

pub const ADMIN_USERNAME: &str = "admin";
pub const ADMIN_ROLE_SIGN: &str = "admin";

pub type Database = std::sync::Arc<prisma::PrismaClient>;
pub type Result<T> = std::result::Result<T, ServiceError>;

pub async fn new_client() -> Result<Database> {
    let database = std::sync::Arc::new(prisma::PrismaClient::_builder().build().await?);
    let role = sys_role::upsert(&database, "超级管理员", ADMIN_ROLE_SIGN, vec![]).await?;
    sys_user::upset(
        &database,
        ADMIN_USERNAME,
        sys_user::UserCreateParams {
            password: Some("sfWTwt9NxLNapTmoIdzfUbbRODMk266kc7ArZcF2EsQ".to_owned()),
            salt: Some("nodiZ0cU0ER5Vg3n+rOsoQ".to_owned()),
            role_id: Some(Some(role.id)),
        }
        .to_params(),
    )
    .await?;
    Ok(database)
}

#[allow(dead_code)]
pub fn now_time() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}
#[allow(dead_code)]
fn to_local_string(datetime: DateTime<FixedOffset>) -> String {
    datetime
        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

#[derive(Debug)]
pub enum ServiceError {
    BuildClient(String),
    QueryError(String),
    RelationNotFetchedError(String),
    DataNotFound,
}

#[serde_as]
#[derive(Debug, serde::Deserialize)]
pub struct PaginateRequest {
    #[serde_as(as = "DisplayFromStr")]
    page: i64,
    #[serde_as(as = "DisplayFromStr")]
    limit: i64,
}

impl PaginateRequest {
    fn get_skip(&self) -> i64 {
        match self.page > 0 {
            true => (self.page - 1) * self.limit,
            false => self.limit,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PaginateResponse<T: serde::Serialize> {
    total: i64,
    data: T,
}

#[derive(Debug, serde::Serialize)]
pub struct DataPower<T: serde::Serialize> {
    _can_edit: bool,
    _can_delete: bool,
    #[serde(flatten)]
    data: T,
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
