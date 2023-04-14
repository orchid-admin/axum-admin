mod prisma;
pub mod system_user;

pub type Database = std::sync::Arc<prisma::PrismaClient>;
pub type Result<T> = std::result::Result<T, ServiceError>;

pub async fn new_client() -> Result<Database> {
    Ok(std::sync::Arc::new(
        prisma::PrismaClient::_builder().build().await?,
    ))
}

#[derive(Debug, custom_attrs::CustomAttrs)]
#[attr(pub code: &str)]
pub enum ServiceError {
    #[attr(code = "BuildClient")]
    BuildClient,
    #[attr(code = "QueryError")]
    QueryError,
}

impl From<prisma_client_rust::NewClientError> for ServiceError {
    fn from(_value: prisma_client_rust::NewClientError) -> Self {
        Self::BuildClient
    }
}

impl From<prisma_client_rust::QueryError> for ServiceError {
    fn from(_value: prisma_client_rust::QueryError) -> Self {
        Self::QueryError
    }
}
