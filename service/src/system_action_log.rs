use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_action_log};
use serde::Deserialize;
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(
    pool: &ConnectPool,
    params: system_action_log::FormParamsForCreate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_action_log::Entity::create(&mut conn, &params).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    system_action_log::Entity::find(
        &mut conn,
        &system_action_log::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn paginate(pool: &ConnectPool, filter: &Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = system_action_log::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        &filter.filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}

pub type Info = system_action_log::Entity;

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(flatten)]
    filter: system_action_log::Filter,
    #[serde(flatten)]
    paginate: PaginateParams,
}
