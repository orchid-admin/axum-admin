use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_login_log;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/login_log", get(index))
        .route("/login_log/:id", get(info))
        .with_state(state)
}

/// login_log list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = system_login_log::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// login_log detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_login_log::info(&state.db, id).await?))
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    user_id: Option<i32>,
    keyword: Option<String>,
    date: Option<String>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for system_login_log::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            user_id: value.user_id,
            keyword: value.keyword,
            date: value.date,
            paginate: value.paginate,
        }
    }
}
