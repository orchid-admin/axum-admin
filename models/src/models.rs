use getset::Getters;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Getters)]
pub struct SystemUser {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub")]
    username: String,
    nickname: String,
    role_id: Option<i32>,
    dept_id: Option<i32>,
    phone: String,
    email: String,
    sex: i32,
    #[serde(skip)]
    #[getset(get = "pub")]
    password: String,
    #[serde(skip)]
    #[getset(get = "pub")]
    salt: String,
    describe: String,
    expire_time: Option<String>,
    status: i32,
    created_at: String,
    last_login_ip: String,
    last_login_time: Option<String>,
    #[getset(get = "pub")]
    dept: Option<SystemDept>,
    #[getset(get = "pub")]
    role: Option<SystemRole>,
}

#[derive(Debug, Clone, Serialize, Getters)]
pub struct SystemDept {
    #[getset(get = "pub")]
    id: i32,
    parent_id: i32,
    name: String,
    person_name: String,
    person_phone: String,
    person_email: String,
    describe: String,
    status: i32,
    sort: i32,
    created_at: String,
}
#[derive(Debug, Clone, Serialize, Getters)]
pub struct SystemRole {
    #[getset(get = "pub")]
    id: i32,
    name: String,
    #[getset(get = "pub")]
    sign: String,
    describe: String,
    status: i32,
    sort: i32,
    created_at: String,
    menu_ids: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MenuType {
    /// 1.菜单
    Menu = 1,
    /// 2.重定向/目录
    Redirect = 2,
    /// 3.外链
    Link = 3,
    /// 4.嵌套
    Iframe = 4,
    /// 5.按钮权限
    BtnAuth = 5,
    /// 6.接口权限
    Api = 6,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemMenu {
    /// 菜单ID
    pub id: i32,
    /// 父级ID
    pub parent_id: i32,
    /// 菜单类型：1.菜单，2.重定向/目录，3.外链，4.嵌套，5.按钮权限，6.接口权限
    pub r#type: MenuType,
    /// 菜单名称
    pub title: String,
    /// 图标
    pub icon: String,
    /// 路由名称
    pub router_name: String,
    /// 组件地址
    pub router_component: String,
    /// 路径
    pub router_path: String,
    /// 重定向
    pub redirect: String,
    /// 外链地址
    pub link: String,
    /// 内嵌地址
    pub iframe: String,
    /// 按钮权限
    pub btn_auth: String,
    /// 接口地址
    pub api_url: String,
    /// 接口请求方法
    pub api_method: String,
    /// 是否隐藏
    pub is_hide: i32,
    /// 是否开启keep_alive
    pub is_keep_alive: i32,
    /// 是否固定
    pub is_affix: i32,
    /// 排序
    pub sort: i32,
    pub created_at: String,
    pub updated_time: String,
}

#[derive(Debug, Serialize)]
pub struct SystemDict {
    id: i32,
    name: String,
    sign: String,
    status: i32,
    remark: String,
    created_at: String,
    data: Vec<SystemDictData>,
}

#[derive(Debug, Serialize)]
pub struct SystemDictData {
    id: i32,
    dict_id: i32,
    dict: SystemDict,
    label: String,
    value: i32,
    status: i32,
    sort: i32,
    remark: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
pub struct SystemLoginLog {
    id: i32,
    user_id: i32,
    user: Option<SystemUser>,
    ip_address: String,
    ip_address_name: String,
    browser_agent: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
pub struct SystemActionLog {
    id: i32,
    user_id: i32,
    user: Option<SystemUser>,
    menu_id: i32,
    menu: Option<SystemMenu>,
    menu_names: String,
    ip_address: String,
    ip_address_name: String,
    browser_agent: String,
    created_at: String,
}

#[derive(Debug, Serialize, Getters)]
pub struct Member {
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

#[derive(Debug, Serialize)]
pub struct MemberBill {
    id: i32,
    user_id: i32,
    user: Option<Member>,
    r#type: i32,
    pm: i32,
    number: f32,
    created_at: String,
}

#[derive(Debug, Serialize)]
pub struct MemberTeam {
    id: i32,
    owner_uid: i32,
    owner: Option<Member>,
    parent_uid: i32,
    parent: Option<Member>,
    uid: i32,
    user: Option<Member>,
    level: i32,
    created_at: String,
}
