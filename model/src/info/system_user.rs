use super::system_dept::InfoForUserInfo as Dept;
use super::system_role::InfoForUserInfo as Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub phone: String,
    pub email: String,
    pub sex: i32,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub salt: String,
    pub describe: String,
    pub expire_time: Option<String>,
    pub status: i32,
    pub last_login_ip: String,
    pub last_login_time: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InfoWithDeptAndRole {
    #[serde(flatten)]
    pub user: Info,
    pub dept: Option<Dept>,
    pub role: Option<Role>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    pub id: Option<i32>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
}

#[derive(Debug, Default, Deserialize)]
pub struct FormParamsForCreate {
    pub username: String,
    pub nickname: String,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub phone: String,
    pub email: String,
    pub sex: i32,
    pub password: String,
    #[serde(skip)]
    pub salt: String,
    pub describe: String,
    pub expire_time: Option<String>,
    pub status: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
