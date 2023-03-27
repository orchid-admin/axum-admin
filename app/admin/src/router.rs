use axum::{
    body::Body,
    extract::{rejection::MatchedPathRejection, MatchedPath},
    http::Request,
    middleware::{self, map_request},
    RequestExt, Router,
};

use crate::{
    ctls::{Auth, CtlRouter, User},
    state::{AppState, State},
};
pub fn init() -> Router {
    let state = State::build();
    Router::new()
        .merge(Auth::routers(state.clone()))
        .merge(auth_routers(state))
}

fn auth_routers(state: AppState) -> Router {
    use crate::middleware::{access_matched_path, auth};
    Router::new()
        .merge(User::routers(state.clone()))
        .layer(map_request(access_matched_path))
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
}
