use super::Claims;
use crate::{error::Result, state::AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_dept_service;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/dept", get(index))
        .route("/dept/:id", get(info))
        .route("/dept", post(create))
        .route("/dept/:id", put(update))
        .route("/dept/:id", delete(del))
        .with_state(state)
}
/// get tree dept
async fn index(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        system_dept_service::get_user_dept_trees(&state.db, claims.user_id, &params.into()).await?,
    ))
}

/// dept detail
async fn info(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    Ok(Json(system_dept_service::info(&state.db, id).await?))
}

/// create dept
async fn create(
    State(state): State<AppState>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    system_dept_service::create(&state.db, &params.name.clone(), params.into()).await?;
    Ok(Body::empty())
}

/// update dept
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    system_dept_service::update(&state.db, id, params.into()).await?;
    Ok(Body::empty())
}

/// delete dept
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    system_dept_service::delete(&state.db, id).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    status: Option<i32>,
}
impl From<SearchRequest> for system_dept_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.keyword, value.status)
    }
}
#[derive(Debug, Deserialize)]
struct CreateRequest {
    parent_id: i32,
    name: String,
    person_name: Option<String>,
    person_phone: Option<String>,
    person_email: Option<String>,
    describe: Option<String>,
    status: i32,
    sort: i32,
}

impl From<CreateRequest> for system_dept_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            parent_id: Some(value.parent_id),
            person_name: value.person_name,
            person_phone: value.person_phone,
            person_email: value.person_email,
            describe: value.describe,
            status: Some(value.status),
            sort: Some(value.sort),
        }
    }
}
impl From<CreateRequest> for system_dept_service::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            parent_id: Some(value.parent_id),
            name: Some(value.name),
            person_name: value.person_name,
            person_phone: value.person_phone,
            person_email: value.person_email,
            describe: value.describe,
            status: Some(value.status),
            sort: Some(value.sort),
        }
    }
}
