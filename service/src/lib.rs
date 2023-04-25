use prisma_client_rust::chrono::{DateTime, FixedOffset, Utc};

#[allow(unused, warnings)]
mod prisma;
pub mod sys_menu;
pub mod sys_role;
mod sys_role_menu;
pub mod sys_user;

pub type Database = std::sync::Arc<prisma::PrismaClient>;
pub type Result<T> = std::result::Result<T, ServiceError>;

pub async fn new_client() -> Result<Database> {
    let database = std::sync::Arc::new(prisma::PrismaClient::_builder().build().await?);
    let role = sys_role::upsert(&database, "超级管理员", "admin", vec![]).await?;
    sys_user::upset(
        &database,
        "admin",
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
fn now_time() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}

#[derive(Debug, custom_attrs::CustomAttrs)]
#[attr(pub code: &str)]
pub enum ServiceError {
    #[attr(code = "BuildClient")]
    BuildClient(String),
    #[attr(code = "QueryError")]
    QueryError(String),
    #[attr(code = "RelationNotFetchedError")]
    RelationNotFetchedError(String),
    #[attr(code = "DataNotFound")]
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
