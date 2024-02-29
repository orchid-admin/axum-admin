use getset::Getters;
use serde::Serialize;

#[derive(Debug, Serialize, Getters)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub")]
    unique_code: String,
    #[getset(get = "pub")]
    email: String,
    #[getset(get = "pub")]
    mobile: String,
    #[getset(get = "pub")]
    nickname: String,
    #[getset(get = "pub")]
    avatar: String,
    #[serde(skip)]
    #[getset(get = "pub")]
    password: String,
    #[serde(skip)]
    #[getset(get = "pub")]
    salt: String,
    #[getset(get = "pub")]
    sex: i32,
    #[getset(get = "pub")]
    balance: f32,
    #[getset(get = "pub")]
    integral: i32,
    remark: String,
    #[getset(get = "pub")]
    status: i32,
    #[getset(get = "pub")]
    is_promoter: i32,
    #[getset(get = "pub")]
    last_login_ip: String,
    #[getset(get = "pub")]
    last_login_time: Option<String>,
    #[getset(get = "pub")]
    created_at: String,
}
