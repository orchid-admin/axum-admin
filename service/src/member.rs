use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, member};
use serde::Deserialize;
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(pool: &ConnectPool, params: &mut member::FormParamsForCreate) -> Result<Info> {
    let mut conn = pool.conn().await?;
    params.unique_code = generate_code(pool, 8).await?;
    Ok(member::Entity::create(&mut conn, &params).await?)
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: member::FormParamsForUpdate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(member::Entity::update(&mut conn, id, params).await?)
}
pub async fn delete(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(member::Entity::soft_delete(&mut conn, id).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    member::Entity::find(
        &mut conn,
        member::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn get_by_unique_code(
    pool: &ConnectPool,
    unique_code: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut conn = pool.conn().await?;
    Ok(member::Entity::get_by_unique_code(&mut conn, unique_code, filter_id).await?)
}
pub async fn get_by_email(
    pool: &ConnectPool,
    email: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut conn = pool.conn().await?;
    Ok(member::Entity::get_by_email(&mut conn, email, filter_id).await?)
}
pub async fn all(pool: &ConnectPool) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;
    Ok(member::Entity::query(&mut conn, member::Filter::default()).await?)
}

pub async fn paginate(pool: &ConnectPool, filter: Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = member::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}

pub async fn increment(
    pool: &ConnectPool,
    user_id: i32,
    balance: Option<f64>,
    integral: Option<i32>,
) -> Result<Info> {
    let mut conn = pool.conn().await?;

    member::Entity::increment_transaction(&mut conn, user_id, balance, integral)
        .await?
        .ok_or(ServiceError::DataNotFound)
}

pub async fn decrement(
    pool: &ConnectPool,
    user_id: i32,
    balance: Option<f64>,
    integral: Option<i32>,
) -> Result<Info> {
    let mut conn = pool.conn().await?;

    member::Entity::decrement_transaction(&mut conn, user_id, balance, integral)
        .await?
        .ok_or(ServiceError::DataNotFound)
}

#[async_recursion::async_recursion]
pub async fn generate_code(pool: &ConnectPool, code_length: usize) -> Result<String> {
    let unique_code: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(code_length)
        .collect();
    match get_by_unique_code(pool, &unique_code, None).await? {
        Some(_) => generate_code(pool, code_length).await,
        None => Ok(unique_code),
    }
}

pub async fn set_last_login(pool: &ConnectPool, id: &i32, login_ip: &str) -> Result<Info> {
    let mut conn = pool.conn().await?;
    let mut info = member::Entity::find(
        &mut conn,
        member::Filter {
            id: Some(id.to_owned()),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?;

    Ok(member::Entity::set_last_login(&mut conn, &mut info, login_ip).await?)
}
pub type Info = member::Entity;
pub type FormParamsForCreate = member::FormParamsForCreate;

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub sex: Option<i32>,
    pub status: Option<i32>,
    pub is_promoter: Option<i32>,
    #[serde(flatten)]
    pub paginate: PaginateParams,
}

impl From<Filter> for member::Filter {
    fn from(value: Filter) -> Self {
        Self {
            keyword: value.keyword,
            status: value.status,
            ..Default::default()
        }
    }
}
