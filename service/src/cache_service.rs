use crate::{Database, Result};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::{
    datetime::{self, now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

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
trait Driver {
    async fn put<T: Serialize + std::marker::Send>(
        &self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>;
    async fn forever<T: Serialize + std::marker::Send>(
        &self,
        r#type: CacheType,
        key: &str,
        value: T,
        attach: Option<String>,
    ) -> Result<Info>;

    async fn first(
        &self,
        r#type: CacheType,
        key: &str,
        default: Option<Info>,
    ) -> Result<Option<Info>>;
    async fn pull(&self, r#type: CacheType, key: &str) -> Result<Option<Info>>;
    async fn forget(&self, r#type: CacheType, key: &str) -> Result<Info>;
    async fn flush(&self, r#type: Option<CacheType>) -> Result<()>;
    // async fn has(&self, r#type: CacheType, key: &str) -> Result<bool>;
    // async fn add<T: Serialize + std::marker::Send>(
    //     &self,
    //     r#type: CacheType,
    //     key: &str,
    //     value: T,
    //     valid_time_length: Option<i64>,
    //     attach: Option<String>,
    // ) -> Result<Info>;
    // async fn increment(&self, r#type: CacheType, key: &str, number: Option<f64>) -> Result<bool>;
    // async fn decrement(&self, r#type: CacheType, key: &str, number: Option<f64>) -> Result<bool>;
    // async fn remember<F>(
    //     &self,
    //     r#type: CacheType,
    //     key: &str,
    //     valid_time_length: Option<i64>,
    //     attach: Option<String>,
    //     r#fn: F,
    // ) -> Result<Info>
    // where
    //     F: Fn() -> Result<Info> + std::marker::Send;
    // async fn remember_forever<F>(
    //     &self,
    //     r#type: CacheType,
    //     key: &str,
    //     attach: Option<String>,
    //     r#fn: F,
    // ) -> Result<Info>
    // where
    //     F: Fn() -> Result<Info> + std::marker::Send;
}
pub enum CacheDriverType {
    Memory,
    Database(Database),
    // Redis,
    // Memcached,
    // DynamoDB,
    // File,
}
struct Cache<D: Driver>(D);
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
        &self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send,
    {
        self.0
            .put(r#type, key, value, valid_time_length, attach)
            .await
    }

    async fn forever<T>(
        &self,
        r#type: CacheType,
        key: &str,
        value: T,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send,
    {
        self.0.forever(r#type, key, value, attach).await
    }

    async fn first(
        &self,
        r#type: CacheType,
        key: &str,
        default: Option<Info>,
    ) -> Result<Option<Info>> {
        self.0.first(r#type, key, default).await
    }

    async fn pull(&self, r#type: CacheType, key: &str) -> Result<Option<Info>> {
        self.0.pull(r#type, key).await
    }

    async fn forget(&self, r#type: CacheType, key: &str) -> Result<Info> {
        self.0.forget(r#type, key).await
    }

    async fn flush(&self, r#type: Option<CacheType>) -> Result<()> {
        self.0.flush(r#type).await
    }
}

impl<D> Cache<D>
where
    D: Driver,
{
    async fn has(&self, r#type: &CacheType, key: &str) -> Result<bool> {
        Ok(self.0.first(r#type.clone(), key, None).await?.is_some())
    }
    async fn get(&self, r#type: CacheType, key: &str, default: Option<Info>) -> Result<Info> {
        Ok(self.0.first(r#type, key, default).await?.unwrap())
    }
    async fn add<T>(
        &self,
        r#type: CacheType,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>
    where
        T: Serialize + std::marker::Send,
    {
        if !self.has(&r#type, key).await? {
            return self
                .0
                .put(r#type, key, value, valid_time_length, attach)
                .await;
        }
        self.get(r#type, key, None).await
    }
    async fn increment(&self, r#type: CacheType, key: &str, number: Option<f64>) -> Result<bool> {
        let number = match number {
            Some(n) => n,
            None => 1f64,
        };
        if self.has(&r#type, key).await? {
            let info = self.get(r#type.clone(), key, None).await?;
            let value = info.value.parse::<f64>().unwrap();
            self.0
                .put(
                    r#type,
                    key,
                    value + number,
                    info.valid_time
                        .map(|x| datetime::parse_string(x).timestamp_nanos()),
                    info.attach,
                )
                .await;
            return Ok(true);
        }
        Ok(false)
    }

    async fn decrement(&self, r#type: CacheType, key: &str, number: Option<f64>) -> Result<bool> {
        let number = match number {
            Some(n) => n,
            None => 1f64,
        };
        if self.has(&r#type, key).await? {
            let info = self.get(r#type.clone(), key, None).await?;
            let value = info.value.parse::<f64>().unwrap();
            self.0
                .put(
                    r#type,
                    key,
                    value - number,
                    info.valid_time
                        .map(|x| datetime::parse_string(x).timestamp_nanos()),
                    info.attach,
                )
                .await;
            return Ok(true);
        }
        Ok(false)
    }

    async fn remember<F>(
        &self,
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

    async fn remember_forever<F>(
        &self,
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
mod database {
    use crate::{generate_prisma::system_cache, prisma::SortOrder, Database, Result, ServiceError};
    use prisma_client_rust::or;
    pub async fn create(
        db: &Database,
        key: &str,
        code: &str,
        params: CreateParams,
    ) -> Result<Info> {
        Ok(db
            .client
            .system_cache()
            .create_unchecked(key.to_owned(), code.to_owned(), params.to_params())
            .exec()
            .await?
            .into())
    }

    pub async fn update(db: &Database, id: i32, params: UpdateParams) -> Result<Info> {
        Ok(db
            .client
            .system_cache()
            .update_unchecked(system_cache::id::equals(id), params.to_params())
            .exec()
            .await?
            .into())
    }
    pub async fn delete(db: &Database, id: i32) -> Result<Info> {
        Ok(db
            .client
            .system_cache()
            .update(
                system_cache::id::equals(id),
                vec![system_cache::deleted_at::set(Some(now_time()))],
            )
            .exec()
            .await?
            .into())
    }
    pub async fn batch_delete(db: &Database, ids: Vec<i32>) -> Result<i64> {
        Ok(db
            .client
            .system_cache()
            .update_many(
                vec![system_cache::id::in_vec(ids)],
                vec![system_cache::deleted_at::set(Some(now_time()))],
            )
            .exec()
            .await?)
    }
    pub async fn info(db: &Database, id: i32) -> Result<Info> {
        Ok(db
            .client
            .system_cache()
            .find_unique(system_cache::id::equals(id))
            .exec()
            .await?
            .ok_or(ServiceError::DataNotFound)?
            .into())
    }
    pub async fn get_by_type_key(
        db: &Database,
        r#type: &super::CacheType,
        key: &str,
        filter_id: Option<i32>,
    ) -> Result<Option<super::Info>> {
        let mut params = vec![
            system_cache::r#type::equals(r#type.clone().into()),
            system_cache::key::equals(key.to_owned()),
            system_cache::deleted_at::equals(None),
        ];
        if let Some(id) = filter_id {
            params.push(system_cache::id::not(id));
        }
        Ok(db
            .client
            .system_cache()
            .find_first(params)
            .exec()
            .await?
            .map(|x| x.into()))
    }
    pub async fn paginate(
        db: &Database,
        params: &SearchParams,
    ) -> Result<PaginateResult<Vec<Info>>> {
        let (data, total) = db
            .client
            ._batch((
                db.client
                    .system_cache()
                    .find_many(params.to_params())
                    .skip(params.paginate.get_skip())
                    .take(params.paginate.get_limit())
                    .order_by(system_cache::id::order(SortOrder::Desc)),
                db.client.system_cache().count(params.to_params()),
            ))
            .await?;
        Ok(PaginateResult {
            total,
            data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
        })
    }

    impl From<system_cache::Data> for Info {
        fn from(value: system_cache::Data) -> Self {
            Self {
                id: value.id,
                r#type: value.r#type.into(),
                code: value.code,
                attach: value.attach,
                valid_time: to_local_string(value.valid_time),
                created_at: to_local_string(value.created_at),
            }
        }
    }

    system_cache::partial_unchecked!(CreateParams {
        r#type
        attach
        valid_time
    });
    system_cache::partial_unchecked!(UpdateParams {
        r#type
        attach
        valid_time
    });
}

// pub struct SearchParams {
//     keyword: Option<String>,
//     r#type: Option<CacheType>,
//     paginate: PaginateParams,
// }
// impl SearchParams {
//     fn to_params(&self) -> Vec<system_cache::WhereParam> {
//         let mut params = vec![system_cache::deleted_at::equals(None)];
//         if let Some(keyword) = &self.keyword {
//             params.push(or!(system_cache::code::contains(keyword.to_string()),));
//         }
//         if let Some(t) = &self.r#type {
//             params.push(system_cache::r#type::equals(t.clone().into()));
//         }
//         params
//     }

//     pub fn new(
//         keyword: Option<String>,
//         r#type: Option<CacheType>,
//         paginate: PaginateParams,
//     ) -> Self {
//         Self {
//             keyword,
//             r#type,
//             paginate,
//         }
//     }
// }

#[derive(Debug, Serialize)]
pub struct Info {
    key: String,
    r#type: CacheType,
    value: String,
    attach: Option<String>,
    valid_time: Option<String>,
    create_time: String,
}
