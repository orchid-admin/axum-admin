use crate::{
    member_service,
    prisma::{member, member_team, SortOrder},
    Database, Result, ServiceError,
};
use prisma_client_rust::{or, prisma_models::parse_datetime};
use serde::Serialize;
use utils::{
    datetime::to_local_string,
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(
    db: &Database,
    owner_uid: i32,
    parent_uid: i32,
    uid: i32,
    params: CreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .member_team()
        .create_unchecked(owner_uid, parent_uid, uid, params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .member_team()
        .find_unique(member_team::id::equals(id))
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}
pub async fn paginate(db: &Database, params: &SearchParams) -> Result<PaginateResult<Vec<Info>>> {
    let (data, total) = db
        .client
        ._batch((
            db.client
                .member_team()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(member_team::id::order(SortOrder::Desc))
                .with(member_team::owner::fetch())
                .with(member_team::parent::fetch())
                .with(member_team::user::fetch()),
            db.client.member_team().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct SearchParams {
    keyword: Option<String>,
    date: Option<String>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<member_team::WhereParam> {
        let mut params = vec![];
        if let Some(keyword) = &self.keyword {
            let user_search = vec![or!(
                member::unique_code::contains(keyword.to_string()),
                member::email::contains(keyword.to_string()),
                member::mobile::contains(keyword.to_string()),
                member::nickname::contains(keyword.to_string()),
            )];
            params.push(or!(
                member_team::owner::is(user_search.clone()),
                member_team::parent::is(user_search.clone()),
                member_team::user::is(user_search),
            ));
        }
        if let Some(date) = &self.date {
            params.push(member_team::created_at::equals(
                parse_datetime(date).unwrap(),
            ));
        }
        params
    }

    pub fn new(keyword: Option<String>, date: Option<String>, paginate: PaginateParams) -> Self {
        Self {
            keyword,
            date,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    owner_uid: i32,
    owner: Option<member_service::Info>,
    parent_uid: i32,
    parent: Option<member_service::Info>,
    uid: i32,
    user: Option<member_service::Info>,
    level: i32,
    created_at: String,
}

impl From<member_team::Data> for Info {
    fn from(value: member_team::Data) -> Self {
        Self {
            id: value.id,
            owner_uid: value.owner_uid,
            owner: match value.owner() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            parent_uid: value.parent_uid,
            parent: match value.parent() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            uid: value.uid,
            user: match value.user() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            level: value.level,
            created_at: to_local_string(value.created_at),
        }
    }
}
member_team::partial_unchecked!(CreateParams { level });
