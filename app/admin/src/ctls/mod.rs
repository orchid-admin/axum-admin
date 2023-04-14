pub mod auth;
pub mod menu;
pub mod role;
pub mod user;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    user_id: i32,
    exp: i128,
}

impl Claims {
    pub fn build(user_id: i32) -> Self {
        Self {
            user_id,
            exp: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
        }
    }
}
