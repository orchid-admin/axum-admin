mod auth;
mod menu;
mod role;
mod user;

pub use auth::Auth;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    user_id: i64,
}
