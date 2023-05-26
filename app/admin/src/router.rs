use crate::{
    ctls::{auth, sys_dept, sys_menu, sys_role, sys_user},
    state::AppState,
};
use axum::{middleware, Router};

pub async fn init(state: AppState) -> Router {
    Router::new()
        .merge(auth::routers(state.clone()))
        .merge(auth_routers(state))
}

fn auth_routers(state: AppState) -> Router {
    use crate::ctls::{access_matched_path, token_check};
    Router::new()
        .merge(sys_user::routers(state.clone()))
        .merge(sys_role::routers(state.clone()))
        .merge(sys_menu::routers(state.clone()))
        .merge(sys_dept::routers(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            access_matched_path,
        ))
        .layer(middleware::from_fn_with_state(state.clone(), token_check))
        .with_state(state)
}
