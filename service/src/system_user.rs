use crate::{prisma::system_user, Database, Result};

system_user::partial_unchecked!(SystemUserBaseInfo { username password });

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
