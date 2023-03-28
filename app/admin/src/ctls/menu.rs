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

#[derive(Debug, Serialize, ToSchema)]
struct IndexResponse {}
