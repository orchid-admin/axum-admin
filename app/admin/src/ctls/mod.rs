use ts_rs::TS;

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

// #[allow(dead_code)]
// #[derive(Debug, serde::Serialize, TS)]
// #[ts(export)]
// pub struct ActionSuccess {
//     msg: String,
// }

// impl ActionSuccess {
//     pub fn new(message: &str) -> Self {
//         Self {
//             msg: message.to_owned(),
//         }
//     }

//     pub fn ok() -> Self {
//         Self {
//             msg: "ok".to_owned(),
//         }
//     }
// }
