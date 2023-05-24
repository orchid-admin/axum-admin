use crate::{
    now_time,
    prisma::{system_role, system_role_menu, SortOrder},
    sys_menu, sys_role_menu, to_local_string, DataPower, Database, PaginateParams, PaginateResult,
    Result, ServiceError, ADMIN_ROLE_SIGN,
};
use prisma_client_rust::or;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn create(
    client: &Database,
    name: &str,
    sign: &str,
    params: RoleCreateParams,
    menus: Vec<sys_menu::Info>,
) -> Result<system_role::Data> {
    // todo wait PCR 0.7
    // link: https://github.com/Brendonovich/prisma-client-rust/issues/44
    let result = client
        ._transaction()
        .run::<ServiceError, _, _, _>(|client| async move {
            let role = client
                .system_role()
                .create_unchecked(name.to_owned(), sign.to_owned(), params.to_params())
                .exec()
                .await?;

            if !menus.is_empty() {
                let role_menus = menus
                    .into_iter()
                    .map(|x| system_role_menu::create_unchecked(role.id, x.id, vec![]))
                    .collect::<Vec<system_role_menu::CreateUnchecked>>();
                if !role_menus.is_empty() {
                    client
                        .system_role_menu()
                        .create_many(role_menus)
                        .exec()
                        .await?;
                }
            }

            Ok(role)
        })
        .await?;
    Ok(result)
}

pub async fn update(
    client: &Database,
    id: i32,
    params: RoleUpdateParams,
    menus: Vec<sys_menu::Info>,
) -> Result<system_role::Data> {
    let result = client
        ._transaction()
        .run::<ServiceError, _, _, _>(|client| async move {
            let role = client
                .system_role()
                .update_unchecked(system_role::id::equals(id), params.to_params())
                .exec()
                .await?;

            let new_client = Arc::new(client);
            let current_menus = sys_role_menu::get_role_menus(&new_client, role.id).await?;

            if !menus.is_empty() {
                let wait_creates = menus
                    .clone()
                    .into_iter()
                    .filter(|x| match current_menus.is_empty() {
                        false => !current_menus.contains(x),
                        true => true,
                    })
                    .map(|x| system_role_menu::create_unchecked(role.id, x.id, vec![]))
                    .collect::<Vec<system_role_menu::CreateUnchecked>>();

                let wait_deletes = match current_menus.is_empty() {
                    false => current_menus
                        .clone()
                        .into_iter()
                        .filter(|x| !menus.contains(x))
                        .map(|x| (role.id, x.id))
                        .collect::<Vec<(i32, i32)>>(),
                    true => vec![],
                };

                if !wait_deletes.is_empty() {
                    for wait_delete in wait_deletes {
                        sys_role_menu::delete_by_role_id_menu_id(
                            &new_client,
                            wait_delete.0,
                            wait_delete.1,
                        )
                        .await?;
                    }
                }

                if !wait_creates.is_empty() {
                    new_client
                        .system_role_menu()
                        .create_many(wait_creates)
                        .exec()
                        .await?;
                }
            } else if !current_menus.is_empty() {
                sys_role_menu::delete_by_role_id(&new_client, id).await?;
            }

            Ok(role)
        })
        .await?;
    Ok(result)
}

pub async fn delete(client: &Database, id: i32) -> Result<system_role::Data> {
    let result = client
        ._transaction()
        .run::<ServiceError, _, _, _>(|client| async move {
            let info = client
                .system_role()
                .update(
                    system_role::id::equals(id),
                    vec![system_role::deleted_at::set(Some(now_time()))],
                )
                .exec()
                .await?;
            sys_role_menu::delete_by_role_id(&Arc::new(client), id).await?;
            Ok(info)
        })
        .await?;
    Ok(result)
}

pub async fn all(client: &Database) -> Result<impl Serialize> {
    let data = client
        .system_role()
        .find_many(vec![system_role::deleted_at::equals(None)])
        .order_by(system_role::id::order(SortOrder::Desc))
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<DataPower<Info>>>();
    Ok(data)
}

pub async fn paginate(client: &Database, params: &RoleSearchParams) -> Result<impl Serialize> {
    let (data, total) = client
        ._batch((
            client
                .system_role()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.limit)
                .order_by(system_role::id::order(SortOrder::Desc)),
            client.system_role().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<DataPower<Info>>>(),
    })
}

pub async fn info(client: &Database, id: i32) -> Result<Info> {
    let data = client
        .system_role()
        .find_first(vec![
            system_role::id::equals(id),
            system_role::deleted_at::equals(None),
        ])
        .with(
            system_role::system_role_menu::fetch(vec![system_role_menu::deleted_at::equals(None)])
                .with(system_role_menu::menu::fetch()),
        )
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?;
    let mut role: Info = data.clone().into();
    role.menu_ids = sys_menu::get_menu_by_role(client, Some(data))
        .await?
        .into_iter()
        .map(|x| x.id)
        .collect::<Vec<i32>>();
    Ok(role)
}

pub(crate) async fn upsert(client: &Database, name: &str, sign: &str) -> Result<system_role::Data> {
    Ok(client
        .system_role()
        .upsert(
            system_role::sign::equals(sign.to_owned()),
            system_role::create(name.to_owned(), sign.to_owned(), vec![]),
            vec![],
        )
        .exec()
        .await?)
}

pub async fn get_by_sign(
    client: &Database,
    sign: &str,
    id: Option<i32>,
) -> Result<Option<system_role::Data>> {
    let mut params = vec![system_role::sign::equals(sign.to_owned())];
    if let Some(id) = id {
        params.push(system_role::id::not(id));
    }
    Ok(client.system_role().find_first(params).exec().await?)
}

impl From<system_role::Data> for DataPower<Info> {
    fn from(value: system_role::Data) -> Self {
        Self {
            _can_edit: value.sign.ne(ADMIN_ROLE_SIGN),
            _can_delete: value.sign.ne(ADMIN_ROLE_SIGN),
            data: value.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleSearchParams {
    keyword: Option<String>,
    status: Option<bool>,
    #[serde(flatten)]
    paginate: PaginateParams,
}

impl RoleSearchParams {
    fn to_params(&self) -> Vec<system_role::WhereParam> {
        let mut params = vec![system_role::deleted_at::equals(None)];
        if let Some(keyword) = &self.keyword {
            params.push(or!(
                system_role::name::contains(keyword.to_string()),
                system_role::sign::contains(keyword.to_string()),
                system_role::describe::contains(keyword.to_string())
            ));
        }
        if let Some(status) = self.status {
            params.push(system_role::status::equals(status));
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
    pub sign: String,
    describe: String,
    status: bool,
    sort: i32,
    created_at: String,
    menu_ids: Vec<i32>,
}
impl From<system_role::Data> for Info {
    fn from(value: system_role::Data) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sign: value.sign,
            describe: value.describe,
            status: value.status,
            sort: value.sort,
            created_at: to_local_string(value.created_at),
            menu_ids: vec![],
        }
    }
}

system_role::partial_unchecked!(RoleCreateParams {
    sort
    describe
    status
});

system_role::partial_unchecked!(RoleUpdateParams {
    name
    sign
    sort
    describe
    status
});
