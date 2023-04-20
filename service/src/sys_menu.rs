use crate::{
    now_time, prisma::system_menu, sys_role_menu, sys_user, Database, Result, ServiceError,
};

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
    title: String,
    params: MenuCreateParams,
) -> Result<MenuInfo> {
    let now_time = now_time();
    Ok(client
        .system_menu()
        .create_unchecked(title, now_time, now_time, params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn get_menus(client: &Database) -> Result<Vec<system_menu::Data>> {
    Ok(client
        .system_menu()
        .find_many(vec![system_menu::deleted_at::equals(None)])
        .exec()
        .await?)
}

pub async fn get_menus_tree(client: &Database) -> Result<Vec<MenuInfo>> {
    let menus = get_menus(client)
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<MenuInfo>>();
    Ok(menus_tree(0, menus))
}

pub async fn get_info(client: &Database, id: i32) -> Result<MenuInfo> {
    match client
        .system_menu()
        .find_unique(system_menu::id::equals(id))
        .exec()
        .await?
        .map(|x| x.into())
    {
        Some(x) => Ok(x),
        None => Err(ServiceError::DataNotFound),
    }
}

pub async fn get_user_menu(client: &Database, user_id: i32) -> Result<Vec<MenuInfo>> {
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
                    .collect::<Vec<MenuInfo>>();
                    menus_tree(0, role_menus)
                }
                None => vec![],
            },
            None => vec![],
        },
    )
}

fn menus_tree(parent_id: i32, menus: Vec<MenuInfo>) -> Vec<MenuInfo> {
    menus
        .clone()
        .into_iter()
        .filter(|x| x.parent_id.eq(&Some(parent_id)))
        .map(|x| {
            let mut menu = x.clone();
            let children = menus_tree(x.id.unwrap(), menus.clone());
            menu.children = Some(children);
            menu
        })
        .collect::<Vec<MenuInfo>>()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ts_rs::TS, utoipa::ToSchema)]
#[ts(export)]
pub struct MenuInfo {
    /// 菜单ID
    pub id: Option<i32>,
    /// 父级ID
    pub parent_id: Option<i32>,
    /// 路径
    pub path: String,
    /// 路由名称
    pub name: String,
    /// 组件地址
    pub component: String,
    /// 重定向
    pub redirect: Option<String>,
    /// 详细
    pub meta: MenuInfoMeta,
    /// 子菜单
    pub children: Option<Vec<MenuInfo>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ts_rs::TS, utoipa::ToSchema)]
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

impl From<system_menu::Data> for MenuInfo {
    fn from(menu: system_menu::Data) -> Self {
        Self {
            id: Some(menu.id),
            parent_id: Some(menu.parent_id),
            path: menu.path,
            name: menu.router_name,
            component: menu.component,
            redirect: match menu.redirect.is_empty() {
                false => Some(menu.redirect),
                true => None,
            },
            meta: MenuInfoMeta {
                title: menu.meta_title,
                is_link: match menu.meta_link.is_empty() {
                    false => Some(menu.meta_link),
                    true => None,
                },
                is_hide: menu.is_link,
                is_keep_alive: menu.meta_is_keep_alive,
                is_affix: menu.meta_is_affix,
                is_iframe: menu.meta_is_iframe,
                icon: match menu.meta_icon.is_empty() {
                    false => Some(menu.meta_icon),
                    true => None,
                },
            },
            children: None,
        }
    }
}
