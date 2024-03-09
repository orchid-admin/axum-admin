use crate::error::Error;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

pub type Connect = Object<AsyncPgConnection>;
#[derive(Clone)]
pub struct ConnectPool(Pool<AsyncPgConnection>);

impl ConnectPool {
    pub fn new(database_url: &str) -> Result<Self, Error> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build()?;
        Ok(Self(pool))
    }

    pub async fn conn(&self) -> Result<Connect, Error> {
        Ok(self.0.get().await?)
    }
}
