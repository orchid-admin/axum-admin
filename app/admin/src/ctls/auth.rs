use super::{Claims, CtlRouter};
use crate::{error::Result, state::AppState};
use axum::{
    extract::{self, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub struct Auth;

impl CtlRouter for Auth {
    fn routers<S>(state: AppState) -> Router<S> {
        Router::new()
            .route("/login_by_account", post(Auth::login_by_account))
            .route("/login_by_mobile", post(Auth::login_by_mobile))
            .route("/login_by_code", post(Auth::login_by_qrcode))
            .route("/get_captcha", get(Auth::get_captcha))
            .route("/get_captcha/:id", get(Auth::get_captcha))
            .with_state(state)
    }
}
#[allow(dead_code)]
impl Auth {
    async fn login_by_account(
        State(state): State<AppState>,
        extract::Json(_params): extract::Json<LoginByAccountRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = state.jwt.lock().await.generate("admin", Claims::build(1))?;
        Ok(Json(LoginReponse { token }))
    }

    async fn login_by_mobile(
        State(state): State<AppState>,
        extract::Json(_params): extract::Json<LoginByMobileRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = state.jwt.lock().await.generate("admin", Claims::build(1))?;
        Ok(Json(LoginReponse { token }))
    }

    async fn login_by_qrcode(
        State(state): State<AppState>,
        extract::Json(_params): extract::Json<LoginByAccountRequest>,
    ) -> Result<Json<impl Serialize>> {
        let token = state.jwt.lock().await.generate("admin", Claims::build(1))?;
        Ok(Json(LoginReponse { token }))
    }

    async fn get_captcha(State(state): State<AppState>) -> Result<Json<impl Serialize>> {
        let (key, image) = state
            .captcha
            .lock()
            .await
            .generate("login", 5, 130, 40, false, 1)?;

        Ok(Json(GetCaptchaReponse { key, image }))
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct LoginByAccountRequest {
    username: String,
    password: String,
    key: String,
    code: String,
}
#[allow(dead_code)]
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
