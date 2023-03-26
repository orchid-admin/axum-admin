use axum::{
    middleware,
    routing::{get, post, IntoMakeService},
    Router,
};

use crate::{
    ctls::Auth,
    state::{AppState, State},
};
pub fn init() -> IntoMakeService<Router<()>> {
    let state = State::build();
    Router::new()
        .merge(no_auth_routers(state.clone()))
        .merge(auth_routers(state.clone()))
        .with_state(state)
        .into_make_service()
}

fn no_auth_routers(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login_by_account", post(Auth::login_by_account))
        .route("/login_by_mobile", post(Auth::login_by_mobile))
        .route("/login_by_code", post(Auth::login_by_qrcode))
        .route("/get_captcha", get(Auth::get_captcha))
        .with_state(state)
}

fn auth_routers(state: AppState) -> Router<AppState> {
    use crate::middleware::auth;
    Router::new()
        .layer(middleware::from_fn(auth))
        .with_state(state)
}
