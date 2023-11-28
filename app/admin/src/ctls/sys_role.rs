use super::Claims;
use crate::{
    error::{ErrorCode, Result},
    state::AppState,
};
use axum::{
    body::Body,
    extract::{self, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::{system_menu_service, system_role_service};
use utils::paginate::PaginateParams;

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

/// get all role
async fn all(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let data = system_role_service::all(&state.db).await?;
    Ok(Json(data))
}

/// role list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    let data = system_role_service::paginate(&state.db, &params.into()).await?;
    Ok(Json(data))
}

/// role detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_role_service::info(&state.db, id).await?))
}

/// create role
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    if system_role_service::get_by_sign(&state.db, &params.sign, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::RoleSignExsist);
    }
    let user_menus = system_menu_service::get_user_menus_by_menu_ids(
        &state.db,
        claims.user_id,
        params.menu_ids.clone().unwrap_or_default(),
    )
    .await?;
    system_role_service::create(
        &state.db,
        &params.name.clone(),
        &params.sign.clone(),
        params.into(),
        user_menus,
    )
    .await?;
    Ok(Body::empty())
}

/// update role
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    match system_role_service::get_by_sign(&state.db, &params.sign, Some(id)).await? {
        Some(_) => {
            return Err(ErrorCode::RoleSignExsist);
        }
        None => {
            let info = system_role_service::info(&state.db, id).await?;
            if info.sign().eq(&state.db.config().get_admin_role_sign()) {
                return Err(ErrorCode::NotChangeAdmin);
            }
        }
    }
    let user_menus = system_menu_service::get_user_menus_by_menu_ids(
        &state.db,
        claims.user_id,
        params.menu_ids.clone().unwrap_or_default(),
    )
    .await?;
    system_role_service::update(&state.db, id, params.into(), user_menus).await?;
    Ok(Body::empty())
}

/// delete role
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    let info = system_role_service::info(&state.db, id).await?;
    if info.sign().eq(&state.db.config().get_admin_role_sign()) {
        return Err(ErrorCode::NotDeleteData);
    }
    system_role_service::delete(&state.db, id).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<SearchRequest> for system_role_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(value.keyword, value.status, value.paginate)
    }
}
#[derive(Debug, Deserialize)]
struct CreateRequest {
    name: String,
    sign: String,
    #[serde(default)]
    sort: i32,
    #[serde(default)]
    describe: String,
    #[serde(default)]
    status: i32,
    menu_ids: Option<Vec<i32>>,
}

impl From<CreateRequest> for system_role_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            sort: Some(value.sort),
            describe: Some(value.describe),
            status: Some(value.status),
        }
    }
}

impl From<CreateRequest> for system_role_service::UpdateParams {
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
