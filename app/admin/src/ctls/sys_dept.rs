use super::Claims;
use crate::{error::Result, state::AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_dept;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/dept", get(index))
        .route("/dept/:id", get(info))
        .route("/dept", post(create))
        .route("/dept/update", post(update))
        .route("/dept/:id", get(del))
        .with_state(state)
}
/// get tree dept
async fn index(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(mut params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    params.user_id = Some(claims.user_id);
    Ok(Json(
        system_dept::get_user_dept_trees(&state.db, params.into()).await?,
    ))
}

/// dept detail
async fn info(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    Ok(Json(system_dept::info(&state.db, id).await?))
}

/// create dept
async fn create(
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    system_dept::create(&state.db, params.into()).await?;
    Ok(Body::empty())
}

/// update dept
async fn update(
    State(state): State<AppState>,
    Json(params): Json<RequestFormUpdate>,
) -> Result<impl IntoResponse> {
    system_dept::update(&state.db, params.id, params.form.into()).await?;
    Ok(Body::empty())
}

/// delete dept
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    system_dept::delete(&state.db, id).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    status: Option<i32>,
    user_id: Option<i32>,
}
impl From<RequestSearch> for system_dept::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            keyword: value.keyword,
            status: value.status,
            user_id: value.user_id,
            ..Default::default()
        }
    }
}
#[derive(Debug, Deserialize)]
struct RequestFormCreate {
    parent_id: i32,
    name: String,
    person_name: Option<String>,
    person_phone: Option<String>,
    person_email: Option<String>,
    describe: Option<String>,
    status: i32,
    sort: i32,
}

impl From<RequestFormCreate> for system_dept::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        Self {
            parent_id: value.parent_id,
            name: value.name,
            person_name: value.person_name.unwrap_or_default(),
            person_phone: value.person_phone.unwrap_or_default(),
            person_email: value.person_email.unwrap_or_default(),
            describe: value.describe.unwrap_or_default(),
            status: value.status,
            sort: value.sort,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RequestFormUpdate {
    id: i32,
    #[serde(flatten)]
    form: RequestFormCreate,
}
