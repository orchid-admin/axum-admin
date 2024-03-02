use crate::{Result, ServiceError};
use getset::Getters;
use model::{connect::DbConnectPool as ConnectPool, system_dept, system_role, system_user};
use serde::{Deserialize, Serialize};
use utils::paginate::{PaginateParams, PaginateResult};

pub async fn find_user_by_username(
    pool: &ConnectPool,
    username: &str,
) -> Result<Option<system_user::Entity>> {
    let mut conn = pool.conn().await?;
    let info = system_user::Entity::find(
        &mut conn,
        &system_user::Filter {
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
        &system_user::Filter {
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
        &system_user::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?
    .into();
    if user_info.user.dept_id().is_some() {
        user_info.dept = system_dept::Entity::find(
            &mut conn,
            &system_dept::Filter {
                id: *user_info.user.dept_id(),
                ..Default::default()
            },
        )
        .await?;
    }

    if user_info.user.role_id().is_some() {
        user_info.role = system_role::Entity::find(
            &mut conn,
            &system_role::Filter {
                id: *user_info.user.role_id(),
                ..Default::default()
            },
        )
        .await?;
    }

    Ok(user_info)
}

// pub async fn check_user_permission(
//     db: &Database,
//     user_id: i32,
//     method: &str,
//     path: &str,
// ) -> Result<bool> {
//     let user = db
//         .client
//         .system_user()
//         .find_first(vec![system_user::id::equals(user_id)])
//         .with(system_user::role::fetch())
//         .exec()
//         .await?
//         .ok_or(ServiceError::DataNotFound)?;
//     if user.username.eq(&db.config.admin_username) {
//         return Ok(true);
//     }
//     let role = user.role().map(|x| x.cloned()).unwrap_or_default();
//     let auths = system_menu_service::filter_menu_types(
//         Some(vec![system_menu_service::MenuType::Api]),
//         system_menu_service::get_menu_by_role(db, role.clone().map(|x| x.into())).await?,
//     )
//     .into_iter()
//     .filter(|x| x.api_method.to_uppercase().eq(method) && x.api_url.eq(path))
//     .count();
//     Ok(auths > 0)
// }

pub async fn get_users_by_dept_id(
    pool: &ConnectPool,
    dept_id: i32,
) -> Result<Vec<system_user::Entity>> {
    let mut conn = pool.conn().await?;
    let infos = system_user::Entity::query(
        &mut conn,
        &system_user::Filter {
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
    params: &system_user::FormParamsForCreate,
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

pub async fn delete(pool: &ConnectPool, id: i32) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    Ok(system_user::Entity::delete(&mut conn, id).await?)
}

pub async fn info(pool: &ConnectPool, id: i32) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    let info = system_user::Entity::find(
        &mut conn,
        &system_user::Filter {
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
    params: SearchParams,
) -> Result<PaginateResult<Vec<system_user::Entity>>> {
    let mut conn = pool.conn().await?;
    let (data, total): (Vec<system_user::Entity>, i64) = system_user::Entity::paginate(
        &mut conn,
        params.paginate.get_page(),
        params.paginate.get_limit(),
        &params.filter,
    )
    .await?;
    Ok(PaginateResult { total, data })
}

pub async fn set_last_login(
    pool: &ConnectPool,
    id: &i32,
    login_ip: &str,
) -> Result<system_user::Entity> {
    let mut conn = pool.conn().await?;
    let mut info = system_user::Entity::find(
        &mut conn,
        &system_user::Filter {
            id: Some(id.to_owned()),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?;

    Ok(system_user::Entity::set_last_login(&mut conn, &mut info, login_ip).await?)
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    #[serde(flatten)]
    filter: system_user::Filter,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl SearchParams {
    pub fn new(filter: system_user::Filter, paginate: PaginateParams) -> Self {
        Self { filter, paginate }
    }
}

#[derive(Debug, Serialize, Getters)]
pub struct UserInfo {
    user: system_user::Entity,
    #[getset(get = "pub")]
    dept: Option<system_dept::Entity>,
    #[getset(get = "pub")]
    role: Option<system_role::Entity>,
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
