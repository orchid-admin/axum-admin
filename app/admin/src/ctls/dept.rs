use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routers<S>(state: crate::state::AppState) -> axum::Router<S> {
    Router::new()
        // .route("/role", get(index))
        // .route("/role/:id", get(info))
        // .route("/role", post(create))
        // .route("/role/:id", put(update))
        // .route("/role/:id", delete(del))
        .with_state(state)
}
