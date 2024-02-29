use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
pub struct Entity {
    /// 菜单ID
    pub id: i32,
    /// 父级ID
    pub parent_id: i32,
    /// 菜单类型：1.菜单，2.重定向/目录，3.外链，4.嵌套，5.按钮权限，6.接口权限
    pub r#type: i32,
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
