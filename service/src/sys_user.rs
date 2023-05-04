use crate::{
    prisma::{system_dept, system_role, system_user},
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
            vec![match dept_id {
                Some(dept_id) => system_user::dept::connect(system_dept::id::equals(dept_id)),
                None => system_user::dept::disconnect(),
            }],
        )
        .exec()
        .await?)
}

pub async fn create(
    client: &Database,
    username: &str,
    params: Vec<system_user::UncheckedSetParam>,
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
    params: UserCreateParams,
) -> Result<system_user::Data> {
    let mut data = vec![];
    if let Some(password) = params.password {
        data.push(system_user::password::set(password));
    }
    if let Some(salt) = params.salt {
        data.push(system_user::salt::set(salt));
    }
    if let Some(Some(role_id)) = params.role_id {
        data.push(system_user::role::connect(system_role::id::equals(role_id)));
    }

    Ok(client
        .system_user()
        .upsert(
            system_user::username::equals(username.to_owned()),
            system_user::create(username.to_owned(), data.clone()),
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

system_user::partial_unchecked!(UserCreateParams {
    password
    salt
    role_id
});
