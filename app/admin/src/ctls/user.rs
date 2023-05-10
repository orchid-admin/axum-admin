use super::Claims;
use crate::{error::Result, extracts::ValidatorJson, password, state::AppState};
use axum::{
    body::Empty,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use service::{
    sys_menu::{self, MenuType},
    sys_user::{self, UserCreateParams, UserUpdateParams},
};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/user", get(index))
        .route("/user/:id", get(info))
        .route("/user", post(create))
        .route("/user/:id", put(update))
        .route("/user/:id", delete(del))
        .route("/user/get_menu", get(get_menu))
        .route("/user/get_user_permission", get(get_user_permission))
        .with_state(state)
}
/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<sys_user::UserSearchParams>,
) -> Result<impl IntoResponse> {
    Ok(Json(sys_user::paginate(&state.db, params).await?))
}

/// 详情
async fn info(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse> {
    Ok(Json(sys_user::info(&state.db, id).await?))
}

/// 新增
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    sys_user::create(&state.db, &params.username.clone(), params.into()).await?;
    Ok(Empty::new())
}

/// 更新
async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    sys_user::update(&state.db, id, params.into()).await?;
    Ok(Empty::new())
}

/// 删除
async fn del(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse> {
    sys_user::delete(&state.db, id).await?;
    Ok(Empty::new())
}

/// 获取当前用户角色菜单
async fn get_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        sys_menu::get_user_slide_menu_trees(
            &state.db,
            claims.user_id,
            Some(vec![
                MenuType::Menu,
                MenuType::Redirect,
                MenuType::Iframe,
                MenuType::Link,
            ]),
        )
        .await?,
    ))
}

/// 获取当前用户权限
async fn get_user_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let user_permission = sys_user::get_current_user_info(&state.db, claims.user_id).await?;
    Ok(Json(UserPermission {
        username: user_permission.user.get_username(),
        photo: None,
        time: 0,
        roles: match user_permission.role {
            Some(role) => vec![role.sign],
            None => vec!["admin".to_owned()],
        },
        // todo
        btn_auths: user_permission.btn_auths,
    }))
}

#[derive(Debug, Deserialize, Validate)]
struct CreateRequest {
    username: String,
    nickname: String,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    phone: Option<String>,
    email: Option<String>,
    sex: i32,
    password: Option<String>,
    describe: Option<String>,
    expire_time: Option<String>,
    status: bool,
}

impl From<CreateRequest> for UserCreateParams {
    fn from(value: CreateRequest) -> Self {
        let mut data = Self {
            nickname: Some(value.nickname),
            role_id: value
                .role_id
                .and_then(|x| if x <= 0 { None } else { Some(Some(x)) }),
            dept_id: value
                .dept_id
                .and_then(|x| if x <= 0 { None } else { Some(Some(x)) }),
            phone: value.phone,
            email: value.email,
            sex: Some(value.sex),
            password: None,
            salt: None,
            expire_time: None,
            status: Some(value.status),
            describe: value.describe,
        };

        if let Some(password) = value.password {
            let (encode_password, salt) =
                password::Password::generate_hash_salt(password.as_bytes()).unwrap();
            data.password = Some(encode_password);
            data.salt = Some(salt);
        }
        if let Some(expire_time) = value.expire_time {
            data.expire_time = Some(Some(service::parse_string(expire_time)))
        }
        data
    }
}

impl From<CreateRequest> for UserUpdateParams {
    fn from(value: CreateRequest) -> Self {
        let mut data = Self {
            username: Some(value.username),
            nickname: Some(value.nickname),
            role_id: value
                .role_id
                .and_then(|x| if x <= 0 { None } else { Some(Some(x)) }),
            dept_id: value
                .dept_id
                .and_then(|x| if x <= 0 { None } else { Some(Some(x)) }),
            phone: value.phone,
            email: value.email,
            sex: Some(value.sex),
            password: None,
            salt: None,
            expire_time: None,
            status: Some(value.status),
            describe: value.describe,
        };

        if let Some(password) = value.password {
            let (encode_password, salt) =
                password::Password::generate_hash_salt(password.as_bytes()).unwrap();
            data.password = Some(encode_password);
            data.salt = Some(salt);
        }
        if let Some(expire_time) = value.expire_time {
            data.expire_time = Some(Some(service::parse_string(expire_time)))
        }
        data
    }
}

#[derive(Debug, Serialize)]
struct UserPermission {
    username: String,
    photo: Option<String>,
    time: i64,
    roles: Vec<String>,
    btn_auths: Vec<String>,
}
