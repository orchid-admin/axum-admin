use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_dict_data};
use serde::Deserialize;
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(
    pool: &ConnectPool,
    params: system_dict_data::FormParamsForCreate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dict_data::Entity::create(&mut conn, &params).await?)
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_dict_data::FormParamsForUpdate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dict_data::Entity::update(&mut conn, id, params).await?)
}
pub async fn delete(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_dict_data::Entity::delete(&mut conn, id).await?)
}
pub async fn batch_delete(pool: &ConnectPool, ids: Vec<i32>) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;
    Ok(system_dict_data::Entity::batch_soft_delete(&mut conn, ids).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    system_dict_data::Entity::find(
        &mut conn,
        &system_dict_data::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn get_by_label(
    pool: &ConnectPool,
    dict_id: i32,
    label: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut filter = system_dict_data::Filter {
        dict_id: Some(dict_id.to_owned()),
        label: Some(label.to_owned()),
        ..Default::default()
    };
    if let Some(id) = filter_id {
        filter.id_ne = Some(id);
    }
    let mut conn = pool.conn().await?;
    Ok(system_dict_data::Entity::find(&mut conn, &filter).await?)
}
pub async fn paginate(pool: &ConnectPool, filter: &Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = system_dict_data::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        &filter.filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}
pub type Info = system_dict_data::Entity;

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(flatten)]
    filter: system_dict_data::Filter,
    #[serde(flatten)]
    paginate: PaginateParams,
}
