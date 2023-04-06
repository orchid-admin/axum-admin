use super::Claims;
use crate::{error::Result, openapi::DocmentPathSchema, state::AppState};
use axum::{
    extract::{self, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{Path, ToSchema};

pub fn routers<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/login_by_account", post(login_by_account))
        .route("/login_by_mobile", post(login_by_mobile))
        .route("/login_by_code", post(login_by_qrcode))
        .route("/get_captcha", get(get_captcha))
        .with_state(state)
}

pub fn api_docment() -> DocmentPathSchema {
    let paths = crate::api_doc_path! {
        __path_login_by_account,
        __path_login_by_mobile,
        __path_login_by_qrcode,
        __path_get_captcha
    };
    let schemas = crate::api_doc_schema! {
        LoginByAccountRequest,
        LoginByMobileRequest,
        LoginReponse,
        GetCaptchaReponse
    };
    (paths, schemas)
}
/// 账户登录
///
///
#[utoipa::path(
    post,
    path = "/login_by_account",
    tag = "auth",
    request_body = LoginByAccountRequest,
    responses(
        (status = 200, body = [LoginReponse])
    )
)]
async fn login_by_account(
    State(state): State<AppState>,
    extract::Json(_params): extract::Json<LoginByAccountRequest>,
) -> Result<Json<impl Serialize>> {
    let token = state.jwt.lock().await.generate("admin", Claims::build(1))?;
    Ok(Json(LoginReponse { token }))
}

/// 手机号登录
///
///
#[utoipa::path(
    post,
    path = "/login_by_mobile",
    tag = "auth",
    request_body = LoginByMobileRequest,
    responses(
        (status = 200, body = [LoginReponse])
    )
)]
async fn login_by_mobile(
    State(state): State<AppState>,
    extract::Json(_params): extract::Json<LoginByMobileRequest>,
) -> Result<Json<impl Serialize>> {
    let token = state.jwt.lock().await.generate("admin", Claims::build(1))?;
    Ok(Json(LoginReponse { token }))
}

/// 扫码登录
///
///
#[utoipa::path(
    post,
    path = "/login_by_qrcode",
    tag = "auth",
    request_body = LoginByAccountRequest,
    responses(
        (status = 200, body = [LoginReponse])
    )
)]
async fn login_by_qrcode(
    State(state): State<AppState>,
    extract::Json(_params): extract::Json<LoginByAccountRequest>,
) -> Result<Json<impl Serialize>> {
    let token = state.jwt.lock().await.generate("admin", Claims::build(1))?;
    Ok(Json(LoginReponse { token }))
}

/// 获取登录验证码
///
///
#[utoipa::path(
    get,
    path = "/get_captcha",
    tag = "auth",
    responses(
        (status = 200, body = [GetCaptchaReponse])
    )
)]
async fn get_captcha(State(state): State<AppState>) -> Result<Json<impl Serialize>> {
    let (key, image) = state
        .captcha
        .lock()
        .await
        .generate("login", 5, 130, 40, false, 1)?;

    Ok(Json(GetCaptchaReponse { key, image }))
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct LoginByAccountRequest {
    /// 用户名
    username: String,
    /// 密码
    password: String,
    /// 图片验证码KEY值
    key: String,
    /// 图片验证码值
    code: String,
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct LoginByMobileRequest {
    /// 手机号码
    mobile: String,
    /// 短信验证码
    code: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct LoginReponse {
    /// 登录账户的TOKEN
    token: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct GetCaptchaReponse {
    /// 图片key（用于提交时识别）
    key: String,
    /// 图片base64编码
    image: String,
}
