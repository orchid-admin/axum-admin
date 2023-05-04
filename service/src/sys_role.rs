use crate::{
    now_time,
    prisma::{
        system_role,
        system_role_menu,
        SortOrder,
    },
    sys_menu, sys_role_menu, DataPower, Database, PaginateRequest, PaginateResponse, Result,
    ServiceError, ADMIN_ROLE_SIGN,
};
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
    // let result = client
    // ._transaction()
    // .run::<ServiceError, _, _, _>(|client| async move {
    let role = client
        .system_role()
        .create_unchecked(name.to_owned(), sign.to_owned(), params.to_params())
        .exec()
        .await?;

    if !menus.is_empty() {
        let new_client = Arc::new(client);
        let role_menus = menus
            .into_iter()
            .map(|x| sys_role_menu::_create(&new_client, role.id, x.id))
            .collect::<Vec<system_role_menu::CreateQuery>>();
        if !role_menus.is_empty() {
            new_client._batch(role_menus).await?;
        }
    }

    Ok(role)
    // })
    // .await?;
    // Ok(result)
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
                if !current_menus.is_empty() {
                    let mut wait_delete = vec![];
                    let mut wait_create = vec![];
                    for current_menu in current_menus.clone() {
                        if !menus.contains(&current_menu) {
                            wait_delete.push(current_menu.id);
                        }
                    }
                    for menu in menus {
                        if !current_menus.contains(&menu) {
                            wait_create.push(menu.id);
                        }
                    }
                    if !wait_delete.is_empty() {
                        sys_role_menu::delete_by_role_id_menu_id(
                            &new_client,
                            wait_delete
                                .into_iter()
                                .map(|x| system_role_menu::role_id_menu_id(role.id, x))
                                .collect::<Vec<system_role_menu::WhereParam>>(),
                        )
                        .await?;
                    }
                    if !wait_create.is_empty() {
                        new_client
                            ._batch(
                                wait_create
                                    .into_iter()
                                    .map(|x| sys_role_menu::_create(&new_client, role.id, x))
                                    .collect::<Vec<system_role_menu::CreateQuery>>(),
                            )
                            .await?;
                    }
                } else {
                    let role_menus = menus
                        .into_iter()
                        .map(|x| sys_role_menu::_create(&new_client, role.id, x.id))
                        .collect::<Vec<system_role_menu::CreateQuery>>();
                    if !role_menus.is_empty() {
                        new_client._batch(role_menus).await?;
                    }
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

pub async fn paginate(client: &Database, params: RoleSearchParams) -> Result<impl Serialize> {
    let data = client
        .system_role()
        .find_many(params.to_params())
        .skip(params.paginate.get_skip())
        .take(params.paginate.limit)
        .order_by(system_role::id::order(SortOrder::Desc))
        .select(RoleQuery::select())
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<DataPower<RoleQuery::Data>>>();
    let res = PaginateResponse {
        total: client
            .system_role()
            .count(params.to_params())
            .exec()
            .await?,
        data,
    };
    Ok(res)
}

pub async fn info(client: &Database, id: i32) -> Result<RoleQuery::Data> {
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
        .select(RoleQuery::select())
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?;
    Ok(data)
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

impl From<RoleQuery::Data> for DataPower<RoleQuery::Data> {
    fn from(value: RoleQuery::Data) -> Self {
        Self {
            _can_edit: value.sign.ne(ADMIN_ROLE_SIGN),
            _can_delete: value.sign.ne(ADMIN_ROLE_SIGN),
            data: value,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleSearchParams {
    name: Option<String>,
    sign: Option<String>,
    describe: Option<String>,
    status: Option<bool>,
    #[serde(flatten)]
    paginate: PaginateRequest,
}

impl RoleSearchParams {
    fn to_params(&self) -> Vec<system_role::WhereParam> {
        let mut params = vec![system_role::deleted_at::equals(None)];
        if let Some(name) = &self.name {
            params.push(system_role::name::contains(name.to_string()));
        }
        if let Some(sign) = &self.sign {
            params.push(system_role::sign::contains(sign.to_string()));
        }
        if let Some(describe) = &self.describe {
            params.push(system_role::describe::contains(describe.to_string()));
        }
        if let Some(status) = self.status {
            params.push(system_role::status::equals(status));
        }
        params
    }
}

system_role::select!(RoleQuery {
    id
    name
    sign
    describe
    status
});

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
