use super::Claims;
use crate::{error::Result, state::AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use service::system_menu;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/menu", get(index))
        .route("/menu/:id", get(info))
        .route("/menu", post(create))
        .route("/menu/update", post(update))
        .route("/menu/:id", get(del))
        .with_state(state)
}

/// get tree menu
async fn index(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<RequestSearch>,
) -> Result<impl IntoResponse> {
    Ok(Json(
        system_menu::get_user_menu_trees(&state.db, claims.user_id, query).await?,
    ))
}

/// menu detail
async fn info(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    Ok(Json(system_menu::info(&state.db, id).await?))
}

/// create menu
async fn create(
    State(state): State<AppState>,
    Json(params): Json<RequestFormCreate>,
) -> Result<impl IntoResponse> {
    system_menu::create(&state.db, params.into()).await?;
    Ok(Body::empty())
}

/// update menu
async fn update(
    State(state): State<AppState>,
    Json(params): Json<RequestFormUpdate>,
) -> Result<impl IntoResponse> {
    system_menu::update(&state.db, params.id, params.form.into()).await?;
    Ok(Body::empty())
}

/// delete menu
async fn del(Path(id): Path<i32>, State(state): State<AppState>) -> Result<impl IntoResponse> {
    system_menu::delete(&state.db, id).await?;
    Ok(Body::empty())
}

// #[serde_as]
#[derive(Debug, Deserialize)]
struct RequestSearch {
    keyword: Option<String>,
    menu_types: Option<Vec<i32>>,
}

impl From<RequestSearch> for system_menu::Filter {
    fn from(value: RequestSearch) -> Self {
        Self::new(
            value.keyword,
            value
                .menu_types
                .map(|x| x.into_iter().map(|y| y.into()).collect()),
        )
    }
}

#[derive(Debug, Deserialize)]
struct RequestFormCreate {
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

impl From<RequestFormCreate> for system_menu::FormParamsForCreate {
    fn from(value: RequestFormCreate) -> Self {
        Self {
            parent_id: value.parent_id,
            r#type: value.r#type,
            title: value.title,
            icon: value.icon,
            router_name: value.router_name,
            router_component: value.router_component,
            router_path: value.router_path,
            redirect: value.redirect,
            link: value.link,
            iframe: value.iframe,
            btn_auth: value.btn_auth,
            api_url: value.api_url,
            api_method: value.api_method,
            is_hide: value.is_hide,
            is_keep_alive: value.is_keep_alive,
            is_affix: value.is_affix,
            sort: value.sort,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RequestFormUpdate {
    id: i32,
    #[serde(flatten)]
    form: RequestFormCreate,
}
