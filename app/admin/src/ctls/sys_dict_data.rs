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
use service::system_dict_data;
use utils::paginate::PaginateParams;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/dict_data", get(index))
        .route("/dict_data/:id", get(info))
        .route("/dict_data", post(create))
        .route("/dict_data/:id", put(update))
        .route("/dict_data/:id", delete(del))
        .route("/dict_data", delete(batch_del))
        .with_state(state)
}

/// dict data list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = system_dict_data::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// dict data detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_dict_data::info(&state.db, id).await?))
}

/// create dict data
async fn create(
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if system_dict_data::get_by_label(&state.db, params.dict_id, &params.label, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::DictDataLableExsist);
    }
    system_dict_data::create(&state.db, params.into()).await?;
    Ok(Body::empty())
}

/// update dict data
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if system_dict_data::get_by_label(&state.db, params.dict_id, &params.label, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::DictDataLableExsist);
    }
    system_dict_data::update(&state.db, id, params.into()).await?;
    Ok(Body::empty())
}

/// delete dict data
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    system_dict_data::info(&state.db, id).await?;
    system_dict_data::delete(&state.db, id).await?;
    Ok(Body::empty())
}

/// batch delete dict data
async fn batch_del(
    State(state): State<AppState>,
    Json(params): Json<RequestFormBatchAction>,
) -> Result<impl IntoResponse> {
    system_dict_data::batch_delete(&state.db, params.ids).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct RequestFormBatchAction {
    ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    dict_id: Option<i32>,
    keyword: Option<String>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for system_dict_data::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            dict_id: value.dict_id,
            keyword: value.keyword,
            status: value.status,
            paginate: value.paginate,
        }
    }
}
#[derive(Debug, Deserialize)]
struct RequestFormCreate {
    dict_id: i32,
    label: String,
    value: i32,
    remark: Option<String>,
    #[serde(default)]
    status: i32,
    sort: i32,
}

impl From<RequestFormCreate> for system_dict_data::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        Self {
            dict_id: value.dict_id,
            label: value.label,
            value: value.value,
            remark: value.remark.unwrap_or_default(),
            status: value.status,
            sort: value.sort,
        }
    }
}
