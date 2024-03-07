use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, member_bill};
use serde::Deserialize;
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn create(pool: &ConnectPool, params: member_bill::FormParamsForCreate) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(member_bill::Entity::create(&mut conn, &params).await?)
}
pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    member_bill::Entity::find(
        &mut conn,
        member_bill::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)
}
pub async fn paginate(pool: &ConnectPool, filter: Filter) -> Result<PaginateResult<Vec<Info>>> {
    let mut conn = pool.conn().await?;
    let (data, total) = member_bill::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}

pub type Info = member_bill::Entity;

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub r#type: Option<i32>,
    pub pm: Option<i32>,
    pub date: Option<String>,
    #[serde(flatten)]
    pub paginate: PaginateParams,
}
impl From<Filter> for member_bill::Filter {
    fn from(value: Filter) -> Self {
        Self {
            keyword: value.keyword,
            r#type: value.r#type,
            pm: value.pm,
            ..Default::default()
        }
    }
}
