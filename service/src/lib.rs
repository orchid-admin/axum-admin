use prisma_client_rust::chrono::{
    DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use serde_with::{serde_as, DisplayFromStr};

#[allow(unused, warnings)]
mod prisma;
pub mod sys_dept;
pub mod sys_menu;
pub mod sys_role;
#[allow(unused)]
mod sys_role_menu;
pub mod sys_user;

pub const ADMIN_USERNAME: &str = "admin";
pub const ADMIN_ROLE_SIGN: &str = "admin";

pub type Database = std::sync::Arc<prisma::PrismaClient>;
pub type Result<T> = std::result::Result<T, ServiceError>;

pub async fn new_client() -> Result<Database> {
    let database = std::sync::Arc::new(prisma::PrismaClient::_builder().build().await?);
    let role = sys_role::upsert(&database, "超级管理员", ADMIN_ROLE_SIGN).await?;
    sys_user::upsert_system_user(
        &database,
        ADMIN_USERNAME,
        "sfWTwt9NxLNapTmoIdzfUbbRODMk266kc7ArZcF2EsQ",
        "nodiZ0cU0ER5Vg3n+rOsoQ",
        role.id,
    )
    .await?;
    Ok(database)
}

#[allow(dead_code)]
pub fn now_time() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())
}
#[allow(dead_code)]
fn to_local_string(datetime: DateTime<FixedOffset>) -> String {
    datetime
        .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

#[allow(dead_code)]
pub fn parse_string(datetime: String) -> DateTime<FixedOffset> {
    let time = NaiveTime::from_hms_opt(00, 00, 00).unwrap();

    let now_datetime = now_time();
    match NaiveDate::parse_from_str(datetime.as_str(), "%Y-%m-%d") {
        Ok(date) => {
            let local_datetime = NaiveDateTime::new(date, time);
            let tz_offset = FixedOffset::east_opt(8 * 3600).unwrap();
            TimeZone::from_local_datetime(&tz_offset, &local_datetime).unwrap()
        }
        Err(_) => now_datetime,
    }
}

#[derive(Debug)]
pub enum ServiceError {
    BuildClient(String),
    QueryError(String),
    RelationNotFetchedError(String),
    DataNotFound,
}

#[serde_as]
#[derive(Debug, serde::Deserialize)]
pub struct PaginateRequest {
    #[serde_as(as = "DisplayFromStr")]
    page: i64,
    #[serde_as(as = "DisplayFromStr")]
    limit: i64,
}

impl PaginateRequest {
    fn get_skip(&self) -> i64 {
        match self.page > 0 {
            true => (self.page - 1) * self.limit,
            false => self.limit,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PaginateResponse<T: serde::Serialize> {
    total: i64,
    data: T,
}

#[derive(Debug, serde::Serialize)]
pub struct DataPower<T: serde::Serialize> {
    _can_edit: bool,
    _can_delete: bool,
    #[serde(flatten)]
    data: T,
}

pub trait Tree<T> {
    fn set_child(&mut self, data: Vec<T>);
}

trait TreeInfo {
    fn get_parent_id(&self) -> i32;
    fn get_id(&self) -> i32;
}

fn get_tree_start_parent_id<S>(infos: &[S]) -> i32
where
    S: TreeInfo,
{
    let mut parent_ids = infos
        .iter()
        .map(|x| x.get_parent_id())
        .collect::<Vec<i32>>();
    parent_ids.sort();
    let parent_id = parent_ids.first().copied().unwrap_or_default();
    parent_id
}
fn vec_to_tree_into<T, S>(parent_id: &i32, menus: &Vec<S>) -> Vec<T>
where
    T: Tree<T> + std::convert::From<S>,
    S: TreeInfo + Clone,
{
    menus
        .iter()
        .filter(|x| x.get_parent_id().eq(parent_id))
        .map(|node| {
            let node_id = node.get_id();
            let mut data: T = node.clone().into();
            data.set_child(vec_to_tree_into::<T, S>(&node_id, menus));
            data
        })
        .collect::<Vec<T>>()
}

impl From<prisma_client_rust::NewClientError> for ServiceError {
    fn from(value: prisma_client_rust::NewClientError) -> Self {
        Self::BuildClient(value.to_string())
    }
}

impl From<prisma_client_rust::QueryError> for ServiceError {
    fn from(value: prisma_client_rust::QueryError) -> Self {
        Self::QueryError(value.to_string())
    }
}

impl From<prisma_client_rust::RelationNotFetchedError> for ServiceError {
    fn from(value: prisma_client_rust::RelationNotFetchedError) -> Self {
        Self::RelationNotFetchedError(value.to_string())
    }
}
