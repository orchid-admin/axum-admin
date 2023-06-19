use prisma_client_rust::{or, prisma_models::parse_datetime};
use serde::Serialize;
use utils::{
    datetime::to_local_string,
    paginate::{PaginateParams, PaginateResult},
};

use crate::{
    prisma::{system_action_log, SortOrder},
    sys_menu, sys_user, Database, Result, ServiceError,
};

pub async fn create(
    db: &Database,
    user_id: i32,
    menu_id: i32,
    ip_address: &str,
    params: CreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .system_action_log()
        .create_unchecked(user_id, menu_id, ip_address.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_action_log()
        .find_unique(system_action_log::id::equals(id))
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
                .system_action_log()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(system_action_log::id::order(SortOrder::Desc)),
            db.client.system_action_log().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct SearchParams {
    user_id: Option<i32>,
    menu_id: Option<i32>,
    keyword: Option<String>,
    date: Option<String>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<system_action_log::WhereParam> {
        let mut params = vec![];
        if let Some(user_id) = self.user_id {
            params.push(system_action_log::user_id::equals(user_id));
        }
        if let Some(menu_id) = self.menu_id {
            params.push(system_action_log::menu_id::equals(menu_id));
        }
        if let Some(keyword) = &self.keyword {
            params.push(or!(
                system_action_log::ip_address::contains(keyword.to_string()),
                system_action_log::ip_address_name::contains(keyword.to_string()),
                system_action_log::browser_agent::contains(keyword.to_string()),
            ));
        }
        if let Some(date) = &self.date {
            params.push(system_action_log::created_at::equals(
                parse_datetime(date).unwrap(),
            ));
        }
        params
    }

    pub fn new(
        user_id: Option<i32>,
        menu_id: Option<i32>,
        keyword: Option<String>,
        date: Option<String>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            user_id,
            menu_id,
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
    user: Option<sys_user::Info>,
    menu_id: i32,
    menu: Option<sys_menu::Info>,
    menu_names: String,
    ip_address: String,
    ip_address_name: String,
    browser_agent: String,
    created_at: String,
}

impl From<system_action_log::Data> for Info {
    fn from(value: system_action_log::Data) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            user: match value.user() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            menu_id: value.menu_id,
            menu: match value.menu() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            menu_names: value.menu_names,
            ip_address: value.ip_address,
            ip_address_name: value.ip_address_name,
            browser_agent: value.browser_agent,
            created_at: to_local_string(value.created_at),
        }
    }
}
system_action_log::partial_unchecked!(CreateParams {
    menu_names
    ip_address_name
    browser_agent
});
