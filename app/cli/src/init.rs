use service::{system_dept_service, system_menu_service, system_role_service, system_user_service};
use utils::password::Password;

#[derive(Debug, clap::Args)]
pub struct CliInitParams {
    /// Dept Name
    pub dept_name: String,
    /// System User`s Username Password
    pub username_password: String,
}

pub async fn exec(params: &CliInitParams) -> service::Result<()> {
    let db_config = service::DatabaseConfig::default();
    let db = service::Database::new(db_config.clone()).await?;

    let role_sign = db_config.get_admin_role_sign();
    let mut role: Option<system_role_service::Info> =
        system_role_service::get_by_sign(&db, &role_sign, None)
            .await?
            .map(|x| x.into());
    if role.is_none() {
        role = Some(
            system_role_service::create(
                &db,
                &role_sign,
                &role_sign,
                system_role_service::CreateParams {
                    sort: Some(0),
                    describe: Some(String::new()),
                    status: Some(1),
                },
                vec![],
            )
            .await?
            .into(),
        );
    }
    tracing::info!("System Role finish..");

    let dept = system_dept_service::create(
        &db,
        &params.dept_name,
        service::system_dept_service::CreateParams {
            parent_id: None,
            person_name: None,
            person_phone: None,
            person_email: None,
            describe: None,
            status: Some(1),
            sort: Some(0),
        },
    )
    .await?;
    tracing::info!("System Dept finish..");

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
                role_id: role.map(|x| x.id().clone()),
                dept_id: Some(dept.id().clone()),
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
