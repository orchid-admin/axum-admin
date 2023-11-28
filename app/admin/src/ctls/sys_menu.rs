use super::Claims;
use crate::{error::Result, state::AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_menu_service;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/menu", get(index))
        .route("/menu/:id", get(info))
        .route("/menu", post(create))
        .route("/menu/:id", put(update))
        .route("/menu/:id", delete(del))
        .with_state(state)
}

/// get tree menu
async fn index(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<SearchRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        system_menu_service::get_user_menu_trees(&state.db, claims.user_id, &query.into()).await?,
    ))
}

/// menu detail
async fn info(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    Ok(Json(system_menu_service::info(&state.db, id).await?))
}

/// create menu
async fn create(
    State(state): State<AppState>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    system_menu_service::create(&state.db, &params.title.clone(), params.into()).await?;
    Ok(Body::empty())
}

/// update menu
async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(params): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    system_menu_service::update(&state.db, id, params.into()).await?;
    Ok(Body::empty())
}

/// delete menu
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    system_menu_service::delete(&state.db, id).await?;
    Ok(Body::empty())
}

// #[serde_as]
#[derive(Debug, Deserialize)]
struct SearchRequest {
    keyword: Option<String>,
    menu_types: Option<Vec<i32>>,
}

impl From<SearchRequest> for system_menu_service::SearchParams {
    fn from(value: SearchRequest) -> Self {
        Self::new(
            value.keyword,
            value
                .menu_types
                .map(|x| x.into_iter().map(|y| y.into()).collect()),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateRequest {
    parent_id: i32,
    r#type: i32,
    title: String,
    icon: String,
    router_name: String,
    router_component: String,
    router_path: String,
    redirect: String,
    link: String,
    iframe: String,
    btn_auth: String,
    api_url: String,
    api_method: String,
    is_hide: Option<i32>,
    is_keep_alive: Option<i32>,
    is_affix: Option<i32>,
    sort: i32,
}

impl From<CreateRequest> for system_menu_service::CreateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            parent_id: Some(value.parent_id),
            r#type: Some(value.r#type),
            icon: Some(value.icon),
            router_name: Some(value.router_name),
            router_component: Some(value.router_component),
            router_path: Some(value.router_path),
            redirect: Some(value.redirect),
            link: Some(value.link),
            iframe: Some(value.iframe),
            btn_auth: Some(value.btn_auth),
            api_url: Some(value.api_url),
            api_method: Some(value.api_method),
            is_hide: value.is_hide,
            is_keep_alive: value.is_keep_alive,
            is_affix: value.is_affix,
            sort: Some(value.sort),
        }
    }
}

impl From<CreateRequest> for system_menu_service::UpdateParams {
    fn from(value: CreateRequest) -> Self {
        Self {
            parent_id: Some(value.parent_id),
            r#type: Some(value.r#type),
            title: Some(value.title),
            icon: Some(value.icon),
            router_name: Some(value.router_name),
            router_component: Some(value.router_component),
            router_path: Some(value.router_path),
            redirect: Some(value.redirect),
            link: Some(value.link),
            iframe: Some(value.iframe),
            btn_auth: Some(value.btn_auth),
            api_url: Some(value.api_url),
            api_method: Some(value.api_method),
            is_hide: value.is_hide,
            is_keep_alive: value.is_keep_alive,
            is_affix: value.is_affix,
            sort: Some(value.sort),
        }
    }
}
