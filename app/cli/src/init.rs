use config::Config;
use service::{system_menu, system_user};
use utils::password::Password;

#[derive(Debug, clap::Args)]
pub struct CliInitParams {
    /// System User`s Username Password
    pub username_password: String,
}

pub async fn exec(params: &CliInitParams) -> service::Result<()> {
    let config = Config::load();
    let database_connect_pool = model::connect::DbConnectPool::new(&config.database_url())?;

    let user = system_user::find_user_by_username(&database_connect_pool, "admin").await?;
    if user.is_none() {
        let (password, slat) =
            Password::generate_hash_salt(params.username_password.as_bytes()).unwrap();
        system_user::create(
            &database_connect_pool,
            system_user::FormParamsForCreate {
                username: "admin".to_owned(),
                nickname: "admin".to_owned(),
                role_id: None,
                dept_id: None,
                sex: 1,
                password,
                salt: slat,
                status: 1,
                ..Default::default()
            },
        )
        .await?;
    }
    tracing::info!("System User finish..");

    tracing::info!("Check Menu..");
    let menus = system_menu::get_menus(&database_connect_pool).await?;
    if menus.is_empty() {
        crate::menu::import().await?;
    }
    tracing::info!("Menu Import finish..");
    Ok(())
}
