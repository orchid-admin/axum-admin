use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{Path, ToSchema};

use crate::{error::Result, openapi::DocmentPathSchema};

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
///
///
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
///
///
#[utoipa::path(
    get,
    path = "/menu/info/:id",
    tag = "menu",
    responses(
        (status = 200, body = [IndexResponse])
    )
)]
async fn info(extract::Path(_id): extract::Path<i64>) -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

/// 新增
///
///
#[utoipa::path(
    get,
    path = "/menu/create",
    tag = "menu",
    responses(
        (status = 200, body = [IndexResponse])
    )
)]
async fn create(Json(_params): Json<CreateRequest>) -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

#[derive(Debug, Serialize, ToSchema)]
struct IndexResponse {}

#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
struct CreateRequest {
    parent_id: Option<i64>,
    r#type: String,
    router_name: String,
    component_alias: String,
    is_link: bool,
    path: String,
    redirect: String,
    meta_title: String,
    meta_icon: String,
    meta_is_hide: bool,
    meta_is_keep_alive: bool,
    meta_is_affix: bool,
    meta_link: String,
    meta_is_iframe: bool,
    meta_roles: String,
    btn_power: String,
    sort: i64,
}
