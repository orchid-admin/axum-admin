use crate::{
    prisma::{system_menu, system_role_menu},
    Database, Result,
};

pub async fn get_role_menus(client: &Database, role_id: i32) -> Result<Vec<system_menu::Data>> {
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
        .map(|x| x.menu().unwrap().clone())
        .collect::<Vec<system_menu::Data>>())
}
