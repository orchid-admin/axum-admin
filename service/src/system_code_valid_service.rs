use crate::{
    generate_prisma::system_code_valid, prisma::SortOrder, Database, Result, ServiceError,
};
use prisma_client_rust::or;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(db: &Database, key: &str, code: &str, params: CreateParams) -> Result<Info> {
    Ok(db
        .client
        .system_code_valid()
        .create_unchecked(key.to_owned(), code.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: UpdateParams) -> Result<Info> {
    Ok(db
        .client
        .system_code_valid()
        .update_unchecked(system_code_valid::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_code_valid()
        .update(
            system_code_valid::id::equals(id),
            vec![system_code_valid::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}
pub async fn batch_delete(db: &Database, ids: Vec<i32>) -> Result<i64> {
    Ok(db
        .client
        .system_code_valid()
        .update_many(
            vec![system_code_valid::id::in_vec(ids)],
            vec![system_code_valid::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?)
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_code_valid()
        .find_unique(system_code_valid::id::equals(id))
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}
pub async fn get_by_type_code(
    db: &Database,
    r#type: &CodeType,
    code: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut params = vec![
        system_code_valid::r#type::equals(r#type.clone().into()),
        system_code_valid::code::equals(code.to_owned()),
        system_code_valid::deleted_at::equals(None),
    ];
    if let Some(id) = filter_id {
        params.push(system_code_valid::id::not(id));
    }
    Ok(db
        .client
        .system_code_valid()
        .find_first(params)
        .exec()
        .await?
        .map(|x| x.into()))
}
pub async fn paginate(db: &Database, params: &SearchParams) -> Result<PaginateResult<Vec<Info>>> {
    let (data, total) = db
        .client
        ._batch((
            db.client
                .system_code_valid()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(system_code_valid::id::order(SortOrder::Desc)),
            db.client.system_code_valid().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct SearchParams {
    keyword: Option<String>,
    r#type: Option<CodeType>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<system_code_valid::WhereParam> {
        let mut params = vec![system_code_valid::deleted_at::equals(None)];
        if let Some(keyword) = &self.keyword {
            params.push(or!(system_code_valid::code::contains(keyword.to_string()),));
        }
        if let Some(t) = &self.r#type {
            params.push(system_code_valid::r#type::equals(t.clone().into()));
        }
        params
    }

    pub fn new(
        keyword: Option<String>,
        r#type: Option<CodeType>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            keyword,
            r#type,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    r#type: CodeType,
    code: String,
    attach: String,
    valid_time: String,
    created_at: String,
}

impl From<system_code_valid::Data> for Info {
    fn from(value: system_code_valid::Data) -> Self {
        Self {
            id: value.id,
            r#type: value.r#type.into(),
            code: value.code,
            attach: value.attach,
            valid_time: to_local_string(value.valid_time),
            created_at: to_local_string(value.created_at),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CodeType {
    SystemUserMobileLogin = 1,
    SystemUserQrCodeLogin = 2,
    MemberEmailRegister = 3,
    MemberEmailLogin = 4,
}
impl From<i32> for CodeType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::SystemUserMobileLogin,
            2 => Self::SystemUserQrCodeLogin,
            3 => Self::MemberEmailRegister,
            4 => Self::MemberEmailLogin,
            _ => Self::SystemUserMobileLogin,
        }
    }
}
impl From<CodeType> for i32 {
    fn from(value: CodeType) -> Self {
        match value {
            CodeType::SystemUserMobileLogin => 1,
            CodeType::SystemUserQrCodeLogin => 2,
            CodeType::MemberEmailRegister => 3,
            CodeType::MemberEmailLogin => 4,
        }
    }
}
system_code_valid::partial_unchecked!(CreateParams {
    r#type
    attach
    valid_time
});
system_code_valid::partial_unchecked!(UpdateParams {
    r#type
    attach
    valid_time
});
