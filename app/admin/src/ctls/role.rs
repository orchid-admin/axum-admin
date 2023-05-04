use super::Claims;
use crate::{
    error::{ErrorCode, Result},
    extracts::ValidatorJson,
    state::AppState,
};
use axum::{
    body::Empty,
    extract::{self, Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use service::{sys_menu, sys_role};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/role", get(index))
        .route("/role/:id", get(info))
        .route("/role", post(create))
        .route("/role/:id", put(update))
        .route("/role/:id", delete(del))
        .with_state(state)
}

/// 列表
async fn index(
    State(state): State<AppState>,
    Query(params): Query<sys_role::RoleSearchParams>,
) -> Result<impl IntoResponse> {
    let data = sys_role::paginate(&state.db, params).await?;
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
        params.menus.clone().unwrap_or_default(),
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
            if info.sign.eq(service::ADMIN_ROLE_SIGN) {
                return Err(ErrorCode::Other("不可编辑系统管理员"));
            }
        }
    }
    let user_menus = sys_menu::get_user_menus_by_menu_ids(
        &state.db,
        claims.user_id,
        params.menus.clone().unwrap_or_default(),
    )
    .await?;
    sys_role::update(&state.db, id, params.into(), user_menus).await?;
    Ok(Empty::new())
}

/// 删除
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    let info = sys_role::info(&state.db, id).await?;
    if info.sign.eq(service::ADMIN_ROLE_SIGN) {
        return Err(ErrorCode::Other("不可删除系统管理员"));
    }
    sys_role::delete(&state.db, id).await?;
    Ok(Empty::new())
}

#[derive(Debug, Serialize)]
struct IndexResponse {}

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
    menus: Option<Vec<i32>>,
}

impl From<CreateRequest> for sys_role::RoleCreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            sort: Some(value.sort),
            describe: Some(value.describe),
            status: Some(value.status),
        }
    }
}

impl From<CreateRequest> for sys_role::RoleUpdateParams {
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
