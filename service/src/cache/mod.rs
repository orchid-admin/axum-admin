use crate::Result;
use serde::Serialize;
use utils::datetime::now_time;

mod driver;

pub enum CacheDriverType {
    Memory,
    Database(model::connect::DbConnectPool),
    // Redis,
    // Memcached,
    // DynamoDB,
    // File,
}

pub struct Cache<D: driver::Driver>(D);

#[allow(dead_code)]
impl<D> Cache<D>
where
    D: driver::Driver,
{
    pub fn new(driver: D) -> Self {
        Self(driver)
    }
}

#[async_trait::async_trait]
impl<D> driver::Driver for Cache<D>
where
    D: driver::Driver + std::marker::Sync + std::marker::Send,
{
    async fn put<T>(
        &mut self,
        r#type: i32,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        self.0
            .put(r#type, key, value, valid_time_length, attach)
            .await
    }

    async fn first(&self, r#type: i32, key: &str, default: Option<Info>) -> Result<Option<Info>> {
        self.0.first(r#type, key, default).await
    }

    async fn pull(&mut self, r#type: i32, key: &str) -> Result<Info> {
        self.0.pull(r#type, key).await
    }

    async fn flush(&mut self, r#type: Option<i32>) -> Result<i64> {
        self.0.flush(r#type).await
    }
}

#[allow(dead_code)]
impl<D> Cache<D>
where
    D: driver::Driver,
{
    pub async fn has(&self, r#type: i32, key: &str) -> Result<bool> {
        Ok(self.0.first(r#type.clone(), key, None).await?.is_some())
    }
    pub async fn get(&self, r#type: i32, key: &str, default: Option<Info>) -> Result<Info> {
        self.0
            .first(r#type, key, default)
            .await?
            .ok_or(super::ServiceError::CacheNotFound)
    }
    pub async fn add<T>(
        &mut self,
        r#type: i32,
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
                .put(r#type, key, value, valid_time_length, attach)
                .await;
        }
        self.get(r#type, key, None).await
    }
    /// Storing Items Forever
    pub async fn forever<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: i32,
        key: &str,
        value: T,
        attach: Option<String>,
    ) -> Result<Info> {
        self.0.put(r#type, key, value, None, attach).await
    }

    /// increment value
    pub async fn increment(&mut self, r#type: i32, key: &str, number: Option<f64>) -> Result<Info> {
        let number = number.unwrap_or(1f64);
        let info = self.get(r#type.clone(), key, None).await?;
        let value = info.value.parse::<f64>().unwrap();
        self.0
            .put(
                r#type,
                key,
                value + number,
                info.valid_time_length,
                info.attach,
            )
            .await
    }

    /// decrement value
    pub async fn decrement(&mut self, r#type: i32, key: &str, number: Option<f64>) -> Result<Info> {
        let number = number.unwrap_or(1f64);
        let info = self.get(r#type.clone(), key, None).await?;
        let value = info.value.parse::<f64>().unwrap();
        self.0
            .put(
                r#type,
                key,
                value - number,
                info.valid_time_length,
                info.attach,
            )
            .await
    }

    /// remember
    pub async fn remember<F>(
        &mut self,
        r#type: i32,
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
            return self
                .0
                .put(r#type, key, info, valid_time_length, attach)
                .await;
        }
        self.get(r#type, key, None).await
    }

    /// remember_forever
    pub async fn remember_forever<F>(
        &mut self,
        r#type: i32,
        key: &str,
        attach: Option<String>,
        r#fn: F,
    ) -> Result<Info>
    where
        F: Fn() -> Result<Info> + std::marker::Send,
    {
        if !self.has(r#type, key).await? {
            let info = r#fn()?;
            return self.0.put(r#type, key, info, None, attach).await;
        }
        self.get(r#type, key, None).await
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
