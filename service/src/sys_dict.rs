use prisma_client_rust::or;
use serde::Serialize;
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

use crate::{
    prisma::{system_dict, SortOrder},
    Database, Result, ServiceError,
};

pub async fn create(
    db: &Database,
    name: String,
    sign: String,
    params: DictCreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .system_dict()
        .create_unchecked(name, sign, params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: DictUpdateParams) -> Result<Info> {
    Ok(db
        .client
        .system_dict()
        .update_unchecked(system_dict::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_dict()
        .update(
            system_dict::id::equals(id),
            vec![system_dict::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_dict()
        .find_unique(system_dict::id::equals(id))
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}

pub async fn paginate(db: &Database, params: DictSearchParams) -> Result<impl Serialize> {
    let (data, total) = db
        .client
        ._batch((
            db.client
                .system_dict()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(system_dict::id::order(SortOrder::Desc)),
            db.client.system_dict().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct DictSearchParams {
    keyword: Option<String>,
    status: Option<bool>,
    paginate: PaginateParams,
}
impl DictSearchParams {
    fn to_params(&self) -> Vec<system_dict::WhereParam> {
        let mut params = vec![system_dict::deleted_at::equals(None)];
        if let Some(keyword) = &self.keyword {
            params.push(or!(
                system_dict::name::contains(keyword.to_string()),
                system_dict::sign::contains(keyword.to_string()),
            ));
        }
        if let Some(status) = self.status {
            params.push(system_dict::status::equals(status));
        }
        params
    }

    pub fn new(keyword: Option<String>, status: Option<bool>, paginate: PaginateParams) -> Self {
        Self {
            keyword,
            status,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    name: String,
    sign: String,
    status: bool,
    remark: String,
    created_at: String,
}

impl From<system_dict::Data> for Info {
    fn from(value: system_dict::Data) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sign: value.sign,
            status: value.status,
            remark: value.remark,
            created_at: to_local_string(value.created_at),
        }
    }
}
system_dict::partial_unchecked!(DictCreateParams {
    status
    remark
});
system_dict::partial_unchecked!(DictUpdateParams {
    name
    sign
    status
    remark
});
