use crate::{
    generate_prisma::system_user,
    prisma::{system_login_log, SortOrder},
    system_user_service, Database, Result, ServiceError,
};
use prisma_client_rust::{or, prisma_models::parse_datetime};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::{
    datetime::to_local_string,
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(
    db: &Database,
    user_id: &i32,
    ip_address: &str,
    params: CreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .system_login_log()
        .create_unchecked(*user_id, ip_address.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_login_log()
        .find_unique(system_login_log::id::equals(id))
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
                .system_login_log()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(system_login_log::id::order(SortOrder::Desc)),
            db.client.system_login_log().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct SearchParams {
    user_id: Option<i32>,
    keyword: Option<String>,
    date: Option<String>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<system_login_log::WhereParam> {
        let mut params = vec![];
        if let Some(user_id) = self.user_id {
            params.push(system_login_log::user_id::equals(user_id));
        }
        if let Some(keyword) = &self.keyword {
            let user_search = vec![or!(
                system_user::username::contains(keyword.to_string()),
                system_user::email::contains(keyword.to_string()),
                system_user::phone::contains(keyword.to_string()),
                system_user::nickname::contains(keyword.to_string()),
            )];
            params.push(or!(
                system_login_log::user::is(user_search),
                system_login_log::ip_address::contains(keyword.to_string()),
                system_login_log::ip_address_name::contains(keyword.to_string()),
                system_login_log::browser_agent::contains(keyword.to_string()),
            ));
        }
        if let Some(date) = &self.date {
            params.push(system_login_log::created_at::equals(
                parse_datetime(date).unwrap(),
            ));
        }
        params
    }

    pub fn new(
        user_id: Option<i32>,
        keyword: Option<String>,
        date: Option<String>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            user_id,
            keyword,
            date,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    user_id: i32,
    user: Option<system_user_service::Info>,
    ip_address: String,
    ip_address_name: String,
    browser_agent: String,
    created_at: String,
}

impl From<system_login_log::Data> for Info {
    fn from(value: system_login_log::Data) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            user: match value.user() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            ip_address: value.ip_address,
            ip_address_name: value.ip_address_name,
            browser_agent: value.browser_agent,
            created_at: to_local_string(value.created_at),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LoginType {
    Account = 1,
    Mobile = 2,
    QrCode = 3,
}
impl From<i32> for LoginType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Account,
            2 => Self::Mobile,
            3 => Self::QrCode,
            _ => Self::Account,
        }
    }
}
impl From<LoginType> for i32 {
    fn from(value: LoginType) -> Self {
        match value {
            LoginType::Account => 1,
            LoginType::Mobile => 2,
            LoginType::QrCode => 3,
        }
    }
}
system_login_log::partial_unchecked!(CreateParams {
    r#type
    ip_address_name
    browser_agent
});
