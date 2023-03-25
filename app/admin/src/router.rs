use axum::{middleware, Router};

use crate::ctls::Auth;
pub fn init() -> Router {
    Router::new().nest(
        "/adminapi",
        Router::new().merge(no_auth_routers()).merge(auth_routers()),
    )
}

fn no_auth_routers() -> Router {
    Router::new().merge(Auth::routers())
}

fn auth_routers() -> Router {
    use crate::middleware::auth;
    Router::new().layer(middleware::from_fn(auth))
}
