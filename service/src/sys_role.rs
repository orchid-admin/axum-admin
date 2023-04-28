use std::sync::Arc;

use crate::{
    prisma::{
        system_role::{self, SetParam, UncheckedSetParam},
        system_role_menu::Create,
    },
    sys_menu, sys_role_menu, Database, Result, ServiceError,
};

pub async fn create(
    client: &Database,
    name: &str,
    sign: &str,
    params: RoleCreateParams,
    menus: Vec<sys_menu::Info>,
) -> Result<system_role::Data> {
    let result = client
        ._transaction()
        .run::<ServiceError, _, _, _>(|client| async move {
            let role = client
                .system_role()
                .create_unchecked(name.to_owned(), sign.to_owned(), params.to_params())
                .exec()
                .await?;

            let new_client = Arc::new(client);
            if !menus.is_empty() {
                let role_menus = menus
                    .into_iter()
                    .map(|x| sys_role_menu::_create(&new_client, role.id, x.id))
                    .collect::<Vec<Create>>();
                if !role_menus.is_empty() {
                    new_client._batch(role_menus).await?;
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
            if !menus.is_empty() {
                let role_menus = menus
                    .into_iter()
                    .map(|x| sys_role_menu::_create(&new_client, role.id, x.id))
                    .collect::<Vec<Create>>();
                if !role_menus.is_empty() {
                    new_client._batch(role_menus).await?;
                }
            }

            Ok(role)
        })
        .await?;
    Ok(result)
}
pub(crate) async fn upsert(
    client: &Database,
    name: &str,
    sign: &str,
    params: Vec<UncheckedSetParam>,
) -> Result<system_role::Data> {
    let data = params
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<SetParam>>();
    Ok(client
        .system_role()
        .upsert(
            system_role::sign::equals(sign.to_owned()),
            (name.to_owned(), sign.to_owned(), data.clone()),
            data,
        )
        .exec()
        .await?)
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
