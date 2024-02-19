use service::{system_menu_service, system_user_service};
use utils::password::Password;

#[derive(Debug, clap::Args)]
pub struct CliInitParams {
    /// System User`s Username Password
    pub username_password: String,
}

pub async fn exec(params: &CliInitParams) -> service::Result<()> {
    let db_config = service::DatabaseConfig::default();
    let db = service::Database::new(db_config.clone()).await?;

    let user_sign = db_config.get_admin_username();
    let user = system_user_service::find_user_by_username(&db, &user_sign).await?;
    if user.is_none() {
        let (password, slat) =
            Password::generate_hash_salt(params.username_password.as_bytes()).unwrap();
        system_user_service::create(
            &db,
            &user_sign.clone(),
            system_user_service::CreateParams {
                nickname: Some(user_sign),
                role_id: None,
                dept_id: None,
                phone: Some(String::new()),
                email: Some(String::new()),
                sex: Some(1),
                password: Some(password),
                salt: Some(slat),
                describe: Some(String::new()),
                expire_time: None,
                status: Some(1),
            },
        )
        .await?;
    }
    tracing::info!("System User finish..");

    tracing::info!("Check Menu..");
    let menus = system_menu_service::get_menus(&db).await?;
    if menus.is_empty() {
        crate::menu::import().await?;
    }
    tracing::info!("Menu Import finish..");
    Ok(())
}
