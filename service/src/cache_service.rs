use crate::{generate_prisma::system_cache, Database, Result};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::datetime::{now_time, offset_from_timestamp, timestamp_nanos};

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CacheType {
    SystemAuthJwt = 0,
    SystemAuthLoginCaptcha = 1,
    SystemAuthLoginMobile = 2,
    SystemAuthLoginQrCode = 3,
    MemberAuthRegisterEmail = 4,
    MemberAuthLoginEmail = 5,
}
impl From<i32> for CacheType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::SystemAuthJwt,
            1 => Self::SystemAuthLoginCaptcha,
            2 => Self::SystemAuthLoginMobile,
            3 => Self::MemberAuthRegisterEmail,
            4 => Self::SystemAuthLoginQrCode,
            5 => Self::MemberAuthLoginEmail,
            _ => Self::SystemAuthJwt,
        }
    }
}
impl From<CacheType> for i32 {
    fn from(value: CacheType) -> Self {
        match value {
            CacheType::SystemAuthJwt => 0,
            CacheType::SystemAuthLoginCaptcha => 1,
            CacheType::SystemAuthLoginMobile => 2,
            CacheType::MemberAuthRegisterEmail => 3,
            CacheType::SystemAuthLoginQrCode => 4,
            CacheType::MemberAuthLoginEmail => 5,
        }
    }
}

#[async_trait::async_trait]
pub trait Driver {
    /// Storing Items In The Cache
    async fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>;

    /// Retrieving Items From The Cache
    async fn first(
        &self,
        r#type: CacheType,
        key: &str,
        default: Option<Info>,
    ) -> Result<Option<Info>>;

    /// Retrieve & Delete
    async fn pull(&mut self, r#type: CacheType, key: &str) -> Result<Info>;
    /// clear the entire cache
    async fn flush(&mut self, r#type: Option<CacheType>) -> Result<i64>;
}
pub enum CacheDriverType {
    Memory,
    Database(Database),
    // Redis,
    // Memcached,
    // DynamoDB,
    // File,
}

pub struct Cache<D: Driver>(D);

#[allow(dead_code)]
impl<D> Cache<D>
where
    D: Driver,
{
    pub fn new(driver: D) -> Self {
        Self(driver)
    }
}

#[async_trait::async_trait]
impl<D> Driver for Cache<D>
where
    D: Driver + std::marker::Sync + std::marker::Send,
{
    async fn put<T>(
        &mut self,
        r#type: CacheType,
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

    async fn first(
        &self,
        r#type: CacheType,
        key: &str,
        default: Option<Info>,
    ) -> Result<Option<Info>> {
        self.0.first(r#type, key, default).await
    }

    async fn pull(&mut self, r#type: CacheType, key: &str) -> Result<Info> {
        self.0.pull(r#type, key).await
    }

    async fn flush(&mut self, r#type: Option<CacheType>) -> Result<i64> {
        self.0.flush(r#type).await
    }
}

#[allow(dead_code)]
impl<D> Cache<D>
where
    D: Driver,
{
    pub async fn has(&self, r#type: &CacheType, key: &str) -> Result<bool> {
        Ok(self.0.first(r#type.clone(), key, None).await?.is_some())
    }
    pub async fn get(&self, r#type: CacheType, key: &str, default: Option<Info>) -> Result<Info> {
        Ok(self.0.first(r#type, key, default).await?.unwrap())
    }
    pub async fn add<T>(
        &mut self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send + std::marker::Sync,
    {
        if !self.has(&r#type, key).await? {
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
        r#type: CacheType,
        key: &str,
        value: T,
        attach: Option<String>,
    ) -> Result<Info> {
        self.0.put(r#type, key, value, None, attach).await
    }

    /// increment value
    pub async fn increment(
        &mut self,
        r#type: CacheType,
        key: &str,
        number: Option<f64>,
    ) -> Result<Info> {
        let number = number.unwrap_or(1f64);
        let info = self.get(r#type.clone(), key, None).await?;
        let value = info.value.parse::<f64>().unwrap();
        self.0
            .put(
                r#type,
                key,
                value + number,
                info.valid_time
                    .map(|x| now_time().timestamp() - x.timestamp()),
                info.attach,
            )
            .await
    }

    /// decrement value
    pub async fn decrement(
        &mut self,
        r#type: CacheType,
        key: &str,
        number: Option<f64>,
    ) -> Result<Info> {
        let number = number.unwrap_or(1f64);
        let info = self.get(r#type.clone(), key, None).await?;
        let value = info.value.parse::<f64>().unwrap();
        self.0
            .put(
                r#type,
                key,
                value - number,
                info.valid_time
                    .map(|x| now_time().timestamp() - x.timestamp()),
                info.attach,
            )
            .await
    }

    /// remember
    pub async fn remember<F>(
        &mut self,
        r#type: CacheType,
        key: &str,
        valid_time_length: Option<i64>,
        attach: Option<String>,
        r#fn: F,
    ) -> Result<Info>
    where
        F: FnOnce() -> Result<Info> + std::marker::Send,
    {
        if !self.has(&r#type, key).await? {
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
        r#type: CacheType,
        key: &str,
        attach: Option<String>,
        r#fn: F,
    ) -> Result<Info>
    where
        F: Fn() -> Result<Info> + std::marker::Send,
    {
        if !self.has(&r#type, key).await? {
            let info = r#fn()?;
            return self.0.put(r#type, key, info, None, attach).await;
        }
        self.get(r#type, key, None).await
    }
}

#[derive(Default)]
pub struct CacheDriverMemory {
    data: Vec<Info>,
}

#[async_trait::async_trait]
impl Driver for CacheDriverMemory {
    async fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info> {
        let info = Info {
            r#type,
            key: key.to_owned(),
            value: serde_json::to_string(&value).unwrap(),
            valid_time: valid_time_length
                .map(|_: i64| offset_from_timestamp(timestamp_nanos(valid_time_length))),
            attach,
            create_time: now_time(),
        };
        self.data.push(info.clone());
        Ok(info)
    }

    async fn first(
        &self,
        r#type: CacheType,
        key: &str,
        default: Option<Info>,
    ) -> Result<Option<Info>> {
        let info = self
            .data
            .clone()
            .into_iter()
            .find(|x| x.r#type.eq(&r#type) && x.key.eq(key));
        if info.is_none() {
            return Ok(default);
        }
        Ok(info)
    }
    async fn pull(&mut self, r#type: CacheType, key: &str) -> Result<Info> {
        let info = self.first(r#type.clone(), key, None).await?.unwrap();
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.r#type.ne(&r#type) && x.key.ne(key))
            .collect::<Vec<Info>>();
        Ok(info)
    }

    async fn flush(&mut self, r#type: Option<CacheType>) -> Result<i64> {
        if let Some(cache_type) = r#type {
            let count = self
                .data
                .clone()
                .into_iter()
                .filter(|x| x.r#type.eq(&cache_type))
                .count() as i64;
            self.data = self
                .data
                .clone()
                .into_iter()
                .filter(|x| x.r#type.ne(&cache_type))
                .collect::<Vec<Info>>();
            return Ok(count);
        }
        let count = self.data.len() as i64;
        self.data = vec![];
        Ok(count)
    }
}

pub struct CacheDriverDatabase(Database);

#[async_trait::async_trait]
impl Driver for CacheDriverDatabase {
    async fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info> {
        Ok(self
            .0
            .client
            .system_cache()
            .create_unchecked(
                key.to_owned(),
                r#type.into(),
                serde_json::to_string(&value)?,
                CreateParams {
                    attach,
                    valid_time: Some(
                        valid_time_length.map(|_: i64| {
                            offset_from_timestamp(timestamp_nanos(valid_time_length))
                        }),
                    ),
                }
                .to_params(),
            )
            .exec()
            .await?
            .into())
    }

    async fn first(
        &self,
        r#type: CacheType,
        key: &str,
        default: Option<Info>,
    ) -> Result<Option<Info>> {
        let info = self
            .0
            .client
            .system_cache()
            .find_first(vec![
                system_cache::r#type::equals(r#type.into()),
                system_cache::key::equals(key.to_owned()),
                system_cache::deleted_at::equals(None),
            ])
            .exec()
            .await?
            .map(|x| x.into());
        if info.is_none() {
            return Ok(default);
        }
        Ok(info)
    }
    async fn pull(&mut self, r#type: CacheType, key: &str) -> Result<Info> {
        Ok(self
            .0
            .client
            .system_cache()
            .update(
                system_cache::key_type(key.to_owned(), r#type.into()),
                vec![system_cache::deleted_at::set(Some(now_time()))],
            )
            .exec()
            .await?
            .into())
    }

    async fn flush(&mut self, r#type: Option<CacheType>) -> Result<i64> {
        let mut params = vec![];
        if let Some(cache_type) = r#type {
            params.push(system_cache::r#type::equals(cache_type.into()))
        }
        Ok(self
            .0
            .client
            .system_cache()
            .update_many(
                params,
                vec![system_cache::deleted_at::set(Some(now_time()))],
            )
            .exec()
            .await?)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Info {
    key: String,
    r#type: CacheType,
    value: String,
    attach: Option<String>,
    valid_time:
        Option<prisma_client_rust::chrono::DateTime<prisma_client_rust::chrono::FixedOffset>>,
    create_time: prisma_client_rust::chrono::DateTime<prisma_client_rust::chrono::FixedOffset>,
}

impl Info {
    pub fn get_value(self) -> String {
        self.value
    }

    pub fn is_valid(self) -> bool {
        if let Some(valid_time) = self.valid_time {
            return valid_time.timestamp_nanos() > now_time().timestamp_nanos();
        }
        false
    }
}
impl From<system_cache::Data> for Info {
    fn from(value: system_cache::Data) -> Self {
        Self {
            key: value.key,
            r#type: value.r#type.into(),
            value: value.value,
            attach: match value.attach.is_empty() {
                true => None,
                false => Some(value.attach),
            },
            valid_time: value.valid_time,
            create_time: value.created_at,
        }
    }
}

system_cache::partial_unchecked!(CreateParams {
    attach
    valid_time
});
system_cache::partial_unchecked!(UpdateParams {
    r#type
    attach
    valid_time
});
