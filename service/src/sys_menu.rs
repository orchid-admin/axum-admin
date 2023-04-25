use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::{prisma::system_menu, sys_role_menu, sys_user, Database, Result};

system_menu::partial_unchecked!(MenuCreateParams {
    parent_id
    r#type
    router_name
    component
    is_link
    path
    redirect
    meta_icon
    meta_is_hide
    meta_is_keep_alive
    meta_is_affix
    meta_link
    meta_is_iframe
    btn_power
    sort
});

pub async fn create(
    client: &Database,
    title: &str,
    params: MenuCreateParams,
) -> Result<system_menu::Data> {
    Ok(client
        .system_menu()
        .create_unchecked(title.to_owned(), params.to_params())
        .exec()
        .await?)
}

pub async fn get_menus(client: &Database) -> Result<Vec<system_menu::Data>> {
    Ok(client
        .system_menu()
        .find_many(vec![system_menu::deleted_at::equals(None)])
        .exec()
        .await?)
}

pub async fn get_menus_tree(client: &Database) -> Result<Vec<MenuTreeInfo>> {
    let menus = get_menus(client)
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<MenuTreeInfo>>();
    Ok(menus_tree(0, menus))
}

pub async fn get_user_menu(client: &Database, user_id: i32) -> Result<Vec<MenuTreeInfo>> {
    Ok(
        match sys_user::get_current_user_info(client, user_id).await? {
            Some(user_permission) => match user_permission.role {
                Some(role) => {
                    let role_menus = match role.sign.as_str() {
                        "admin" => get_menus(client).await?,
                        _ => sys_role_menu::get_role_menus(client, role.id).await?,
                    }
                    .into_iter()
                    .filter(|x| x.r#type.eq(&"menu"))
                    .collect::<Vec<system_menu::Data>>()
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<MenuTreeInfo>>();
                    menus_tree(0, role_menus)
                }
                None => vec![],
            },
            None => vec![],
        },
    )
}

fn menus_tree(parent_id: i32, menus: Vec<MenuTreeInfo>) -> Vec<MenuTreeInfo> {
    menus
        .clone()
        .into_iter()
        .filter(|x| x.parent_id.eq(&parent_id))
        .map(|x| {
            let mut menu = x.clone();
            let children = menus_tree(x.id, menus.clone());
            menu.children = children;
            menu
        })
        .collect::<Vec<MenuTreeInfo>>()
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, ToSchema)]
#[ts(export)]
pub struct MenuBase {
    /// 类型，menu:菜单,btn:按钮权限
    pub r#type: String,
    /// 路径
    pub path: String,
    /// 路由名称
    pub name: String,
    /// 组件地址
    pub component: String,
    /// 重定向
    pub redirect: String,
    /// 详细
    pub meta: MenuInfoMeta,
}

impl From<system_menu::Data> for MenuBase {
    fn from(menu: system_menu::Data) -> Self {
        let meta = menu.clone().into();
        Self {
            r#type: menu.r#type,
            path: menu.path,
            name: menu.router_name,
            component: menu.component,
            redirect: menu.redirect,
            meta,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, ToSchema)]
#[ts(export)]
pub struct MenuTreeInfo {
    /// 菜单ID
    pub id: i32,
    /// 父级ID
    pub parent_id: i32,
    #[serde(flatten)]
    pub base_info: MenuBase,
    /// 排序
    pub sort: i32,
    /// 子菜单
    pub children: Vec<MenuTreeInfo>,
}

impl From<system_menu::Data> for MenuTreeInfo {
    fn from(menu: system_menu::Data) -> Self {
        Self {
            id: menu.id,
            parent_id: menu.parent_id,
            base_info: menu.clone().into(),
            sort: menu.sort,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, ToSchema)]
#[ts(export)]
pub struct MenuInfoMeta {
    /// 菜单名称
    pub title: String,
    /// 是否是链接
    #[serde(rename = "isLink")]
    pub is_link: Option<String>,
    /// 是否隐藏
    #[serde(rename = "isHide")]
    pub is_hide: bool,
    /// 是否保持
    #[serde(rename = "isKeepAlive")]
    pub is_keep_alive: bool,
    #[serde(rename = "isAffix")]
    pub is_affix: bool,
    /// 是否内嵌
    #[serde(rename = "isIframe")]
    pub is_iframe: bool,
    /// 图标
    pub icon: Option<String>,
}

impl From<system_menu::Data> for MenuInfoMeta {
    fn from(menu: system_menu::Data) -> Self {
        Self {
            title: menu.meta_title,
            is_link: match menu.meta_link.is_empty() {
                false => Some(menu.meta_link),
                true => None,
            },
            is_hide: menu.meta_is_hide,
            is_keep_alive: menu.meta_is_keep_alive,
            is_affix: menu.meta_is_affix,
            is_iframe: menu.meta_is_iframe,
            icon: match menu.meta_icon.is_empty() {
                false => Some(menu.meta_icon),
                true => None,
            },
        }
    }
}
