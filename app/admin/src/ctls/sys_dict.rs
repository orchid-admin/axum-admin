use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{
    body::Empty,
    extract::{self, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_dict_service;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/dict", get(index))
        .route("/dict/all", get(all))
        .route("/dict/:id", get(info))
        .route("/dict", post(create))
        .route("/dict/:id", put(update))
        .route("/dict/:id", delete(del))
        .with_state(state)
}

/// get all dict data
async fn all(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let data = system_dict_service::all(&state.db).await?;
    Ok(Json(data))
}

/// dict list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = system_dict_service::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// dict detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_dict_service::info(&state.db, id).await?))
}

/// create dict
async fn create(
    State(state): State<AppState>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    if system_dict_service::get_by_sign(&state.db, &params.sign, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::DictSignExsist);
    }
    system_dict_service::create(
        &state.db,
        &params.name.clone(),
        &params.sign.clone(),
        params.into(),
    )
    .await?;
    Ok(Empty::new())
}

/// update dict
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    if system_dict_service::get_by_sign(&state.db, &params.sign, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::DictSignExsist);
    }
    system_dict_service::update(&state.db, id, params.into()).await?;
    Ok(Empty::new())
}

/// delete dict
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    let info = system_dict_service::info(&state.db, id).await?;
    if !info.data_is_empty() {
        return Err(ErrorCode::NotDeleteData);
    }
    system_dict_service::delete(&state.db, id).await?;
    Ok(Empty::new())
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for system_dict_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.keyword, value.status, value.paginate)
    }
}
#[derive(Debug, Deserialize)]
struct CreateRequest {
    name: String,
    sign: String,
    remark: Option<String>,
    #[serde(default)]
    status: i32,
}

impl From<CreateRequest> for system_dict_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            remark: value.remark,
            status: Some(value.status),
        }
    }
}

impl From<CreateRequest> for system_dict_service::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            name: Some(value.name),
            sign: Some(value.sign),
            remark: value.remark,
            status: Some(value.status),
        }
    }
}
