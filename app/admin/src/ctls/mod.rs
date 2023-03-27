mod auth;
mod menu;
mod role;
mod user;

pub use {auth::Auth, user::User};

use crate::state::AppState;
use axum::Router;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    user_id: i64,
    exp: i128,
}

impl Claims {
    pub fn build(user_id: i64) -> Self {
        Self {
            user_id,
            exp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
        }
    }
}

pub trait CtlRouter {
    fn routers<S>(state: AppState) -> Router<S>;
}
