use service::{sys_menu, sys_role, sys_user, Database};
use utils::password::Password;

pub async fn exec() -> service::Result<()> {
    let db_config = service::DatabaseConfig::default();
    let role_sign = db_config.get_admin_role_sign();
    let user_sign = db_config.get_admin_username();
    let db = service::Database::new(db_config).await?;
    let mut role = sys_role::get_by_sign(&db, &role_sign, None).await?;
    let mut role_sign_input = String::new();
    if role.is_none() {
        role_sign_input = get_input_role_sign(&db, role_sign_input, &role_sign).await?;
    }

    let mut username_sign_input = String::new();
    let mut username_password_input = String::new();

    let user = sys_user::find_user_by_username(&db, &user_sign).await?;

    if user.is_none() {
        username_sign_input = get_input_username_sign(&db, username_sign_input, &user_sign).await?;

        println!("请输入超级管理员登录密码(默认:admin123456):");
        std::io::stdin()
            .read_line(&mut username_password_input)
            .unwrap();
        if username_sign_input.is_empty() {
            username_sign_input = user_sign;
        }

        if username_password_input.is_empty() {
            username_password_input = String::from("admin123456");
        }
    }
    if !role_sign_input.is_empty() {
        println!("正在创建角色...");
        role = Some(
            sys_role::create(
                &db,
                "超级管理员",
                &role_sign_input,
                sys_role::CreateParams {
                    sort: Some(0),
                    describe: Some(String::new()),
                    status: Some(true),
                },
                vec![],
            )
            .await?,
        );
        println!("创建角色完成");
    }

    if !username_sign_input.is_empty() {
        println!("正在创建登录账户...");
        let (password, slat) =
            Password::generate_hash_salt(username_password_input.as_bytes()).unwrap();
        sys_user::create(
            &db,
            &username_sign_input,
            sys_user::CreateParams {
                nickname: Some("超级管理员".to_owned()),
                role_id: Some(role.map(|x| x.id)),
                dept_id: None,
                phone: Some(String::new()),
                email: Some(String::new()),
                sex: Some(1),
                password: Some(password),
                salt: Some(slat),
                describe: Some(String::new()),
                expire_time: None,
                status: Some(true),
            },
        )
        .await?;
        println!("创建登录账户完成");
    }

    println!("正在检测菜单...");
    let menus = sys_menu::get_menus(&db).await?;
    if menus.is_empty() {
        println!("正在创建菜单数据...");
        crate::menu::import().await?;
        println!("创建菜单数据完成");
    } else {
        println!("菜单数据不为空，不可创建菜单数据");
    }
    println!("完成");
    Ok(())
}

#[async_recursion::async_recursion]
async fn get_input_role_sign(
    db: &Database,
    mut role_sign_input: String,
    default_role_sign: &str,
) -> service::Result<String> {
    println!("请输入超级管理员角色标识(默认:admin):");
    std::io::stdin().read_line(&mut role_sign_input).unwrap();
    if role_sign_input.is_empty() {
        role_sign_input = default_role_sign.to_owned();
    }
    let role = sys_role::get_by_sign(&db, &role_sign_input, None).await?;
    if role.is_some() {
        println!("数据已存在，请重新输入");
        return get_input_role_sign(db, role_sign_input, default_role_sign).await;
    }
    Ok(role_sign_input)
}

#[async_recursion::async_recursion]
async fn get_input_username_sign(
    db: &Database,
    mut username_input: String,
    default_role_sign: &str,
) -> service::Result<String> {
    println!("请输入超级管理员用户名(默认:admin):");
    std::io::stdin().read_line(&mut username_input).unwrap();
    if username_input.is_empty() {
        username_input = default_role_sign.to_owned();
    }
    let role = sys_user::find_user_by_username(db, &username_input).await?;
    if role.is_some() {
        println!("用户名已存在，请重新输入");
        return get_input_role_sign(db, username_input, default_role_sign).await;
    }
    Ok(username_input)
}
