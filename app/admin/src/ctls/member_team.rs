use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::member_team;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/member_team", get(index))
        .route("/member_team/:id", get(info))
        .with_state(state)
}

/// member team list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = member_team::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// member team detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(member_team::info(&state.db, id).await?))
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    date: Option<String>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for member_team::Filter {
    fn from(value: RequestSearch) -> Self {
        Self::new(value.keyword, value.date, value.paginate)
    }
}
