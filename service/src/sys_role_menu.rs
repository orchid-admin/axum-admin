use crate::{
    now_time,
    prisma::{system_menu, system_role_menu},
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

pub async fn delete_by_role_id(client: &Database, role_id: i32) -> Result<i64> {
    Ok(client
        .system_role_menu()
        .delete_many(vec![system_role_menu::role_id::equals(role_id)])
        .exec()
        .await?)
}

pub async fn delete_by_role_id_menu_id(
    client: &Database,
    role_id: i32,
    menu_id: i32,
) -> Result<system_role_menu::Data> {
    Ok(client
        .system_role_menu()
        .delete(system_role_menu::role_id_menu_id(role_id, menu_id))
        .exec()
        .await?)
}
