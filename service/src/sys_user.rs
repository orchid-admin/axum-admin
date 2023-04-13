use crate::{prisma::system_user, Database, Result};

system_user::partial_unchecked!(CreateUserInfo { username password salt });

pub async fn find_user_by_username(
    client: Database,
    username: &str,
) -> Result<Option<system_user::Data>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::username::equals(username.to_owned())])
        .exec()
        .await?)
}

pub async fn find_user_by_phone(
    client: Database,
    phone: &str,
) -> Result<Option<system_user::Data>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::phone::equals(Some(phone.to_owned()))])
        .exec()
        .await?)
}

pub async fn create_user(
    client: Database,
    username: &str,
    password: &str,
    salt: &str,
) -> Result<system_user::Data> {
    use prisma_client_rust::chrono::{DateTime, FixedOffset, Utc};
    let now_time: DateTime<FixedOffset> =
        Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
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
