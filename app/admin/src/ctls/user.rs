use super::Claims;
use crate::{error::Result, state::AppState};
use axum::{extract::State, response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;
use service::{
    sys_menu::{self, MenuType},
    sys_user,
};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/user", get(index))
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
        sys_menu::get_user_slide_menu_trees(
            &state.db,
            claims.user_id,
            Some(vec![MenuType::Menu, MenuType::Redirect, MenuType::Iframe]),
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
        username: user_permission.user.username,
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

#[derive(Debug, Serialize)]
struct IndexResponse {}

#[derive(Debug, Serialize)]
struct UserPermission {
    username: String,
    photo: Option<String>,
    time: i64,
    roles: Vec<String>,
    btn_auths: Vec<String>,
}
