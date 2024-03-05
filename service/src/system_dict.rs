use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_dict};
use serde::Deserialize;
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(pool: &ConnectPool, params: system_dict::FormParamsForCreate) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dict::Entity::create(&mut conn, &params).await?)
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_dict::FormParamsForUpdate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dict::Entity::update(&mut conn, id, params).await?)
}
pub async fn delete(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dict::Entity::soft_delete(&mut conn, id).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    system_dict::Entity::find(
        &mut conn,
        &system_dict::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn get_by_sign(
    pool: &ConnectPool,
    sign: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut filter = system_dict::Filter {
        sign: Some(sign.to_owned()),
        ..Default::default()
    };
    if let Some(id) = filter_id {
        filter.id = Some(id);
    }
    let mut conn = pool.conn().await?;
    Ok(system_dict::Entity::find(&mut conn, &filter).await?)
}
pub async fn all(pool: &ConnectPool) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;
    Ok(system_dict::Entity::query(
        &mut conn,
        &system_dict::Filter {
            ..Default::default()
        },
    )
    .await?)
}

pub async fn paginate(pool: &ConnectPool, filter: &Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = system_dict::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        &filter.filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}
pub type Info = system_dict::Entity;
#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(flatten)]
    filter: system_dict::Filter,
    #[serde(flatten)]
    paginate: PaginateParams,
}
