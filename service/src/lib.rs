use prisma_client_rust::chrono::{DateTime, FixedOffset, Utc};

#[allow(
    unused,
    clippy::from_over_into,
    clippy::self_named_constructors,
    clippy::redundant_closure,
    clippy::wrong_self_convention,
    clippy::enum_variant_names,
    clippy::useless_conversion,
    clippy::crate_in_macro_def,
    clippy::redundant_static_lifetimes
)]
mod prisma;
pub mod sys_menu;
mod sys_role_menu;
pub mod sys_user;

pub type Database = std::sync::Arc<prisma::PrismaClient>;
pub type Result<T> = std::result::Result<T, ServiceError>;

pub async fn new_client() -> Result<Database> {
    let database = std::sync::Arc::new(prisma::PrismaClient::_builder().build().await?);
    if let Err(e) = sys_user::upset(
        database.clone(),
        "admin",
        "sfWTwt9NxLNapTmoIdzfUbbRODMk266kc7ArZcF2EsQ",
        "nodiZ0cU0ER5Vg3n+rOsoQ",
    )
    .await
    {
        println!("{:#?}", e);
    }
    Ok(database)
}

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
