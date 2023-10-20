use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_action_log_service;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/action_log", get(index))
        .route("/action_log/:id", get(info))
        .with_state(state)
}

/// action log list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = system_action_log_service::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// action log detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_action_log_service::info(&state.db, id).await?))
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    user_id: Option<i32>,
    menu_id: Option<i32>,
    keyword: Option<String>,
    date: Option<String>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for system_action_log_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(
            value.user_id,
            value.menu_id,
            value.keyword,
            value.date,
            value.paginate,
        )
    }
}
