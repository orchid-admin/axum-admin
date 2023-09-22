use super::Claims;
use crate::{error::Result, state::AppState};
use axum::{
    body::Empty,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use service::{system_menu_service, system_user_service};
use utils::{extracts::ValidatorJson, paginate::PaginateParams, password::Password};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/user", get(index))
        .route("/user/:id", get(info))
        .route("/user", post(create))
        .route("/user/:id", put(update))
        .route("/user/:id", delete(del))
        .route("/user/update_password", put(update_password))
        .route("/user/get_menu", get(get_menu))
        .route("/user/get_user_permission", get(get_user_permission))
        .with_state(state)
}
/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        system_user_service::paginate(&state.db, params.into()).await?,
    ))
}

/// 详情
async fn info(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse> {
    Ok(Json(system_user_service::info(&state.db, id).await?))
}

/// 新增
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    system_user_service::create(&state.db, &params.username.clone(), params.into()).await?;
    Ok(Empty::new())
}

/// 更新
async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    system_user_service::update(
        &state.db,
        id,
        Into::<system_user_service::UpdateParams>::into(params).to_params(),
    )
    .await?;
    Ok(Empty::new())
}

/// 删除
async fn del(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse> {
    system_user_service::delete(&state.db, id).await?;
    Ok(Empty::new())
}

/// 修改密码
async fn update_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatorJson(params): ValidatorJson<UpdatePasswordRequest>,
) -> Result<impl IntoResponse> {
    let info = system_user_service::info(&state.db, claims.user_id).await?;
    if !Password::verify_password(info.password(), info.salt(), params.old_password.as_bytes())? {
        return Err(crate::error::ErrorCode::Other("旧密码错误"));
    }
    system_user_service::update(
        &state.db,
        claims.user_id,
        Into::<system_user_service::UpdatePasswordParams>::into(params).to_params(),
    )
    .await?;
    Ok(Empty::new())
}

/// 获取当前用户角色菜单
async fn get_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        system_menu_service::get_user_slide_menu_trees(
            &state.db,
            claims.user_id,
            &system_menu_service::SearchParams::new(
                None,
                Some(vec![
                    system_menu_service::MenuType::Menu,
                    system_menu_service::MenuType::Redirect,
                    system_menu_service::MenuType::Iframe,
                    system_menu_service::MenuType::Link,
                ]),
            ),
        )
        .await?,
    ))
}

/// 获取当前用户权限
async fn get_user_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let info = system_user_service::get_current_user_info(&state.db, claims.user_id).await?;
    let btn_auths = system_menu_service::filter_menu_types(
        Some(vec![system_menu_service::MenuType::BtnAuth]),
        system_menu_service::get_menu_by_role(&state.db, info.role().clone()).await?,
    )
    .into_iter()
    .map(|x| x.btn_auth)
    .collect::<Vec<String>>();

    Ok(Json(UserPermission { info, btn_auths }))
}
#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for system_user_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(
            value.keyword,
            value.status,
            value.role_id,
            value.dept_id,
            value.paginate,
        )
    }
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
    status: i32,
}

impl From<CreateRequest> for system_user_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        let mut data = Self {
            nickname: Some(value.nickname),
            role_id: value.role_id,
            dept_id: value.dept_id,
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
                utils::password::Password::generate_hash_salt(password.as_bytes()).unwrap();
            data.password = Some(encode_password);
            data.salt = Some(salt);
        }
        if let Some(expire_time) = value.expire_time {
            data.expire_time = Some(Some(utils::datetime::parse_string(expire_time)))
        }
        data
    }
}

impl From<CreateRequest> for system_user_service::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        let mut data = Self {
            username: Some(value.username),
            nickname: Some(value.nickname),
            role_id: value.role_id,
            dept_id: value.dept_id,
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
                utils::password::Password::generate_hash_salt(password.as_bytes()).unwrap();
            data.password = Some(encode_password);
            data.salt = Some(salt);
        }
        if let Some(expire_time) = value.expire_time {
            data.expire_time = Some(Some(utils::datetime::parse_string(expire_time)))
        }
        data
    }
}

#[derive(Debug, Deserialize, Validate)]
struct UpdatePasswordRequest {
    #[validate(length(min = 6, message = "旧密码最小长度不能小于6位字符"))]
    old_password: String,
    #[validate(length(min = 1, message = "旧密码最小长度不能小于6位字符"))]
    new_password: String,
    #[validate(must_match(other = "new_password", message = "确认新密码与新密码不一致"))]
    confirm_password: String,
}

impl From<UpdatePasswordRequest> for system_user_service::UpdatePasswordParams {
    fn from(value: UpdatePasswordRequest) -> Self {
        let mut data = Self {
            password: None,
            salt: None,
        };

        let (encode_password, salt) =
            utils::password::Password::generate_hash_salt(value.new_password.as_bytes()).unwrap();
        data.password = Some(encode_password);
        data.salt = Some(salt);
        data
    }
}

#[derive(Debug, Serialize)]
struct UserPermission {
    info: system_user_service::Info,
    btn_auths: Vec<String>,
}
