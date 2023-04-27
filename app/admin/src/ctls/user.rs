use super::Claims;
use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{extract::State, response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;
use service::{sys_menu, sys_user};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/user/index", get(index))
        .route("/user/get_menu", get(get_menu))
        .route("/user/get_user_permission", get(get_user_permission))
        .with_state(state)
}
/// 列表
async fn index() -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

/// 获取当前用户角色菜单
async fn get_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<impl Serialize>> {
    Ok(Json(
        sys_menu::get_user_menu(&state.db, claims.user_id).await?,
    ))
}

/// 获取当前用户权限
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

#[derive(Debug, Serialize)]
struct IndexResponse {}

#[derive(Debug, Serialize)]
struct UserPermission {
    #[serde(rename = "userName")]
    username: String,
    photo: Option<String>,
    time: i64,
    roles: Vec<String>,
    #[serde(rename = "authBtnList")]
    auth_btn_list: Vec<String>,
}
