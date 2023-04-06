use axum::{extract, routing::get, Json, Router};
use serde::Serialize;
use utoipa::{Path, ToSchema};

use crate::{error::Result, openapi::DocmentPathSchema};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/role/index", get(index))
        .route("/role/info/:id", get(info))
        .with_state(state)
}

pub fn api_docment() -> DocmentPathSchema {
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
    path = "/role/index",
    tag = "role",
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
    path = "/role/info/:id",
    tag = "role",
    responses(
        (status = 200, body = [IndexResponse])
    )
)]
async fn info(extract::Path(_id): extract::Path<i64>) -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

#[derive(Debug, Serialize, ToSchema)]
struct IndexResponse {}
