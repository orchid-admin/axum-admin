use super::Claims;
use crate::{
    captcha::UseType as CaptchaUseType,
    error::{ErrorCode, Result},
    extracts::ValidatorJson,
    jwt::UseType as JwtUseType,
    openapi::DocmentPathSchema,
    password::Password,
    state::AppState,
};
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use service::sys_user;
use ts_rs::TS;
use utoipa::{Path, ToSchema};
use validator::Validate;

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
#[utoipa::path(
    post,
    path = "/login_by_account",
    tag = "授权",
    request_body = LoginByAccountRequest,
    responses(
        (status = 200, body = LoginReponse),
        (status = 500, body = ErrorCode, example = json!(ErrorCode::Other("用户名或密码错误").to_json_string()))
    )
)]
async fn login_by_account(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<LoginByAccountRequest>,
) -> Result<impl IntoResponse> {
    let mut captcha = state.captcha.lock().await;
    match captcha.get_item(&CaptchaUseType::AdminLogin, &params.key) {
        Some(captcha_item) => {
            if !captcha_item.verify(&params.code) {
                return Err(ErrorCode::Other("验证码错误"));
            }
            captcha.remove_item(&captcha_item);
            match sys_user::find_user_by_username(&state.db, &params.username).await? {
                Some(user) => {
                    let verify_result = Password::verify_password(
                        user.password,
                        user.salt,
                        params.password.as_bytes(),
                    )?;

                    if !verify_result {
                        return Err(ErrorCode::Other("用户名或密码错误"));
                    }

                    let token = state
                        .jwt
                        .lock()
                        .await
                        .generate(JwtUseType::Admin, Claims::build(user.id))?;
                    Ok(Json(LoginReponse {
                        token,
                        username: Some(user.username),
                    }))
                }
                None => Err(ErrorCode::Other("用户名或密码错误")),
            }
        }
        None => Err(ErrorCode::Other("验证码错误")),
    }
}

/// 手机号登录
#[utoipa::path(
    post,
    path = "/login_by_mobile",
    tag = "授权",
    request_body = LoginByMobileRequest,
    responses(
        (status = 200, body = LoginReponse),
        (status = 500, body = ErrorCode, example = json!(ErrorCode::Other("用户名或密码错误").to_json_string()))
    )
)]
async fn login_by_mobile(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<LoginByMobileRequest>,
) -> Result<impl IntoResponse> {
    match sys_user::find_user_by_phone(&state.db, &params.mobile).await? {
        Some(user) => {
            let token = state
                .jwt
                .lock()
                .await
                .generate(JwtUseType::Admin, Claims::build(user.id))?;
            Ok(Json(LoginReponse {
                token,
                username: Some(user.username),
            }))
        }
        None => Err(ErrorCode::Other("用户名或密码错误")),
    }
}

/// 扫码登录
///
///
#[utoipa::path(
    post,
    path = "/login_by_qrcode",
    tag = "授权",
    request_body = LoginByAccountRequest,
    responses(
        (status = 200, body = [LoginReponse])
    )
)]
async fn login_by_qrcode(
    State(state): State<AppState>,
    ValidatorJson(_params): ValidatorJson<LoginByAccountRequest>,
) -> Result<impl IntoResponse> {
    let token = state
        .jwt
        .lock()
        .await
        .generate(JwtUseType::Admin, Claims::build(1))?;
    Ok(Json(LoginReponse {
        token,
        username: None,
    }))
}

/// 获取登录验证码
#[utoipa::path(
    get,
    path = "/get_captcha",
    tag = "授权",
    responses(
        (status = 200, body = GetCaptchaReponse),
        (status = 500, body = ErrorCode, example = json!(ErrorCode::InternalServer("生成验证码失败").to_json_string()))
    )
)]
async fn get_captcha(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let (key, image) =
        state
            .captcha
            .lock()
            .await
            .generate(CaptchaUseType::AdminLogin, 5, 130, 40, false, 1)?;

    Ok(Json(GetCaptchaReponse { key, image }))
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, TS)]
#[ts(export)]
struct LoginByAccountRequest {
    /// 用户名
    #[validate(length(min = 5, message = "用户名长度错误"))]
    username: String,
    /// 密码
    #[validate(length(min = 6, message = "密码长度错误"))]
    password: String,
    /// 图片验证码KEY值
    #[validate(length(min = 5, message = "请刷新图片验证码后重试"))]
    key: String,
    /// 图片验证码值
    #[validate(length(equal = 5, message = "输入的验证码长度错误"))]
    code: String,
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, TS)]
#[ts(export)]
struct LoginByMobileRequest {
    /// 手机号码
    mobile: String,
    /// 短信验证码
    code: String,
}

#[derive(Debug, Serialize, ToSchema, TS)]
#[ts(export)]
struct LoginReponse {
    /// 登录账户的TOKEN
    token: String,
    #[serde(rename = "userName")]
    username: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, TS)]
#[ts(export)]
struct GetCaptchaReponse {
    /// 图片key（用于提交时识别）
    key: String,
    /// 图片base64编码
    image: String,
}
