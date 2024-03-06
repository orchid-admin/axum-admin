use crate::{system_menu, Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_dept, system_role, system_user};
use serde::{Deserialize, Serialize};
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn find_user_by_username(pool: &ConnectPool, username: &str) -> Result<Option<Info>> {
    let mut conn = pool.conn().await?;
    let info = system_user::Entity::find(
        &mut conn,
        system_user::Filter {
            username: Some(username.to_owned()),
            ..Default::default()
        },
    )
    .await?;
    Ok(info)
}

pub async fn find_user_by_phone(
    pool: &ConnectPool,
    phone: &str,
) -> Result<Option<system_user::Entity>> {
    let mut conn = pool.conn().await?;
    let info = system_user::Entity::find(
        &mut conn,
        system_user::Filter {
            phone: Some(phone.to_owned()),
            ..Default::default()
        },
    )
    .await?;
    Ok(info)
}

pub async fn get_current_user_info(pool: &ConnectPool, id: i32) -> Result<UserInfo> {
    let mut conn = pool.conn().await?;
    let mut user_info: UserInfo = system_user::Entity::find(
        &mut conn,
        system_user::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?
    .into();
    if user_info.user.dept_id.is_some() {
        user_info.dept = system_dept::Entity::find(
            &mut conn,
            &system_dept::Filter {
                id: user_info.user.dept_id,
                ..Default::default()
            },
        )
        .await?;
    }

    if user_info.user.role_id.is_some() {
        user_info.role = system_role::Entity::find(
            &mut conn,
            &system_role::Filter {
                id: user_info.user.role_id,
                ..Default::default()
            },
        )
        .await?;
    }

    Ok(user_info)
}

pub async fn check_user_permission(
    pool: &ConnectPool,
    user_id: i32,
    method: &str,
    path: &str,
) -> Result<bool> {
    let user_info = get_current_user_info(pool, user_id).await?;
    let auths = system_menu::filter_menu_types(
        Some(vec![system_menu::MenuType::Api]),
        system_menu::get_menu_by_role(pool, user_info.role).await?,
    )
    .into_iter()
    .filter(|x| x.check_request_permission(method, path))
    .count();
    Ok(auths > 0)
}

pub async fn get_users_by_dept_id(
    pool: &ConnectPool,
    dept_id: i32,
) -> Result<Vec<system_user::Entity>> {
    let mut conn = pool.conn().await?;
    let infos = system_user::Entity::query(
        &mut conn,
        system_user::Filter {
            dept_id: Some(dept_id),
            ..Default::default()
        },
    )
    .await?;
    Ok(infos)
}

pub async fn batch_set_dept(
    pool: &ConnectPool,
    dept_id: Option<i32>,
    user_ids: Vec<i32>,
) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_user::Entity::batch_set_dept(&mut conn, dept_id, user_ids).await?)
}

pub async fn create(
    pool: &ConnectPool,
    params: system_user::FormParamsForCreate,
) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_user::Entity::create(&mut conn, params).await?)
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_user::FormParamsForUpdate,
) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_user::Entity::update(&mut conn, id, params).await?)
}

pub async fn update_password(
    pool: &ConnectPool,
    id: i32,
    password: &str,
) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_user::Entity::update_password(&mut conn, id, password).await?)
}

pub async fn delete(pool: &ConnectPool, id: i32) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_user::Entity::delete(&mut conn, id).await?)
}

pub async fn info(pool: &ConnectPool, id: i32) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    let info = system_user::Entity::find(
        &mut conn,
        system_user::Filter {
            id: Some(id.to_owned()),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?;
    Ok(info)
}

pub async fn paginate(
    pool: &ConnectPool,
    filter: Filter,
) -> Result<PaginateResult<Vec<system_user::Entity>>> {
    let mut conn = pool.conn().await?;
    let (data, total): (Vec<system_user::Entity>, i64) = system_user::Entity::paginate(
        &mut conn,
        filter.paginate.get_page(),
        filter.paginate.get_limit(),
        filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}

pub async fn set_last_login(
    pool: &ConnectPool,
    id: i32,
    login_ip: &str,
) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    let mut info = system_user::Entity::find(
        &mut conn,
        system_user::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?;

    Ok(system_user::Entity::set_last_login(&mut conn, &mut info, login_ip).await?)
}

pub type Info = system_user::Entity;
pub type FormParamsForCreate = system_user::FormParamsForCreate;

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub status: Option<i32>,
    #[serde(flatten)]
    pub paginate: PaginateParams,
}

impl From<Filter> for system_user::Filter {
    fn from(value: Filter) -> Self {
        Self {
            keyword: value.keyword,
            role_id: value.role_id,
            dept_id: value.dept_id,
            status: value.status,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub user: system_user::Entity,
    dept: Option<system_dept::Entity>,
    pub role: Option<system_role::Entity>,
}

impl From<system_user::Entity> for UserInfo {
    fn from(value: system_user::Entity) -> Self {
        Self {
            user: value,
            dept: None,
            role: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Permission {
    pub user: UserInfo,
    pub role: Option<system_role::Entity>,
    pub dept: Option<system_dept::Entity>,
    pub btn_auths: Vec<String>,
}
