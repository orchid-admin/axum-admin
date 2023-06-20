use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_login_log_server;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/login_log", get(index))
        .route("/login_log/:id", get(info))
        .with_state(state)
}

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = system_login_log_server::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// 详情
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_login_log_server::info(&state.db, id).await?))
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    user_id: Option<i32>,
    keyword: Option<String>,
    date: Option<String>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for system_login_log_server::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.user_id, value.keyword, value.date, value.paginate)
    }
}
