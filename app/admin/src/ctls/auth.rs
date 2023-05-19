use super::Claims;
use crate::{
    captcha::UseType as CaptchaUseType,
    error::{ErrorCode, Result},
    extracts::ValidatorJson,
    jwt::UseType as JwtUseType,
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
use validator::Validate;

pub fn routers<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/login_by_account", post(login_by_account))
        .route("/login_by_mobile", post(login_by_mobile))
        .route("/login_by_code", post(login_by_qrcode))
        .route("/get_captcha", get(get_captcha))
        .with_state(state)
}

/// 账户登录
async fn login_by_account(
    State(state): State<AppState>,
    ValidatorJson(params): ValidatorJson<LoginByAccountRequest>,
) -> Result<impl IntoResponse> {
    let mut captcha = state.captcha.lock().await;
    match captcha.get_item(&CaptchaUseType::AdminLogin, &params.key) {
        Some(captcha_item) => {
            if !captcha_item.verify_lowercase(&params.code) {
                return Err(ErrorCode::Other("验证码错误"));
            }
            captcha.remove_item(&captcha_item);
            match sys_user::find_user_by_username(&state.db, &params.username).await? {
                Some(user) => {
                    let verify_result = Password::verify_password(
                        user.get_password(),
                        user.get_salt(),
                        params.password.as_bytes(),
                    )?;

                    if !verify_result {
                        return Err(ErrorCode::Other("用户名或密码错误"));
                    }

                    let token = state
                        .jwt
                        .lock()
                        .await
                        .generate(JwtUseType::Admin, Claims::build(user.get_id()))?;
                    Ok(Json(LoginReponse {
                        token,
                        username: Some(user.get_username()),
                    }))
                }
                None => Err(ErrorCode::Other("用户名或密码错误")),
            }
        }
        None => Err(ErrorCode::Other("验证码错误")),
    }
}

/// 手机号登录
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
                .generate(JwtUseType::Admin, Claims::build(user.get_id()))?;
            Ok(Json(LoginReponse {
                token,
                username: Some(user.get_username()),
            }))
        }
        None => Err(ErrorCode::Other("用户名或密码错误")),
    }
}

/// 扫码登录
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
async fn get_captcha(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let content =
        state
            .captcha
            .lock()
            .await
            .generate(CaptchaUseType::AdminLogin, 5, 130, 40, false, 1)?;

    Ok(Json(GetCaptchaReponse {
        key: content.key,
        image: content.image,
        value: content.text,
    }))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
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

#[derive(Debug, Serialize, Deserialize, Validate)]
struct LoginByMobileRequest {
    /// 手机号码
    mobile: String,
    /// 短信验证码
    code: String,
}

#[derive(Debug, Serialize)]
struct LoginReponse {
    /// 登录账户的TOKEN
    token: String,
    #[serde(rename = "userName")]
    username: Option<String>,
}

#[derive(Debug, Serialize)]
struct GetCaptchaReponse {
    /// 图片key（用于提交时识别）
    key: String,
    /// 图片base64编码
    image: String,
    /// 文本值
    value: String,
}
