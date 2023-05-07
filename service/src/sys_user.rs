use crate::{
    now_time,
    prisma::{system_dept, system_role, system_user, SortOrder},
    sys_menu, to_local_string, Database, PaginateRequest, PaginateResponse, Result, ServiceError,
};
use serde::{Deserialize, Serialize};

pub async fn find_user_by_username(client: &Database, username: &str) -> Result<Option<Info>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::username::equals(username.to_owned())])
        .exec()
        .await?
        .map(|x| x.into()))
}

pub async fn find_user_by_phone(client: &Database, phone: &str) -> Result<Option<Info>> {
    Ok(client
        .system_user()
        .find_first(vec![system_user::phone::equals(phone.to_owned())])
        .exec()
        .await?
        .map(|x| x.into()))
}

pub async fn get_current_user_info(client: &Database, id: i32) -> Result<UserPermission> {
    let user = client
        .system_user()
        .find_first(vec![system_user::id::equals(id)])
        .with(system_user::role::fetch())
        .with(system_user::dept::fetch())
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?;
    let role = user.role().map(|x| x.cloned()).unwrap_or_default();
    let dept = user.dept().map(|x| x.cloned()).unwrap_or_default();
    let btn_auths = sys_menu::filter_menu_types(
        Some(vec![sys_menu::MenuType::BtnAuth]),
        sys_menu::get_menu_by_role(client, role.clone()).await?,
    )
    .into_iter()
    .map(|x| x.btn_auth)
    .collect::<Vec<String>>();
    let permission = UserPermission {
        user: user.into(),
        role,
        dept,
        btn_auths,
    };
    Ok(permission)
}

pub async fn get_users_by_dept_id(client: &Database, dept_id: i32) -> Result<Vec<Info>> {
    Ok(client
        .system_user()
        .find_many(vec![
            system_user::dept_id::equals(Some(dept_id)),
            system_user::deleted_at::equals(None),
        ])
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
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

pub async fn create(client: &Database, username: &str, params: UserCreateParams) -> Result<Info> {
    Ok(client
        .system_user()
        .create_unchecked(username.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(client: &Database, id: i32, params: UserCreateParams) -> Result<Info> {
    Ok(client
        .system_user()
        .update_unchecked(system_user::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn delete(client: &Database, id: i32) -> Result<Info> {
    Ok(client
        .system_user()
        .update(
            system_user::id::equals(id),
            vec![system_user::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}

pub async fn info(client: &Database, id: i32) -> Result<Info> {
    client
        .system_user()
        .find_first(vec![
            system_user::id::equals(id),
            system_user::deleted_at::equals(None),
        ])
        .exec()
        .await?
        .map(|x| x.into())
        .ok_or(ServiceError::DataNotFound)
}

pub async fn paginate(
    client: &Database,
    params: UserSearchParams,
) -> Result<PaginateResponse<Vec<Info>>> {
    let (data, total): (Vec<system_user::Data>, i64) = client
        ._batch((
            client
                .system_user()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.limit)
                .order_by(system_user::id::order(SortOrder::Desc)),
            client.system_user().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResponse {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub async fn upsert_system_user(
    client: &Database,
    username: &str,
    password: &str,
    salt: &str,
    role_id: i32,
) -> Result<Info> {
    let data = vec![
        system_user::password::set(password.to_owned()),
        system_user::salt::set(salt.to_owned()),
        system_user::role::connect(system_role::id::equals(role_id)),
    ];

    Ok(client
        .system_user()
        .upsert(
            system_user::username::equals(username.to_owned()),
            system_user::create(username.to_owned(), data.clone()),
            data,
        )
        .exec()
        .await?
        .into())
}

#[derive(Debug, Deserialize)]
pub struct UserSearchParams {
    username: Option<String>,
    nickname: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    status: Option<bool>,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateRequest,
}
impl UserSearchParams {
    fn to_params(&self) -> Vec<system_user::WhereParam> {
        let mut params = vec![system_user::deleted_at::equals(None)];
        if let Some(username) = &self.username {
            params.push(system_user::username::contains(username.to_string()));
        }
        if let Some(nickname) = &self.nickname {
            params.push(system_user::nickname::contains(nickname.to_string()));
        }
        if let Some(phone) = &self.phone {
            params.push(system_user::phone::contains(phone.to_string()));
        }
        if let Some(email) = &self.email {
            params.push(system_user::email::contains(email.to_string()));
        }
        if let Some(status) = self.status {
            params.push(system_user::status::equals(status));
        }
        if let Some(role_id) = self.role_id {
            params.push(system_user::role_id::equals(Some(role_id)));
        }
        if let Some(dept_id) = self.dept_id {
            params.push(system_user::dept_id::equals(Some(dept_id)));
        }
        params
    }
}

#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    username: String,
    nickname: String,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    phone: String,
    email: String,
    sex: i32,
    #[serde(skip)]
    password: String,
    #[serde(skip)]
    salt: String,
    describe: String,
    expire_time: Option<String>,
    status: bool,
    created_at: String,
}

impl Info {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
    pub fn get_salt(&self) -> String {
        self.salt.clone()
    }
}

impl From<system_user::Data> for Info {
    fn from(value: system_user::Data) -> Self {
        Self {
            id: value.id,
            username: value.username,
            nickname: value.nickname,
            role_id: value.role_id,
            dept_id: value.dept_id,
            phone: value.phone,
            email: value.email,
            sex: value.sex,
            password: value.password,
            salt: value.salt,
            describe: value.describe,
            expire_time: value.expire_time.map(to_local_string),
            status: value.status,
            created_at: to_local_string(value.created_at),
        }
    }
}

pub struct UserPermission {
    pub user: Info,
    pub role: Option<system_role::Data>,
    pub dept: Option<system_dept::Data>,
    pub btn_auths: Vec<String>,
}

system_user::partial_unchecked!(UserCreateParams {
    nickname
    role_id
    dept_id
    phone
    email
    sex
    password
    salt
    describe
    expire_time
    status
});

system_user::partial_unchecked!(UserUpdateParams {
    username
    nickname
    role_id
    dept_id
    phone
    email
    sex
    password
    salt
    describe
    expire_time
    status
});
