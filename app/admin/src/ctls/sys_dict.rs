use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{
    body::Body,
    extract::{self, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_dict;
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
    let data = system_dict::all(&state.db).await?;
    Ok(Json(data))
}

/// dict list
#[axum_macros::debug_handler]
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = system_dict::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// dict detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_dict::info(&state.db, id).await?))
}

/// create dict
async fn create(
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if system_dict::get_by_sign(&state.db, &params.sign, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::DictSignExsist);
    }
    system_dict::create(&state.db, params.into()).await?;
    Ok(Body::empty())
}

/// update dict
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if system_dict::get_by_sign(&state.db, &params.sign, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::DictSignExsist);
    }
    system_dict::update(&state.db, id, params.into()).await?;
    Ok(Body::empty())
}

/// delete dict
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    let _ = system_dict::info(&state.db, id).await?;
    system_dict::delete(&state.db, id).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for system_dict::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            keyword: value.keyword,
            status: value.status,
            paginate: value.paginate,
        }
    }
}
#[derive(Debug, Deserialize)]
struct RequestFormCreate {
    name: String,
    sign: String,
    remark: Option<String>,
    #[serde(default)]
    status: i32,
}

impl From<RequestFormCreate> for system_dict::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        Self {
            name: value.name,
            sign: value.sign,
            remark: value.remark.unwrap_or_default(),
            status: value.status,
        }
    }
}
