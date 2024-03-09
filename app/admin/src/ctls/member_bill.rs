use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::member_bill;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/member_bill", get(index))
        .route("/member_bill/:id", get(info))
        .with_state(state)
}

/// member bill list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = member_bill::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// member bill detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(member_bill::info(&state.db, id).await?))
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    r#type: Option<i32>,
    pm: Option<i32>,
    date: Option<String>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for member_bill::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            keyword: value.keyword,
            r#type: value.r#type,
            pm: value.pm,
            date: value.date,
            paginate: value.paginate,
        }
    }
}
