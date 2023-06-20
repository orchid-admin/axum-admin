use crate::{
    prisma::{system_dict, SortOrder},
    system_dict_data_service, Database, Result, ServiceError,
};
use prisma_client_rust::or;
use serde::Serialize;
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(db: &Database, name: &str, sign: &str, params: CreateParams) -> Result<Info> {
    Ok(db
        .client
        .system_dict()
        .create_unchecked(name.to_owned(), sign.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: UpdateParams) -> Result<Info> {
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
pub async fn get_by_sign(
    db: &Database,
    sign: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut params = vec![
        system_dict::sign::equals(sign.to_owned()),
        system_dict::deleted_at::equals(None),
    ];
    if let Some(id) = filter_id {
        params.push(system_dict::id::not(id));
    }
    Ok(db
        .client
        .system_dict()
        .find_first(params)
        .exec()
        .await?
        .map(|x| x.into()))
}
pub async fn all(db: &Database) -> Result<Vec<Info>> {
    Ok(db
        .client
        .system_dict()
        .find_many(vec![system_dict::deleted_at::equals(None)])
        .order_by(system_dict::id::order(SortOrder::Asc))
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

pub async fn paginate(db: &Database, params: &SearchParams) -> Result<PaginateResult<Vec<Info>>> {
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

pub struct SearchParams {
    keyword: Option<String>,
    status: Option<bool>,
    paginate: PaginateParams,
}
impl SearchParams {
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
    data: Vec<system_dict_data_service::Info>,
}

impl Info {
    pub fn data_is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl From<system_dict::Data> for Info {
    fn from(value: system_dict::Data) -> Self {
        let data = match value.system_dict_data() {
            Ok(data) => data
                .iter()
                .filter(|x| x.deleted_at.eq(&None))
                .map(|x| x.clone().into())
                .collect::<Vec<system_dict_data_service::Info>>(),
            _ => vec![],
        };
        Self {
            id: value.id,
            name: value.name,
            sign: value.sign,
            status: value.status,
            remark: value.remark,
            created_at: to_local_string(value.created_at),
            data,
        }
    }
}
system_dict::partial_unchecked!(CreateParams {
    status
    remark
});
system_dict::partial_unchecked!(UpdateParams {
    name
    sign
    status
    remark
});
