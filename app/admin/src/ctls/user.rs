use super::Claims;
use crate::{
    error::{ErrorCode, Result},
    openapi::DocmentPathSchema,
    state::AppState,
};
use axum::{extract::State, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_macros::debug_handler;
use serde::Serialize;
use service::{
    sys_menu::{self, MenuTreeInfo},
    sys_user,
};
use utoipa::{Path, ToSchema};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/user/index", get(index))
        .route("/user/get_menu", get(get_menu))
        .route("/user/get_user_permission", get(get_user_permission))
        .with_state(state)
}

pub fn api_docment() -> DocmentPathSchema {
    let paths = crate::api_doc_path! {
        __path_index,
        __path_get_menu,
        __path_get_user_permission
    };
    let schemas = crate::api_doc_schema! {
        IndexResponse,
        UserPermission,
        MenuTreeInfo
    };
    (paths, schemas)
}
/// 列表
#[utoipa::path(
    get,
    path = "/user/index",
    tag = "用户管理",
    responses(
        (status = 200, body = [IndexResponse])
    )
)]
async fn index() -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

/// 获取当前用户角色菜单
#[utoipa::path(
    get,
    path = "/user/get_menu",
    tag = "用户管理",
    responses(
        (status = 200, body = Vec<MenuTreeInfo>)
    )
)]
async fn get_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<impl Serialize>> {
    Ok(Json(
        sys_menu::get_user_menu(&state.db, claims.user_id).await?,
    ))
}

/// 获取当前用户权限
#[utoipa::path(
    get,
    path = "/user/get_user_permission",
    tag = "用户管理",
    responses(
        (status = 200, body = CurrentUserInfo),
        (status = 500, body = ErrorCode, example = json!(ErrorCode::Unauthorized.to_json_string()))
    )
)]
#[debug_handler]
async fn get_user_permission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    match sys_user::get_current_user_info(&state.db, claims.user_id).await? {
        Some(permission) => Ok(Json(UserPermission {
            username: permission.user.username,
            photo: None,
            time: 0,
            roles: match permission.role {
                Some(role) => vec![role.sign],
                None => vec!["admin".to_owned()],
            },
            // todo
            auth_btn_list: vec![],
        })),
        None => Err(ErrorCode::Unauthorized),
    }
}

#[derive(Debug, Serialize, ToSchema)]
struct IndexResponse {}

#[derive(Debug, Serialize, ToSchema)]
struct UserPermission {
    #[serde(rename = "userName")]
    username: String,
    photo: Option<String>,
    time: i64,
    roles: Vec<String>,
    #[serde(rename = "authBtnList")]
    auth_btn_list: Vec<String>,
}
