use crate::{
    ctls::{auth, menu, role, user},
    state::AppState,
};
use axum::{
    middleware::{self, map_request},
    Router,
};

pub async fn init(state: AppState) -> Router {
    Router::new()
        .merge(auth::routers(state.clone()))
        .merge(auth_routers(state))
}

fn auth_routers(state: AppState) -> Router {
    use crate::middleware::{access_matched_path, token_check};
    Router::new()
        .merge(user::routers(state.clone()))
        .merge(role::routers(state.clone()))
        .merge(menu::routers(state.clone()))
        .layer(map_request(access_matched_path))
        .layer(middleware::from_fn_with_state(state.clone(), token_check))
        .with_state(state)
}
