use axum::{middleware, Router};

use crate::{
    ctls::Auth,
    state::{AppState, State},
};
pub fn init() -> Router {
    let state = State::build();
    Router::new()
        .merge(Auth::routers(state.clone()))
        .merge(auth_routers(state))
}

fn auth_routers(state: AppState) -> Router {
    use crate::middleware::auth;
    Router::new()
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
}
