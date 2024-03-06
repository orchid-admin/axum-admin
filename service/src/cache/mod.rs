use crate::Result;
use serde::Serialize;

use utils::datetime::now_time;

mod driver;
mod types;
pub use driver::{database::Database as DatabaseDriver, memory::Memroy as MemoryDriver};
pub use types::Types as CacheType;

pub enum CacheDriver {
    Memory,
    Database,
    // Redis,
    // Memcached,
    // DynamoDB,
    // File,
}

impl CacheDriver {
    pub fn new_memory() -> Cache<driver::memory::Memroy> {
        Cache::new(driver::memory::Memroy::default())
    }

    pub fn new_database(
        connect_pool: model::connect::DbConnectPool,
    ) -> Cache<driver::database::Database> {
        Cache::new(driver::database::Database(connect_pool))
    }
}

pub struct Cache<D>(D)
where
    D: driver::Driver;

#[allow(dead_code)]
impl<D> Cache<D>
where
    D: driver::Driver,
{
    pub fn new(driver: D) -> Self {
        Self(driver)
    }

    pub async fn has<P: Into<i32>>(&self, r#type: P, key: &str) -> Result<bool> {
        Ok(self.0.first(r#type.into(), key, None).await?.is_some())
    }
    pub async fn get<P: Into<i32>>(
        &self,
        r#type: P,
        key: &str,
        default: Option<Info>,
    ) -> Result<Info> {
        self.0
            .first(r#type.into(), key, default)
            .await?
            .ok_or(super::ServiceError::CacheNotFound)
    }
    pub async fn put<T, P: Into<i32>>(
        &mut self,
        r#type: P,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        self.0
            .put(r#type.into(), key, value, valid_time_length, attach)
            .await
    }
    pub async fn add<T, P: Into<i32> + Copy>(
        &mut self,
        r#type: P,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        if !self.has(r#type, key).await? {
            return self
                .0
                .put(r#type.into(), key, value, valid_time_length, attach)
                .await;
        }
        self.get(r#type, key, None).await
    }
    /// Storing Items Forever
    pub async fn forever<T, P: Into<i32>>(
        &mut self,
        r#type: P,
        key: &str,
        value: T,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        self.put(r#type, key, value, None, attach).await
    }

    /// increment value
    pub async fn increment<P: Into<i32> + Copy>(
        &mut self,
        r#type: P,
        key: &str,
        number: Option<f64>,
    ) -> Result<Info> {
        let number = number.unwrap_or(1f64);
        let info = self.get(r#type, key, None).await?;
        let value = info.value.parse::<f64>().unwrap();
        self.put(
            r#type,
            key,
            value + number,
            info.valid_time_length,
            info.attach,
        )
        .await
    }

    /// decrement value
    pub async fn decrement<P: Into<i32> + Copy>(
        &mut self,
        r#type: P,
        key: &str,
        number: Option<f64>,
    ) -> Result<Info> {
        let number = number.unwrap_or(1f64);
        let info = self.get(r#type, key, None).await?;
        let value = info.value.parse::<f64>().unwrap();
        self.put(
            r#type,
            key,
            value - number,
            info.valid_time_length,
            info.attach,
        )
        .await
    }

    /// remember
    pub async fn remember<F, P: Into<i32> + Copy>(
        &mut self,
        r#type: P,
        key: &str,
        valid_time_length: Option<i64>,
        attach: Option<String>,
        r#fn: F,
    ) -> Result<Info>
    where
        F: FnOnce() -> Result<Info> + std::marker::Send,
    {
        if !self.has(r#type, key).await? {
            let info = r#fn()?;
            return self.put(r#type, key, info, valid_time_length, attach).await;
        }
        self.get(r#type, key, None).await
    }

    /// remember_forever
    pub async fn remember_forever<F, P: Into<i32> + Copy>(
        &mut self,
        r#type: P,
        key: &str,
        attach: Option<String>,
        r#fn: F,
    ) -> Result<Info>
    where
        F: Fn() -> Result<Info> + std::marker::Send,
    {
        if !self.has(r#type, key).await? {
            let info = r#fn()?;
            return self.put(r#type, key, info, None, attach).await;
        }
        self.get(r#type, key, None).await
    }

    pub async fn pull<P: Into<i32>>(&mut self, r#type: P, key: &str) -> Result<Info> {
        self.0.pull(r#type.into(), key).await
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Info {
    key: String,
    r#type: i32,
    value: String,
    attach: Option<String>,
    valid_time_length: Option<i64>,
    create_time: i64,
}

impl Info {
    pub fn is_valid(self) -> bool {
        if let Some(valid_time_length) = self.valid_time_length {
            return (self.create_time + valid_time_length) > now_time().timestamp();
        }
        false
    }

    pub fn get_valid_timestamp(self) -> Option<i64> {
        self.valid_time_length.map(|x| self.create_time + x)
    }

    pub fn value<T: serde::de::DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_str::<T>(&self.value)
    }

    pub fn get_value<T: serde::de::DeserializeOwned>(self) -> T {
        serde_json::from_str(self.value.as_str()).unwrap()
    }
}
