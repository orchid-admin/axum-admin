use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

use crate::error::Result;

use super::CtlRouter;

pub struct User;

impl CtlRouter for User {
    fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
        Router::new()
            .route("/user/index", get(Self::index))
            .route("/user/info/:id", get(Self::info))
            .with_state(state)
    }
}
impl User {
    async fn index() -> Result<Json<impl Serialize>> {
        Ok(Json(IndexResponse {}))
    }

    async fn info(Path(id): Path<i64>) -> Result<Json<impl Serialize>> {
        Ok(Json(IndexResponse {}))
    }
}

#[derive(Debug, Serialize)]
struct IndexResponse {}
