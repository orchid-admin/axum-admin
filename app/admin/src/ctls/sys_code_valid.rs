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
use service::cache_service;
use utils::{extracts::ValidatorJson, paginate::PaginateParams};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/code_valid", get(index))
        .route("/code_valid/:id", get(info))
        .route("/code_valid", post(create))
        .route("/code_valid/:id", put(update))
        .route("/code_valid/:id", delete(del))
        .route("/code_valid", delete(batch_del))
        .with_state(state)
}

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = cache_service::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// 详情
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(cache_service::info(&state.db, id).await?))
}

/// 创建
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    if cache_service::get_by_type_code(&state.db, &params.r#type, &params.code, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "该类型的编号为{}已存在",
            params.code
        )));
    }
    cache_service::create(
        &state.db,
        &utils::datetime::timestamp_nanos_string(None),
        &params.code.clone(),
        params.into(),
    )
    .await?;
    Ok(Empty::new())
}

/// 更新
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    if cache_service::get_by_type_code(&state.db, &params.r#type, &params.code, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "该类型的编号为{}已存在",
            params.code
        )));
    }
    cache_service::update(&state.db, id, params.into()).await?;
    Ok(Empty::new())
}

/// 删除
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    cache_service::info(&state.db, id).await?;
    cache_service::delete(&state.db, id).await?;
    Ok(Empty::new())
}

/// 批量删除
async fn batch_del(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<BatchAction>,
) -> Result<impl IntoResponse> {
    cache_service::batch_delete(&state.db, params.ids).await?;
    Ok(Empty::new())
}

#[derive(Debug, Deserialize, Validate)]
struct BatchAction {
    ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    r#type: Option<cache_service::CodeType>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for cache_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.keyword, value.r#type, value.paginate)
    }
}
#[derive(Debug, Deserialize, Validate)]
struct CreateRequest {
    r#type: cache_service::CodeType,
    code: String,
    attach: String,
    valid_time: String,
}

impl From<CreateRequest> for cache_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            r#type: Some(value.r#type.into()),
            attach: Some(value.attach),
            valid_time: Some(utils::datetime::parse_string(value.valid_time)),
        }
    }
}

impl From<CreateRequest> for cache_service::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            r#type: Some(value.r#type.into()),
            attach: Some(value.attach),
            valid_time: Some(utils::datetime::parse_string(value.valid_time)),
        }
    }
}
