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
use service::system_role;
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
    let data = system_role::all(&state.db).await?;
    Ok(Json(data))
}

/// role list
async fn index(
    State(state): State<AppState>,
    Query(params): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    let data = system_role::paginate(&state.db, params.into()).await?;
    Ok(Json(data))
}

/// role detail
async fn info(
    State(state): State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<impl IntoResponse> {
    Ok(Json(system_role::info(&state.db, id).await?))
}

/// create role
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    if system_role::get_by_sign(&state.db, &params.sign, None)
        .await?
        .is_some()
    {
        return Err(ErrorCode::RoleSignExsist);
    }

    system_role::create(
        &state.db,
        params.clone().into(),
        claims.user_id,
        params.menu_ids,
    )
    .await?;
    Ok(Body::empty())
}

/// update role
async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(param): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    let sign_info = system_role::get_by_sign(&state.db, &param.sign, Some(id)).await?;
    if sign_info.is_some() {
        return Err(ErrorCode::RoleSignExsist);
    }

    system_role::update(
        &state.db,
        id,
        param.clone().into(),
        claims.user_id,
        param.menu_ids,
    )
    .await?;
    Ok(Body::empty())
}

/// delete role
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    let _info = system_role::info(&state.db, id).await?;
    system_role::delete(&state.db, id).await?;
    Ok(Body::empty())
}

#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    status: Option<i32>,
    #[serde(flatten)]
    paginate: PaginateParams,
}
impl From<RequestSearch> for system_role::Filter {
    fn from(value: RequestSearch) -> Self {
        Self {
            keyword: value.keyword,
            status: value.status,
            paginate: value.paginate,
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
struct RequestFormCreate {
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

impl From<RequestFormCreate> for system_role::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        Self {
            name: value.name,
            sign: value.sign,
            describe: value.describe,
            status: value.status,
            sort: value.sort,
        }
    }
}
