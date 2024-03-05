use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, member_team};
use serde::Deserialize;
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(pool: &ConnectPool, params: member_team::FormParamsForCreate) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(member_team::Entity::create(&mut conn, &params).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    member_team::Entity::find(
        &mut conn,
        &member_team::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn paginate(pool: &ConnectPool, filter: &Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = member_team::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        &filter.filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}
pub type Info = member_team::Entity;

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(flatten)]
    filter: member_team::Filter,
    #[serde(flatten)]
    paginate: PaginateParams,
}
