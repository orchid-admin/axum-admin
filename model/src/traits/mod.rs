use crate::connect::DbConnectPool as ConnectPool;
use crate::info::system_role;
use crate::info::system_user;
use crate::Result;
pub trait SystemUserTrait {
    async fn find_user_by_username(
        pool: &ConnectPool,
        username: &str,
    ) -> Result<Option<system_user::Info>>;

    async fn find_user_by_phone(
        pool: &ConnectPool,
        username: &str,
    ) -> Result<Option<system_user::Info>>;

    async fn get_current_user_info(
        pool: &ConnectPool,
        id: i32,
    ) -> Result<system_user::InfoWithDeptAndRole>;

    async fn check_user_permission(
        pool: &ConnectPool,
        user_id: i32,
        method: &str,
        path: &str,
    ) -> Result<bool>;

    async fn get_users_by_dept_id(
        pool: &ConnectPool,
        dept_id: i32,
    ) -> Result<Vec<system_user::Info>>;

    async fn batch_set_dept(
        pool: &ConnectPool,
        dept_id: Option<i32>,
        user_ids: Vec<i32>,
    ) -> Result<system_user::Info>;

    async fn create(
        pool: &ConnectPool,
        params: system_user::FormParamsForCreate,
    ) -> Result<system_user::Info>;

    async fn update(
        pool: &ConnectPool,
        id: i32,
        params: system_user::FormParamsForUpdate,
    ) -> Result<system_user::Info>;

    async fn update_password(
        pool: &ConnectPool,
        id: i32,
        password: &str,
    ) -> Result<system_user::Info>;

    async fn delete(pool: &ConnectPool, id: i32) -> Result<system_user::Info>;

    async fn info(pool: &ConnectPool, id: i32) -> Result<system_user::Info>;

    async fn paginate(
        pool: &ConnectPool,
        filter: system_user::Filter,
    ) -> Result<(Vec<system_user::Info>, i64)>;

    async fn set_last_login(
        pool: &ConnectPool,
        id: i32,
        login_ip: &str,
    ) -> Result<system_user::Info>;
}

pub trait SystemRole {
    async fn create(
        pool: &ConnectPool,
        params: system_role::FormParamsForCreate,
        currnt_user_id: i32,
        menu_ids: Option<Vec<i32>>,
    ) -> Result<system_role::Info>;

    async fn update(
        pool: &ConnectPool,
        id: i32,
        params: system_role::FormParamsForCreate,
        currnt_user_id: i32,
        menu_ids: Option<Vec<i32>>,
    ) -> Result<Info>;

    async fn delete(pool: &ConnectPool, id: i32) -> Result<Info>;

    async fn all(pool: &ConnectPool) -> Result<Vec<Info>>;

    async fn paginate(pool: &ConnectPool, filter: Filter) -> Result<PaginateResult<Vec<Info>>>;

    async fn info(pool: &ConnectPool, id: i32) -> Result<InfoWithMenuIds>;
}
