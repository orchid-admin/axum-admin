use prisma_client_rust::UpdateMany;

use crate::{
    now_time,
    prisma::{system_menu, system_role, system_role_menu},
    sys_menu, Database, Result,
};

pub async fn get_role_menus(client: &Database, role_id: i32) -> Result<Vec<sys_menu::Info>> {
    let role_menus = client
        .system_role_menu()
        .find_many(vec![
            system_role_menu::role_id::equals(role_id),
            system_role_menu::deleted_at::equals(None),
            system_role_menu::menu::is(vec![system_menu::deleted_at::equals(None)]),
        ])
        .with(system_role_menu::menu::fetch())
        .exec()
        .await?;

    Ok(role_menus
        .into_iter()
        .map(|x| x.menu().unwrap().clone().into())
        .collect::<Vec<sys_menu::Info>>())
}

pub async fn create(
    client: &Database,
    role_id: i32,
    menu_id: i32,
) -> Result<system_role_menu::Data> {
    Ok(_create(client, role_id, menu_id).exec().await?)
}

pub async fn delete_by_role_id(client: &Database, role_id: i32) -> Result<i64> {
    Ok(client
        .system_role_menu()
        .update_many(
            vec![system_role_menu::role_id::equals(role_id)],
            vec![system_role_menu::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?)
}

pub async fn delete_by_role_id_menu_id(
    client: &Database,
    role_id_menu_ids: Vec<system_role_menu::WhereParam>,
) -> Result<i64> {
    Ok(_delete_by_role_id_menu_id(client, role_id_menu_ids)
        .exec()
        .await?)
}

pub fn _delete_by_role_id_menu_id(
    client: &Database,
    role_id_menu_ids: Vec<system_role_menu::WhereParam>,
) -> UpdateMany<system_role_menu::Types> {
    client.system_role_menu().update_many(
        role_id_menu_ids,
        vec![system_role_menu::deleted_at::set(Some(now_time()))],
    )
}

pub fn _create(client: &Database, role_id: i32, menu_id: i32) -> system_role_menu::CreateQuery {
    //Box<dyn QueryConvert<RawType = system_role_menu::Data, ReturnValue = system_role_menu::Data>>
    client.system_role_menu().create(
        system_role::id::equals(role_id),
        system_menu::id::equals(menu_id),
        vec![
            // system_role_menu::role::connect(system_role::id::equals(role_id)),
            // system_role_menu::menu::connect(system_menu::id::equals(menu_id)),
        ],
    )
}

pub fn _upsert(client: &Database, role_id: i32, menu_id: i32) -> system_role_menu::UpsertQuery {
    client.system_role_menu().upsert(
        system_role_menu::role_id_menu_id(role_id, menu_id),
        system_role_menu::create(
            system_role::id::equals(role_id),
            system_menu::id::equals(menu_id),
            vec![],
        ),
        vec![],
    )
}
