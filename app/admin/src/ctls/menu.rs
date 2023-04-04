use axum::{extract, routing::get, Json, Router};
use serde::Serialize;
use utoipa::{Path, ToSchema};

use crate::error::Result;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/menu/index", get(index))
        .route("/menu/info/:id", get(info))
        .with_state(state)
}

pub fn api_docment() -> (
    Vec<(&'static str, utoipa::openapi::PathItem)>,
    Vec<(
        &'static str,
        utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    )>,
) {
    let paths = crate::api_doc_path! {
        __path_index,
        __path_info
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
async fn create(params: CreateRequest) -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

#[derive(Debug, Serialize, ToSchema)]
struct IndexResponse {}

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
