use crate::{
    prisma::{member, SortOrder},
    Database, Result, ServiceError,
};
use prisma_client_rust::or;
use serde::Serialize;
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(
    db: &Database,
    unique_code: &str,
    email: &str,
    params: CreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .member()
        .create_unchecked(unique_code.to_owned(), email.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: UpdateParams) -> Result<Info> {
    Ok(db
        .client
        .member()
        .update_unchecked(member::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .member()
        .update(
            member::id::equals(id),
            vec![member::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .member()
        .find_unique(member::id::equals(id))
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}
pub async fn get_by_unique_code(
    db: &Database,
    unique_code: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut params = vec![
        member::unique_code::equals(unique_code.to_owned()),
        member::deleted_at::equals(None),
    ];
    if let Some(id) = filter_id {
        params.push(member::id::not(id));
    }
    Ok(db
        .client
        .member()
        .find_first(params)
        .exec()
        .await?
        .map(|x| x.into()))
}
pub async fn get_by_email(
    db: &Database,
    email: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut params = vec![
        member::email::equals(email.to_owned()),
        member::deleted_at::equals(None),
    ];
    if let Some(id) = filter_id {
        params.push(member::id::not(id));
    }
    Ok(db
        .client
        .member()
        .find_first(params)
        .exec()
        .await?
        .map(|x| x.into()))
}
pub async fn all(db: &Database) -> Result<Vec<Info>> {
    Ok(db
        .client
        .member()
        .find_many(vec![member::deleted_at::equals(None)])
        .order_by(member::id::order(SortOrder::Asc))
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
                .member()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(member::id::order(SortOrder::Desc)),
            db.client.member().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub async fn inc_user_balance(db: &Database, user_id: i32, number: f64) -> Result<Info> {}

pub struct SearchParams {
    keyword: Option<String>,
    sex: Option<i32>,
    status: Option<bool>,
    is_promoter: Option<bool>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<member::WhereParam> {
        let mut params = vec![member::deleted_at::equals(None)];
        if let Some(keyword) = &self.keyword {
            params.push(or!(
                member::unique_code::contains(keyword.to_string()),
                member::email::contains(keyword.to_string()),
                member::mobile::contains(keyword.to_string()),
                member::nickname::contains(keyword.to_string()),
                member::remark::contains(keyword.to_string()),
            ));
        }
        if let Some(status) = self.status {
            params.push(member::status::equals(status));
        }
        if let Some(sex) = self.sex {
            params.push(member::sex::equals(sex));
        }
        if let Some(is_promoter) = self.is_promoter {
            params.push(member::is_promoter::equals(is_promoter));
        }
        params
    }

    pub fn new(
        keyword: Option<String>,
        sex: Option<i32>,
        status: Option<bool>,
        is_promoter: Option<bool>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            keyword,
            sex,
            status,
            is_promoter,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    unique_code: String,
    email: String,
    mobile: String,
    nickname: String,
    avatar: String,
    sex: i32,
    balance: f64,
    integral: f64,
    remark: String,
    status: bool,
    is_promoter: bool,
    last_login_ip: String,
    last_login_time: Option<String>,
    created_at: String,
}

impl From<member::Data> for Info {
    fn from(value: member::Data) -> Self {
        Self {
            id: value.id,
            unique_code: value.unique_code,
            email: value.email,
            mobile: value.mobile,
            nickname: value.nickname,
            avatar: value.avatar,
            sex: value.sex,
            balance: value.balance,
            integral: value.balance,
            remark: value.remark,
            status: value.status,
            is_promoter: value.is_promoter,
            last_login_ip: value.last_login_ip,
            last_login_time: value.last_login_time.map(to_local_string),
            created_at: to_local_string(value.created_at),
        }
    }
}
member::partial_unchecked!(CreateParams {
    mobile
    nickname
    avatar
    password
    sex
    remark
    status
    is_promoter
});
member::partial_unchecked!(UpdateParams {
    email
    mobile
    nickname
    avatar
    password
    sex
    remark
    status
    is_promoter
});
