use prisma_client_rust::or;
use serde::Serialize;
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

use crate::{
    prisma::{system_dict_data, SortOrder},
    sys_dict, Database, Result, ServiceError,
};

pub async fn create(
    db: &Database,
    dict_id: i32,
    label: String,
    value: i32,
    params: DictDataCreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .system_dict_data()
        .create_unchecked(dict_id, label, value, params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: DictDataUpdateParams) -> Result<Info> {
    Ok(db
        .client
        .system_dict_data()
        .update_unchecked(system_dict_data::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_dict_data()
        .update(
            system_dict_data::id::equals(id),
            vec![system_dict_data::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_dict_data()
        .find_unique(system_dict_data::id::equals(id))
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
                .system_dict_data()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(system_dict_data::id::order(SortOrder::Desc)),
            db.client.system_dict_data().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct DictSearchParams {
    dict_id: Option<i32>,
    keyword: Option<String>,
    status: Option<bool>,
    paginate: PaginateParams,
}
impl DictSearchParams {
    fn to_params(&self) -> Vec<system_dict_data::WhereParam> {
        let mut params = vec![system_dict_data::deleted_at::equals(None)];
        if let Some(dict_id) = self.dict_id {
            params.push(system_dict_data::dict_id::equals(dict_id));
        }
        if let Some(keyword) = &self.keyword {
            params.push(or!(system_dict_data::label::contains(keyword.to_string()),));
        }
        if let Some(status) = self.status {
            params.push(system_dict_data::status::equals(status));
        }
        params
    }

    pub fn new(
        dict_id: Option<i32>,
        keyword: Option<String>,
        status: Option<bool>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            dict_id,
            keyword,
            status,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    dict_id: i32,
    dict: Option<sys_dict::Info>,
    label: String,
    value: i32,
    status: bool,
    sort: i32,
    remark: String,
    created_at: String,
}

impl From<system_dict_data::Data> for Info {
    fn from(value: system_dict_data::Data) -> Self {
        Self {
            id: value.id,
            dict_id: value.dict_id,
            dict: match value.dict() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            label: value.label,
            value: value.value,
            status: value.status,
            sort: value.sort,
            remark: value.remark,
            created_at: to_local_string(value.created_at),
        }
    }
}
system_dict_data::partial_unchecked!(DictDataCreateParams {
    status
    remark
});
system_dict_data::partial_unchecked!(DictDataUpdateParams {
    label
    value
    status
    sort
    remark
});
