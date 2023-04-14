use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use service::sys_menu::{self, MenuCreateParams};
use ts_rs::TS;
use utoipa::{Path, ToSchema};
use validator::Validate;

use crate::{error::Result, extracts::ValidatorJson, openapi::DocmentPathSchema, state::AppState};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/menu/index", get(index))
        .route("/menu/info/:id", get(info))
        .route("/menu/create", post(create))
        .with_state(state)
}

pub fn api_docment() -> DocmentPathSchema {
    let paths = crate::api_doc_path! {
        __path_index,
        __path_info,
        __path_create
    };
    let schemas = crate::api_doc_schema! {
        IndexResponse
    };
    (paths, schemas)
}
/// 列表
#[utoipa::path(
    get,
    path = "/menu/index",
    tag = "menu",
    responses(
        (status = 200, body = [IndexResponse])
    )
)]
async fn index() -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

/// 详情
#[utoipa::path(
    get,
    path = "/menu/info/:id",
    tag = "menu",
    responses(
        (status = 200, body = [IndexResponse])
    )
)]
async fn info(extract::Path(_id): extract::Path<i64>) -> Result<impl IntoResponse> {
    Ok(Json(IndexResponse {}))
}

/// 新增
#[utoipa::path(
    get,
    path = "/menu/create",
    tag = "menu",
    responses(
        (status = 200, body = IndexResponse)
    )
)]
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<MenuCreateRequest>,
) -> Result<impl IntoResponse> {
    let create_params = MenuCreateParams {
        parent_id: params.parent_id,
        r#type: params.r#type,
        router_name: params.router_name,
        component_alias: params.component_alias,
        is_link: params.is_link,
        path: params.path,
        redirect: params.redirect,
        btn_power: params.btn_power,
        sort: params.sort,
        title: params.meta.title,
        icon: params.meta.icon,
        is_hide: params.meta.is_hide,
        is_keep_alive: params.meta.is_keep_alive,
        is_affix: params.meta.is_affix,
        link: params.meta.link,
        is_iframe: params.meta.is_iframe,
    };
    sys_menu::create(state.db.clone(), create_params).await?;
    Ok("")
}

#[derive(Debug, Serialize, ToSchema)]
struct IndexResponse {}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate, ToSchema, TS)]
#[ts(export)]
struct MenuCreateRequest {
    parent_id: Option<i32>,
    #[validate(length(min = 2, message = "类型长度错误"))]
    #[serde(rename = "menuType")]
    r#type: String,
    #[serde(rename = "name")]
    router_name: String,
    #[serde(rename = "componentAlias")]
    component_alias: String,
    #[serde(rename = "isLink")]
    is_link: bool,
    path: String,
    redirect: String,
    #[serde(rename = "btnPower")]
    btn_power: String,
    #[serde(rename = "menuSort")]
    sort: i32,
    meta: MenuMeta,
}

#[derive(Debug, Deserialize, Validate, ToSchema, TS)]
#[ts(export)]
struct MenuMeta {
    title: String,
    icon: String,
    #[serde(rename = "isHide")]
    is_hide: bool,
    #[serde(rename = "isKeepAlive")]
    is_keep_alive: bool,
    #[serde(rename = "isAffix")]
    is_affix: bool,
    #[serde(rename = "isLink")]
    link: String,
    #[serde(rename = "isIframe")]
    is_iframe: bool,
    roles: Vec<String>,
}
