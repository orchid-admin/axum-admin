use axum::{extract, routing::get, Json, Router};
use serde::Serialize;

use crate::error::Result;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/role/index", get(index))
        .route("/role/info/:id", get(info))
        .with_state(state)
}

/// 列表
async fn index() -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

/// 详情
async fn info(extract::Path(_id): extract::Path<i64>) -> Result<Json<impl Serialize>> {
    Ok(Json(IndexResponse {}))
}

#[derive(Debug, Serialize)]
struct IndexResponse {}
