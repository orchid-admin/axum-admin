use crate::{
    now_time,
    prisma::{
        system_role,
        system_user::{self, UniqueWhereParam},
    },
    Database, Result,
};

pub async fn find_user_by_username(
    client: &Database,
    username: &str,
) -> Result<Option<system_user::Data>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::username::equals(username.to_owned())])
        .exec()
        .await?)
}

pub async fn find_user_by_phone(
    client: &Database,
    phone: &str,
) -> Result<Option<system_user::Data>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::phone::equals(Some(phone.to_owned()))])
        .exec()
        .await?)
}

pub async fn get_current_user_info(client: &Database, id: i32) -> Result<Option<UserPermission>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::id::equals(id)])
        .with(system_user::role::fetch())
        .exec()
        .await?
        .map(|user| {
            let role = user.role().map(|x| x.cloned()).unwrap_or_default();
            UserPermission { user, role }
        }))
}

pub async fn create(
    client: &Database,
    username: &str,
    password: &str,
    salt: &str,
) -> Result<system_user::Data> {
    let now_time = now_time();
    Ok(client
        .system_user()
        .create(
            username.to_owned(),
            password.to_owned(),
            salt.to_owned(),
            now_time,
            now_time,
            vec![],
        )
        .exec()
        .await?)
}

pub async fn upset(
    client: &Database,
    username: &str,
    password: &str,
    salt: &str,
) -> Result<system_user::Data> {
    let now_time = now_time();
    Ok(client
        .system_user()
        .upsert(
            UniqueWhereParam::UsernameEquals(username.to_owned()),
            (
                username.to_owned(),
                password.to_owned(),
                salt.to_owned(),
                now_time,
                now_time,
                vec![],
            ),
            vec![
                system_user::password::set(password.to_owned()),
                system_user::salt::set(salt.to_owned()),
            ],
        )
        .exec()
        .await?)
}

pub struct UserPermission {
    pub user: system_user::Data,
    pub role: Option<system_role::Data>,
}
