use crate::{
    prisma::{
        system_dept, system_role,
        system_user::{self, UncheckedSetParam},
        SortOrder,
    },
    system_dept_service, system_menu_service, system_role_service, DataPower, Database, Result,
    ServiceError,
};
use getset::Getters;
use prisma_client_rust::or;
use serde::{Deserialize, Serialize};
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

pub async fn find_user_by_username(db: &Database, username: &str) -> Result<Option<Info>> {
    Ok(db
        .client
        .system_user()
        .find_first(vec![system_user::username::equals(username.to_owned())])
        .exec()
        .await?
        .map(|x| x.into()))
}

pub async fn find_user_by_phone(db: &Database, phone: &str) -> Result<Option<Info>> {
    Ok(db
        .client
        .system_user()
        .find_first(vec![system_user::phone::equals(phone.to_owned())])
        .exec()
        .await?
        .map(|x| x.into()))
}

pub async fn get_current_user_info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_user()
        .find_first(vec![system_user::id::equals(id)])
        .with(system_user::role::fetch())
        .with(system_user::dept::fetch())
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}

pub async fn check_user_permission(
    db: &Database,
    user_id: i32,
    method: &str,
    path: &str,
) -> Result<bool> {
    let user = db
        .client
        .system_user()
        .find_first(vec![system_user::id::equals(user_id)])
        .with(system_user::role::fetch())
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?;
    if user.username.eq(&db.config.admin_username) {
        return Ok(true);
    }
    let role = user.role().map(|x| x.cloned()).unwrap_or_default();
    if let Some(role) = role.clone() {
        if role.sign.eq(&db.config.admin_role_sign) {
            return Ok(true);
        }
    }
    let auths = system_menu_service::filter_menu_types(
        Some(vec![system_menu_service::MenuType::Api]),
        system_menu_service::get_menu_by_role(db, role.clone().map(|x| x.into())).await?,
    )
    .into_iter()
    .filter(|x| x.api_method.to_uppercase().eq(method) && x.api_url.eq(path))
    .count();
    Ok(auths > 0)
}

pub async fn get_users_by_dept_id(db: &Database, dept_id: i32) -> Result<Vec<Info>> {
    Ok(db
        .client
        .system_user()
        .find_many(vec![
            system_user::dept_id::equals(dept_id),
            system_user::deleted_at::equals(None),
        ])
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

pub async fn batch_set_dept(
    db: &Database,
    dept_id: Option<i32>,
    user_ids: Vec<i32>,
) -> Result<i64> {
    Ok(db
        .client
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

pub async fn create(db: &Database, username: &str, params: CreateParams) -> Result<Info> {
    Ok(db
        .client
        .system_user()
        .create_unchecked(username.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: Vec<UncheckedSetParam>) -> Result<Info> {
    Ok(db
        .client
        .system_user()
        .update_unchecked(system_user::id::equals(id), params)
        .exec()
        .await?
        .into())
}

pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_user()
        .update(
            system_user::id::equals(id),
            vec![system_user::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}

pub async fn info(db: &Database, id: i32) -> Result<Info> {
    db.client
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
    db: &Database,
    params: SearchParams,
) -> Result<PaginateResult<Vec<DataPower<Info>>>> {
    let mut query_params = params.to_params();
    if let Some(dept_id) = params.dept_id {
        query_params.push(system_user::dept_id::in_vec(
            system_dept_service::get_dept_children_ids(db, dept_id).await?,
        ));
    }
    let (data, total): (Vec<system_user::Data>, i64) = db
        .client
        ._batch((
            db.client
                .system_user()
                .find_many(query_params.clone())
                .with(system_user::role::fetch())
                .with(system_user::dept::fetch())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(system_user::id::order(SortOrder::Desc)),
            db.client.system_user().count(query_params),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data
            .into_iter()
            .map(|x| DataPower {
                _can_edit: x.username.ne(&db.config.admin_username),
                _can_delete: x.username.ne(&db.config.admin_username),
                data: x.into(),
            })
            .collect::<Vec<DataPower<Info>>>(),
    })
}

pub async fn set_last_login(db: &Database, user_id: &i32, login_ip: &str) -> Result<Info> {
    Ok(db
        .client
        .system_user()
        .update(
            system_user::id::equals(*user_id),
            vec![
                system_user::last_login_ip::set(login_ip.to_owned()),
                system_user::last_login_time::set(Some(now_time())),
            ],
        )
        .exec()
        .await?
        .into())
}

pub async fn upsert_system_user(
    db: &Database,
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

    Ok(db
        .client
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
pub struct SearchParams {
    keyword: Option<String>,
    status: Option<i32>,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<system_user::WhereParam> {
        let mut params = vec![system_user::deleted_at::equals(None)];
        if let Some(k) = &self.keyword {
            params.push(or!(
                system_user::username::contains(k.to_string()),
                system_user::nickname::contains(k.to_string()),
                system_user::phone::contains(k.to_string()),
                system_user::email::contains(k.to_string())
            ));
        }
        if let Some(status) = self.status {
            params.push(system_user::status::equals(status));
        }
        if let Some(role_id) = self.role_id {
            params.push(system_user::role_id::equals(role_id));
        }
        params
    }

    pub fn new(
        keyword: Option<String>,
        status: Option<i32>,
        role_id: Option<i32>,
        dept_id: Option<i32>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            keyword,
            status,
            role_id,
            dept_id,
            paginate,
        }
    }
}

#[derive(Debug, Serialize, Getters)]
pub struct Info {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub")]
    username: String,
    nickname: String,
    role_id: i32,
    dept_id: i32,
    phone: String,
    email: String,
    sex: i32,
    #[serde(skip)]
    #[getset(get = "pub")]
    password: String,
    #[serde(skip)]
    #[getset(get = "pub")]
    salt: String,
    describe: String,
    expire_time: Option<String>,
    status: i32,
    created_at: String,
    last_login_ip: String,
    last_login_time: Option<String>,
    #[getset(get = "pub")]
    dept: Option<system_dept_service::Info>,
    #[getset(get = "pub")]
    role: Option<system_role_service::Info>,
}

impl From<system_user::Data> for Info {
    fn from(value: system_user::Data) -> Self {
        let dept = match value.dept() {
            Ok(dept) => dept.map(|x| x.clone().into()),
            Err(_) => None,
        };
        let role = match value.role() {
            Ok(role) => role.map(|x| x.clone().into()),
            Err(_) => None,
        };
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
            last_login_ip: value.last_login_ip,
            last_login_time: value.last_login_time.map(to_local_string),
            created_at: to_local_string(value.created_at),
            dept,
            role,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Permission {
    pub user: Info,
    pub role: Option<system_role::Data>,
    pub dept: Option<system_dept::Data>,
    pub btn_auths: Vec<String>,
}

system_user::partial_unchecked!(CreateParams {
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

system_user::partial_unchecked!(UpdateParams {
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

system_user::partial_unchecked!(UpdatePasswordParams { password salt });
