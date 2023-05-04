use crate::{
    prisma::{
        system_dept, system_role,
        system_user::{self, SetParam, UncheckedSetParam},
    },
    Database, Result,
};

system_user::partial_unchecked!(UserCreateParams {
    password
    salt
    role_id
});
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
        .find_first(vec![system_user::phone::equals(phone.to_owned())])
        .exec()
        .await?)
}

pub async fn get_current_user_info(client: &Database, id: i32) -> Result<Option<UserPermission>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::id::equals(id)])
        .with(system_user::role::fetch())
        .with(system_user::dept::fetch())
        .exec()
        .await?
        .map(|user| {
            let role = user.role().map(|x| x.cloned()).unwrap_or_default();
            let dept = user.dept().map(|x| x.cloned()).unwrap_or_default();
            UserPermission { user, role, dept }
        }))
}

pub async fn get_users_by_dept_id(
    client: &Database,
    dept_id: i32,
) -> Result<Vec<system_user::Data>> {
    Ok(client
        .system_user()
        .find_many(vec![
            system_user::dept_id::equals(Some(dept_id)),
            system_user::deleted_at::equals(None),
        ])
        .exec()
        .await?)
}

pub async fn batch_set_dept(
    client: &Database,
    dept_id: Option<i32>,
    user_ids: Vec<i32>,
) -> Result<i64> {
    Ok(client
        .system_user()
        .update_many(
            vec![system_user::id::in_vec(user_ids)],
            vec![system_user::dept_id::set(dept_id)],
        )
        .exec()
        .await?)
}

pub async fn create(
    client: &Database,
    username: &str,
    params: Vec<UncheckedSetParam>,
) -> Result<system_user::Data> {
    Ok(client
        .system_user()
        .create_unchecked(username.to_owned(), params)
        .exec()
        .await?)
}

pub async fn upset(
    client: &Database,
    username: &str,
    params: Vec<UncheckedSetParam>,
) -> Result<system_user::Data> {
    let data = params
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<SetParam>>();

    Ok(client
        .system_user()
        .upsert(
            system_user::username::equals(username.to_owned()),
            (username.to_owned(), data.clone()),
            data,
        )
        .exec()
        .await?)
}

pub struct UserPermission {
    pub user: system_user::Data,
    pub role: Option<system_role::Data>,
    pub dept: Option<system_dept::Data>,
}
