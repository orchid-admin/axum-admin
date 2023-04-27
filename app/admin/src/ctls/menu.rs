use crate::{error::Result, extracts::ValidatorJson, state::AppState};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use service::sys_menu::{self, MenuCreateParams};
use validator::Validate;

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        .route("/menu/index", get(tree))
        .route("/menu/create", post(create))
        .with_state(state)
}

/// 新增
async fn create(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<MenuCreateRequest>,
) -> Result<impl IntoResponse> {
    sys_menu::create(&state.db, &params.meta.title.clone(), params.into()).await?;
    Ok("")
}

/// 获取树形列表
async fn tree(State(state): State<AppState>) -> Result<impl IntoResponse> {
    Ok(Json(sys_menu::get_menus_tree(&state.db).await?))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate)]
struct MenuCreateRequest {
    parent_id: Option<i32>,
    #[validate(length(min = 2, message = "类型长度错误"))]
    #[serde(rename = "menuType")]
    #[validate(required)]
    r#type: Option<String>,
    #[serde(rename = "name")]
    router_name: Option<String>,
    #[serde(rename = "componentAlias")]
    component: Option<String>,
    #[serde(rename = "isLink")]
    is_link: Option<bool>,
    path: Option<String>,
    redirect: Option<String>,
    #[serde(rename = "btnPower")]
    btn_power: Option<String>,
    #[serde(rename = "menuSort")]
    #[validate(required)]
    sort: Option<i32>,
    meta: MenuMeta,
}

#[derive(Debug, Deserialize, Validate)]
struct MenuMeta {
    title: String,
    icon: Option<String>,
    #[serde(rename = "isHide")]
    is_hide: Option<bool>,
    #[serde(rename = "isKeepAlive")]
    is_keep_alive: Option<bool>,
    #[serde(rename = "isAffix")]
    is_affix: Option<bool>,
    #[serde(rename = "isLink")]
    link: Option<String>,
    #[serde(rename = "isIframe")]
    is_iframe: Option<bool>,
}

impl From<MenuCreateRequest> for MenuCreateParams {
    fn from(params: MenuCreateRequest) -> MenuCreateParams {
        MenuCreateParams {
            parent_id: params.parent_id,
            r#type: params.r#type,
            router_name: params.router_name,
            component: params.component,
            is_link: params.is_link,
            path: params.path,
            redirect: params.redirect,
            btn_power: params.btn_power,
            sort: params.sort,
            meta_icon: params.meta.icon,
            meta_is_hide: params.meta.is_hide,
            meta_is_keep_alive: params.meta.is_keep_alive,
            meta_is_affix: params.meta.is_affix,
            meta_link: params.meta.link,
            meta_is_iframe: params.meta.is_iframe,
        }
    }
}
