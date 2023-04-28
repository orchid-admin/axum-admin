use crate::{
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

pub fn _create(
    client: &Database,
    role_id: i32,
    menu_id: i32,
) -> prisma_client_rust::Create<system_role_menu::Types> {
    client.system_role_menu().create(
        system_role::id::equals(role_id),
        system_menu::id::equals(menu_id),
        vec![],
    )
}
