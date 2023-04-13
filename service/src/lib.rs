mod prisma;
pub mod sys_user;

pub type Database = std::sync::Arc<prisma::PrismaClient>;
pub type Result<T> = std::result::Result<T, ServiceError>;

pub async fn new_client() -> Result<Database> {
    let database = std::sync::Arc::new(prisma::PrismaClient::_builder().build().await?);
    if let Err(e) = sys_user::create_user(database.clone(), "admin", "123456", "salt").await {
        println!("{:#?}", e);
    }
    Ok(database)
}

#[derive(Debug, custom_attrs::CustomAttrs)]
#[attr(pub code: &str)]
pub enum ServiceError {
    #[attr(code = "BuildClient")]
    BuildClient(String),
    #[attr(code = "QueryError")]
    QueryError(String),
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
