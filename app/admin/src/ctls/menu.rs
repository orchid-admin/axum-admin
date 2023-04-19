use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Deserialize;
use service::sys_menu::{self, MenuCreateParams, MenuInfo, MenuInfoMeta};
use ts_rs::TS;
use utoipa::{Path, ToSchema};
use validator::Validate;

use crate::{error::Result, extracts::ValidatorJson, openapi::DocmentPathSchema, state::AppState};

use super::Claims;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/menu/index", get(tree))
        .route("/menu/create", post(create))
        .with_state(state)
}

pub fn api_docment() -> DocmentPathSchema {
    let paths = crate::api_doc_path! {
        __path_create,
        __path_tree
    };
    let schemas = crate::api_doc_schema! {
        MenuInfo,
        MenuInfoMeta
    };
    (paths, schemas)
}

/// 新增
#[utoipa::path(get, path = "/menu/create", tag = "菜单")]
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<MenuCreateRequest>,
) -> Result<impl IntoResponse> {
    sys_menu::create(
        state.db.clone(),
        params.meta.title.clone(),
        params.to_partial_unchecked(),
    )
    .await?;
    Ok("")
}

/// 获取树形
#[utoipa::path(
    get,
    path = "/menu/tree",
    tag = "菜单",
    responses(
        (status = 200, body = Vec<MenuInfo>)
    )
)]
async fn tree(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse> {
    let data = sys_menu::get_user_menu(state.db.clone(), claims.user_id).await?;
    Ok(Json(data))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate, ToSchema, TS)]
#[ts(export)]
struct MenuCreateRequest {
    parent_id: Option<i32>,
    #[validate(length(min = 2, message = "类型长度错误"))]
    #[serde(rename = "menuType")]
    #[validate(required)]
    r#type: Option<String>,
    #[serde(rename = "name")]
    router_name: Option<String>,
    #[serde(rename = "componentAlias")]
    component: Option<String>,
    #[serde(rename = "isLink")]
    is_link: Option<bool>,
    path: Option<String>,
    redirect: Option<String>,
    #[serde(rename = "btnPower")]
    btn_power: Option<String>,
    #[serde(rename = "menuSort")]
    #[validate(required)]
    sort: Option<i32>,
    meta: MenuMeta,
}

#[derive(Debug, Deserialize, Validate, ToSchema, TS)]
#[ts(export)]
struct MenuMeta {
    title: String,
    icon: Option<String>,
    #[serde(rename = "isHide")]
    is_hide: Option<bool>,
    #[serde(rename = "isKeepAlive")]
    is_keep_alive: Option<bool>,
    #[serde(rename = "isAffix")]
    is_affix: Option<bool>,
    #[serde(rename = "isLink")]
    link: Option<String>,
    #[serde(rename = "isIframe")]
    is_iframe: Option<bool>,
}

impl MenuCreateRequest {
    fn to_partial_unchecked(self) -> MenuCreateParams {
        MenuCreateParams {
            parent_id: self.parent_id,
            r#type: self.r#type,
            router_name: self.router_name,
            component: self.component,
            is_link: self.is_link,
            path: self.path,
            redirect: self.redirect,
            btn_power: self.btn_power,
            sort: self.sort,
            meta_icon: self.meta.icon,
            meta_is_hide: self.meta.is_hide,
            meta_is_keep_alive: self.meta.is_keep_alive,
            meta_is_affix: self.meta.is_affix,
            meta_link: self.meta.link,
            meta_is_iframe: self.meta.is_iframe,
        }
    }
}
