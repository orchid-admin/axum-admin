use super::Claims;
use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{
    body::Empty,
    extract::{self, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::{sys_menu, sys_role};
use utils::{extracts::ValidatorJson, paginate::PaginateParams};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/role", get(index))
        .route("/role/all", get(all))
        .route("/role/:id", get(info))
        .route("/role", post(create))
        .route("/role/:id", put(update))
        .route("/role/:id", delete(del))
        .with_state(state)
}

/// 获取所有
async fn all(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let data = sys_role::all(&state.db).await?;
    Ok(Json(data))
}

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = sys_role::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// 详情
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(sys_role::info(&state.db, id).await?))
}

/// 创建
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    if sys_role::get_by_sign(&state.db, &params.sign, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::OtherString(format!(
            "标识为{}的角色已存在",
            params.sign
        )));
    }
    let user_menus = sys_menu::get_user_menus_by_menu_ids(
        &state.db,
        claims.user_id,
        params.menu_ids.clone().unwrap_or_default(),
    )
    .await?;
    sys_role::create(
        &state.db,
        &params.name.clone(),
        &params.sign.clone(),
        params.into(),
        user_menus,
    )
    .await?;
    Ok(Empty::new())
}

/// 更新
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ValidatorJson(params): ValidatorJson<CreateRequest>,
) -> Result<impl IntoResponse> {
    match sys_role::get_by_sign(&state.db, &params.sign, Some(id)).await? {
        Some(_) => {
            return Err(ErrorCode::OtherString(format!(
                "标识为{}的角色已存在",
                params.sign
            )));
        }
        None => {
            let info = sys_role::info(&state.db, id).await?;
            if info.get_sign().eq(&state.db.config().get_admin_role_sign()) {
                return Err(ErrorCode::Other("不可编辑系统管理员"));
            }
        }
    }
    let user_menus = sys_menu::get_user_menus_by_menu_ids(
        &state.db,
        claims.user_id,
        params.menu_ids.clone().unwrap_or_default(),
    )
    .await?;
    sys_role::update(&state.db, id, params.into(), user_menus).await?;
    Ok(Empty::new())
}

/// 删除
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    let info = sys_role::info(&state.db, id).await?;
    if info.get_sign().eq(&state.db.config().get_admin_role_sign()) {
        return Err(ErrorCode::Other("不可删除系统管理员"));
    }
    sys_role::delete(&state.db, id).await?;
    Ok(Empty::new())
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    status: Option<bool>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for sys_role::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.keyword, value.status, value.paginate)
    }
}
#[derive(Debug, Deserialize, Validate)]
struct CreateRequest {
    name: String,
    sign: String,
    #[serde(default)]
    sort: i32,
    #[serde(default)]
    describe: String,
    #[serde(default)]
    status: bool,
    menu_ids: Option<Vec<i32>>,
}

impl From<CreateRequest> for sys_role::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            sort: Some(value.sort),
            describe: Some(value.describe),
            status: Some(value.status),
        }
    }
}

impl From<CreateRequest> for sys_role::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            name: Some(value.name),
            sign: Some(value.sign),
            sort: Some(value.sort),
            describe: Some(value.describe),
            status: Some(value.status),
        }
    }
}
