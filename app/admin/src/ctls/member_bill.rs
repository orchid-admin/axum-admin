use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::member_bill_service;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/member_bill", get(index))
        .route("/member_bill/:id", get(info))
        .with_state(state)
}

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = member_bill_service::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// 详情
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(member_bill_service::info(&state.db, id).await?))
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    r#type: Option<member_bill_service::BillType>,
    pm: Option<member_bill_service::BillPm>,
    keyword: Option<String>,
    date: Option<String>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for member_bill_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(
            value.r#type,
            value.pm,
            value.keyword,
            value.date,
            value.paginate,
        )
    }
}
