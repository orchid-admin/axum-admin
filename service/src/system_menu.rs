use crate::{Result, ServiceError};
use model::{connect::DbConnectPool as ConnectPool, system_menu, system_role, system_role_menu};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::tree::{get_tree_start_parent_id, vec_to_tree_into, Tree, TreeInfo};

pub async fn create(pool: &ConnectPool, params: system_menu::FormParamsForCreate) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_menu::Entity::create(&mut conn, &params)
        .await?
        .into())
}

pub async fn update(
    pool: &ConnectPool,
    id: i32,
    params: system_menu::FormParamsForUpdate,
) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_menu::Entity::update(&mut conn, id, params)
        .await?
        .into())
}

pub async fn delete(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_menu::Entity::soft_delete(&mut conn, id)
        .await?
        .into())
}

pub async fn info(pool: &ConnectPool, id: i32) -> Result<Info> {
    let mut conn = pool.conn().await?;
    Ok(system_menu::Entity::find(
        &mut conn,
        &system_menu::Filter {
            id: Some(id),
            ..Default::default()
        },
    )
    .await?
    .ok_or(ServiceError::DataNotFound)?
    .into())
}

pub async fn get_user_menu_trees<F: Into<Filter>>(
    pool: &ConnectPool,
    user_id: i32,
    filter: F,
) -> Result<Vec<Menu>> {
    let infos = get_user_menus(pool, user_id, filter).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    Ok(vec_to_tree_into::<Menu, Info>(&parent_id, &infos))
}

pub async fn get_user_slide_menu_trees<F: Into<Filter>>(
    pool: &ConnectPool,
    user_id: i32,
    filter: F,
) -> Result<Vec<UserMenu>> {
    let infos = get_user_menus(pool, user_id, filter).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    Ok(vec_to_tree_into::<UserMenu, Info>(&parent_id, &infos))
}

pub async fn get_user_menus_by_menu_ids(
    pool: &ConnectPool,
    user_id: i32,
    menu_ids: Vec<i32>,
) -> Result<Vec<Info>> {
    Ok(match menu_ids.is_empty() {
        true => vec![],
        false => get_menus_by_user_id(pool, user_id)
            .await?
            .into_iter()
            .filter(|x| menu_ids.clone().into_iter().any(|z| x.info.id.eq(&z)))
            .collect::<Vec<Info>>(),
    })
}

pub fn filter_menu_types(menu_type: Option<Vec<MenuType>>, x: Vec<Info>) -> Vec<Info> {
    match menu_type {
        Some(t) => x
            .into_iter()
            .filter(|x| t.contains(&x.menu_type))
            .collect::<Vec<Info>>(),
        None => x,
    }
}
pub async fn get_menu_by_role(
    pool: &ConnectPool,
    role: Option<system_role::Entity>,
) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;
    Ok(match role {
        Some(role) => system_role_menu::Entity::get_role_menus(&mut conn, role.id)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<Info>>(),
        None => vec![],
    })
}

pub async fn get_menu_id_by_api_request(
    pool: &ConnectPool,
    method: &str,
    path: &str,
) -> Result<Option<(i32, String)>> {
    let mut conn = pool.conn().await?;
    let info = system_menu::Entity::find(
        &mut conn,
        &system_menu::Filter {
            api_method: Some(method.to_owned()),
            api_url: Some(path.to_owned()),
            ..Default::default()
        },
    )
    .await?;
    if let Some(x) = info {
        let mut parent_names = get_parent_names(pool, x.parent_id).await?;
        parent_names.push(x.title);
        return Ok(Some((x.id, parent_names.join("/"))));
    }
    Ok(None)
}

async fn get_user_menus<F: Into<Filter>>(
    pool: &ConnectPool,
    user_id: i32,
    filter: F,
) -> Result<Vec<Info>> {
    get_menus_by_user_id(pool, user_id)
        .await
        .map(|x| filter_menu_by_search(filter, x))
}

fn filter_menu_by_search<F: Into<Filter>>(filter: F, x: Vec<Info>) -> Vec<Info> {
    let filter: Filter = filter.into();
    let type_filters = match &filter.menu_types {
        Some(t) => x
            .into_iter()
            .filter(|x| t.contains(&x.menu_type))
            .collect::<Vec<Info>>(),
        None => x,
    };
    match &filter.keyword {
        Some(keyword) => {
            if !keyword.is_empty() {
                return type_filters
                    .into_iter()
                    .filter(|x| {
                        let k = keyword.to_owned();
                        x.info.title.contains(&k)
                            || x.info.router_name.contains(&k)
                            || x.info.router_component.contains(&k)
                            || x.info.router_path.contains(&k)
                            || x.info.redirect.contains(&k)
                            || x.info.link.contains(&k)
                            || x.info.iframe.contains(&k)
                            || x.info.btn_auth.contains(&k)
                            || x.info.api_url.contains(&k)
                            || x.info.api_method.contains(&k)
                    })
                    .collect::<Vec<Info>>();
            }
            type_filters
        }
        None => type_filters,
    }
}

pub async fn get_menus(pool: &ConnectPool) -> Result<Vec<Info>> {
    let mut conn = pool.conn().await?;
    Ok(
        system_menu::Entity::query(&mut conn, &system_menu::Filter::default())
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<Info>>(),
    )
}

async fn get_menus_by_user_id(pool: &ConnectPool, user_id: i32) -> Result<Vec<Info>> {
    let user_permission = super::system_user::get_current_user_info(pool, user_id).await?;
    get_menu_by_role(pool, user_permission.role).await
}

#[async_recursion::async_recursion]
async fn get_parent_names(pool: &ConnectPool, menu_id: i32) -> Result<Vec<String>> {
    let mut parent_names = vec![];
    let mut conn = pool.conn().await?;
    let menu = system_menu::Entity::find(
        &mut conn,
        &system_menu::Filter {
            id: Some(menu_id),
            ..Default::default()
        },
    )
    .await?;
    if let Some(x) = menu {
        parent_names = get_parent_names(pool, x.parent_id).await?;
        parent_names.push(x.title);
    }
    Ok(parent_names)
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

impl Default for MenuType {
    fn default() -> Self {
        Self::Menu
    }
}
impl From<i32> for MenuType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Menu,
            2 => Self::Redirect,
            3 => Self::Link,
            4 => Self::Iframe,
            5 => Self::BtnAuth,
            6 => Self::Api,
            _ => Self::Menu,
        }
    }
}

impl From<MenuType> for i32 {
    fn from(value: MenuType) -> Self {
        match value {
            MenuType::Menu => 1,
            MenuType::Redirect => 2,
            MenuType::Link => 3,
            MenuType::Iframe => 4,
            MenuType::BtnAuth => 5,
            MenuType::Api => 6,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserMenu {
    /// 菜单ID
    #[serde(skip)]
    pub id: i32,
    /// 父级ID
    #[serde(skip)]
    pub parent_id: i32,
    /// 路由名称
    #[serde(rename = "name")]
    pub router_name: String,
    /// 组件地址
    #[serde(rename = "component")]
    pub router_component: String,
    /// 路径
    #[serde(rename = "path")]
    pub router_path: String,
    /// 重定向
    pub redirect: String,
    /// Meta信息
    pub meta: UserMenuMeta,
    pub children: Vec<UserMenu>,
}

impl Tree<UserMenu> for UserMenu {
    fn set_child(&mut self, data: Vec<UserMenu>) {
        self.children = data;
    }
}

impl From<Info> for UserMenu {
    fn from(value: Info) -> Self {
        Self {
            id: value.info.id,
            parent_id: value.info.parent_id,
            router_name: match value.info.router_name.is_empty() {
                false => value.info.router_name,
                true => value.info.router_path.replace('/', "_"),
            },
            router_component: value.info.router_component,
            router_path: match value.info.r#type.into() {
                MenuType::Menu => value.info.router_path,
                _ => format!("/{}", value.info.title.replace('.', "_")),
            },
            redirect: value.info.redirect,
            meta: UserMenuMeta {
                title: value.info.title,
                icon: value.info.icon,
                is_link: match value.menu_type {
                    MenuType::Link => value.info.link,
                    MenuType::Iframe => value.info.iframe.clone(),
                    _ => "".to_owned(),
                },
                is_iframe: match !value.info.iframe.is_empty()
                    && value.menu_type.eq(&MenuType::Iframe)
                {
                    true => 1,
                    false => 0,
                },
                is_hide: value.info.is_hide,
                is_keep_alive: value.info.is_keep_alive,
                is_affix: value.info.is_affix,
            },
            children: vec![],
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserMenuMeta {
    /// 菜单名称
    pub title: String,
    /// 图标
    pub icon: String,
    /// 外链地址
    #[serde(rename = "isLink")]
    pub is_link: String,
    /// 内嵌地址
    #[serde(rename = "isIframe")]
    pub is_iframe: i32,
    /// 是否隐藏
    #[serde(rename = "isHide")]
    pub is_hide: i32,
    /// 是否开启keep_alive
    #[serde(rename = "isKeepAlive")]
    pub is_keep_alive: i32,
    /// 是否固定
    #[serde(rename = "isAffix")]
    pub is_affix: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    #[serde(flatten)]
    info: Info,
    pub children: Vec<Menu>,
}

impl Menu {
    pub fn get_title(self) -> String {
        self.info.info.title
    }

    pub fn set_parent_id(&mut self, parent_id: i32) {
        self.info.info.parent_id = parent_id;
    }
}

impl Tree<Menu> for Menu {
    fn set_child(&mut self, data: Vec<Menu>) {
        self.children = data;
    }
}

impl From<system_menu::Entity> for Menu {
    fn from(value: system_menu::Entity) -> Self {
        Self {
            info: value.into(),
            children: vec![],
        }
    }
}

impl From<Menu> for system_menu::Entity {
    fn from(value: Menu) -> Self {
        value.info.info
    }
}

impl From<Info> for Menu {
    fn from(value: Info) -> Self {
        Self {
            info: value,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    #[serde(flatten)]
    info: system_menu::Entity,
    #[serde(skip)]
    menu_type: MenuType,
}
impl Info {
    pub fn id(&self) -> i32 {
        self.info.id
    }

    pub fn btn_auth(&self) -> String {
        self.info.btn_auth.clone()
    }
}
impl TreeInfo for Info {
    fn get_id(&self) -> i32 {
        self.info.id
    }
    fn get_parent_id(&self) -> i32 {
        self.info.parent_id
    }
}
impl From<system_menu::Entity> for Info {
    fn from(value: system_menu::Entity) -> Self {
        Self {
            info: value.clone(),
            menu_type: value.r#type.into(),
        }
    }
}
impl Info {
    pub fn check_request_permission(&self, method: &str, path: &str) -> bool {
        self.info.api_method.eq(method) && self.info.api_url.eq(path)
    }
}

impl From<Menu> for system_menu::FormParamsForCreate {
    fn from(value: Menu) -> Self {
        let value: system_menu::Entity = value.into();
        let value: system_menu::FormParamsForCreate = value.into();
        value
    }
}
#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub menu_types: Option<Vec<MenuType>>,
}
impl From<Filter> for system_menu::Filter {
    fn from(value: Filter) -> Self {
        Self {
            keyword: value.keyword,
            ..Default::default()
        }
    }
}

pub type FormParamsForCreate = system_menu::FormParamsForCreate;
