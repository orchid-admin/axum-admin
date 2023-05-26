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
use service::{sys_menu, sys_user};
use utils::{extracts::ValidatorJson, paginate::PaginateParams};
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
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(sys_user::paginate(&state.db, params.into()).await?))
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
            &sys_menu::MenuSearchParams::new(
                None,
                Some(vec![
                    sys_menu::MenuType::Menu,
                    sys_menu::MenuType::Redirect,
                    sys_menu::MenuType::Iframe,
                    sys_menu::MenuType::Link,
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
    let info = sys_user::get_current_user_info(&state.db, claims.user_id).await?;
    let btn_auths = sys_menu::filter_menu_types(
        Some(vec![sys_menu::MenuType::BtnAuth]),
        sys_menu::get_menu_by_role(&state.db, info.get_role()).await?,
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
    status: Option<bool>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for sys_user::UserSearchParams {
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
    status: bool,
}

impl From<CreateRequest> for sys_user::CreateParams {
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

impl From<CreateRequest> for sys_user::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        let mut data = Self {
            username: Some(value.username),
            nickname: Some(value.nickname),
            role_id: value.role_id.map(|x| if x <= 0 { None } else { Some(x) }),
            dept_id: value.dept_id.map(|x| if x <= 0 { None } else { Some(x) }),
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

#[derive(Debug, Serialize)]
struct UserPermission {
    info: sys_user::Info,
    btn_auths: Vec<String>,
}
