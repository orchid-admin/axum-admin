use axum::{extract, Json};

use serde::{Deserialize, Serialize};

use crate::{error::Result, jwt};

pub struct Auth;

#[allow(dead_code)]
impl Auth {
    pub async fn login_by_account(
        extract::Json(_params): extract::Json<LoginByAccountRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = jwt::Claims::to_token(1)?;
        Ok(Json(LoginReponse { token }))
    }

    pub async fn login_by_mobile(
        extract::Json(_params): extract::Json<LoginByMobileRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = jwt::Claims::to_token(1)?;
        Ok(Json(LoginReponse { token }))
    }

    pub async fn login_by_qrcode(
        extract::Json(_params): extract::Json<LoginByAccountRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = jwt::Claims::to_token(1)?;
        Ok(Json(LoginReponse { token }))
    }

    pub async fn get_captcha() -> Result<Json<impl Serialize>> {
        Ok(Json(GetCaptchaReponse {
            key: String::new(),
            image: String::new(),
        }))
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LoginByAccountRequest {
    username: String,
    password: String,
    key: String,
    code: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LoginByMobileRequest {
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
