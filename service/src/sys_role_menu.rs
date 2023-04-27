use crate::{prisma::system_role_menu, sys_menu, Database, Result};

pub async fn get_role_menus(client: &Database, role_id: i32) -> Result<Vec<sys_menu::Info>> {
    let role_menus = client
        .system_role_menu()
        .find_many(vec![
            system_role_menu::role_id::equals(role_id),
            system_role_menu::deleted_at::equals(None),
        ])
        .with(system_role_menu::menu::fetch())
        .exec()
        .await?;

    Ok(role_menus
        .into_iter()
        .map(|x| x.menu().unwrap().clone().into())
        .collect::<Vec<sys_menu::Info>>())
}
