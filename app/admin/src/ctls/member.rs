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
use service::member_service;
use utils::{extracts::ValidatorJson, paginate::PaginateParams};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/member", get(index))
        .route("/member/:id", get(info))
        .route("/member", post(create))
        .route("/member/:id", put(update))
        .route("/member/:id", delete(del))
        .with_state(state)
}

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = member_service::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// 详情
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(member_service::info(&state.db, id).await?))
}

/// 创建
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    if member_service::get_by_email(&state.db, &params.email, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "邮箱地址为{}的用户已存在",
            params.email
        )));
    }
    let unique_code = member_service::generate_code(&state.db, 8).await?;
    member_service::create(
        &state.db,
        &unique_code,
        &params.email.clone(),
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
    if member_service::get_by_email(&state.db, &params.email, Some(id))
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "邮箱地址为{}的用户已存在",
            params.email
        )));
    }
    member_service::update(&state.db, id, params.into()).await?;
    Ok(Empty::new())
}

/// 删除
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    member_service::info(&state.db, id).await?;
    member_service::delete(&state.db, id).await?;
    Ok(Empty::new())
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    sex: Option<i32>,
    status: Option<bool>,
    is_promoter: Option<bool>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for member_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(
            value.keyword,
            value.sex,
            value.status,
            value.is_promoter,
            value.paginate,
        )
    }
}
#[derive(Debug, Deserialize, Validate)]
struct CreateRequest {
    email: String,
    mobile: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
    password: Option<String>,
    #[serde(default)]
    sex: Option<i32>,
    #[serde(default)]
    balance: Option<f64>,
    #[serde(default)]
    integral: Option<i32>,
    remark: Option<String>,
    #[serde(default)]
    status: Option<bool>,
    #[serde(default)]
    is_promoter: Option<bool>,
}

impl From<CreateRequest> for member_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            mobile: value.mobile,
            nickname: value.nickname,
            avatar: value.avatar,
            password: value.password,
            sex: value.sex,
            balance: value.balance,
            integral: value.integral,
            remark: value.remark,
            status: value.status,
            is_promoter: value.is_promoter,
        }
    }
}

impl From<CreateRequest> for member_service::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            email: Some(value.email),
            mobile: value.mobile,
            nickname: value.nickname,
            avatar: value.avatar,
            password: value.password,
            sex: value.sex,
            balance: value.balance,
            integral: value.integral,
            remark: value.remark,
            status: value.status,
            is_promoter: value.is_promoter,
        }
    }
}
