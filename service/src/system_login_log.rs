use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_login_log};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(
    pool: &ConnectPool,
    params: system_login_log::FormParamsForCreate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_login_log::Entity::create(&mut conn, &params).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    system_login_log::Entity::find(
        &mut conn,
        system_login_log::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn paginate(pool: &ConnectPool, filter: Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = system_login_log::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}
pub type Info = system_login_log::Entity;
pub type FormParamsForCreate = system_login_log::FormParamsForCreate;

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub user_id: Option<i32>,
    pub keyword: Option<String>,
    pub date: Option<String>,
    pub paginate: PaginateParams,
}

impl From<Filter> for system_login_log::Filter {
    fn from(value: Filter) -> Self {
        Self {
            keyword: value.keyword,
            date: value.date,
            user_id: value.user_id,
            ..Default::default()
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LoginType {
    Account = 1,
    Mobile = 2,
    QrCode = 3,
}
impl From<i32> for LoginType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Account,
            2 => Self::Mobile,
            3 => Self::QrCode,
            _ => Self::Account,
        }
    }
}
impl From<LoginType> for i32 {
    fn from(value: LoginType) -> Self {
        match value {
            LoginType::Account => 1,
            LoginType::Mobile => 2,
            LoginType::QrCode => 3,
        }
    }
}
