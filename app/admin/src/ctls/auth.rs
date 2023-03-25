use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{error::Result, jwt};

pub struct Auth;

impl Auth {
    pub fn routers() -> Router {
        Router::new()
            .route("/login_by_account", post(Self::login_by_account))
            .route("/login_by_mobile", post(Self::login_by_mobile))
            .route("/login_by_code", post(Self::login_by_qrcode))
            .route("/get_captcha", get(Self::get_captcha))
    }
    async fn login_by_account(
        extract::Json(_params): extract::Json<LoginByAccountRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = jwt::Claims::to_token(1)?;
        Ok(Json(LoginReponse { token }))
    }

    async fn login_by_mobile(
        extract::Json(_params): extract::Json<LoginByMobileRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = jwt::Claims::to_token(1)?;
        Ok(Json(LoginReponse { token }))
    }

    async fn login_by_qrcode(
        extract::Json(_params): extract::Json<LoginByAccountRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = jwt::Claims::to_token(1)?;
        Ok(Json(LoginReponse { token }))
    }

    async fn get_captcha() -> Result<Json<impl Serialize>> {
        Ok(Json(GetCaptchaReponse {
            key: String::new(),
            image: String::new(),
        }))
    }
}

#[derive(Debug, Deserialize)]
struct LoginByAccountRequest {
    username: String,
    password: String,
    key: String,
    code: String,
}

#[derive(Debug, Deserialize)]
struct LoginByMobileRequest {
    mobile: String,
    code: String,
}

#[derive(Debug, Serialize)]
struct LoginReponse {
    token: String,
}

#[derive(Debug, Serialize)]
struct GetCaptchaReponse {
    key: String,
    image: String,
}
