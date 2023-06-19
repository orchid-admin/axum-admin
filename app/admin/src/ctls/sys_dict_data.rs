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
use service::sys_dict_data;
use utils::{extracts::ValidatorJson, paginate::PaginateParams};
use validator::Validate;

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

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = sys_dict_data::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// 详情
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(sys_dict_data::info(&state.db, id).await?))
}

/// 创建
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    if sys_dict_data::get_by_label(&state.db, params.dict_id, &params.label, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "该字典中名称为{}的已存在",
            params.label
        )));
    }
    sys_dict_data::create(
        &state.db,
        params.dict_id,
        &params.label.clone(),
        params.value,
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
    if sys_dict_data::get_by_label(&state.db, params.dict_id, &params.label, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "该字典中名称为{}的已存在",
            params.label
        )));
    }
    sys_dict_data::update(&state.db, id, params.into()).await?;
    Ok(Empty::new())
}

/// 删除
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    sys_dict_data::info(&state.db, id).await?;
    sys_dict_data::delete(&state.db, id).await?;
    Ok(Empty::new())
}

/// 批量删除
async fn batch_del(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<BatchAction>,
) -> Result<impl IntoResponse> {
    sys_dict_data::batch_delete(&state.db, params.ids).await?;
    Ok(Empty::new())
}

#[derive(Debug, Deserialize, Validate)]
struct BatchAction {
    ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    dict_id: Option<i32>,
    keyword: Option<String>,
    status: Option<bool>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for sys_dict_data::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.dict_id, value.keyword, value.status, value.paginate)
    }
}
#[derive(Debug, Deserialize, Validate)]
struct CreateRequest {
    dict_id: i32,
    label: String,
    value: i32,
    remark: Option<String>,
    #[serde(default)]
    status: bool,
    sort: i32,
}

impl From<CreateRequest> for sys_dict_data::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            remark: value.remark,
            status: Some(value.status),
        }
    }
}

impl From<CreateRequest> for sys_dict_data::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            label: Some(value.label),
            value: Some(value.value),
            remark: value.remark,
            status: Some(value.status),
            sort: Some(value.sort),
        }
    }
}
