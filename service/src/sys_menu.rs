use crate::{
    get_tree_start_parent_id, now_time,
    prisma::{system_menu, system_role},
    sys_role_menu, sys_user, to_local_string, vec_to_tree_into, Database, Result, ServiceError,
    Tree, TreeInfo, ADMIN_ROLE_SIGN,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub async fn create(client: &Database, title: &str, params: MenuCreateParams) -> Result<Info> {
    Ok(client
        .system_menu()
        .create_unchecked(title.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(client: &Database, id: i32, params: MenuCreateParams) -> Result<Info> {
    Ok(client
        .system_menu()
        .update_unchecked(system_menu::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn delete(client: &Database, id: i32) -> Result<Info> {
    Ok(client
        .system_menu()
        .update(
            system_menu::id::equals(id),
            vec![system_menu::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}

pub async fn info(client: &Database, id: i32) -> Result<Info> {
    Ok(client
        .system_menu()
        .find_first(vec![
            system_menu::id::equals(id),
            system_menu::deleted_at::equals(None),
        ])
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}

pub async fn get_user_menu_trees(
    client: &Database,
    user_id: i32,
    menu_type: Option<Vec<MenuType>>,
) -> Result<Vec<Menu>> {
    let infos = get_user_menus(client, user_id, menu_type).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    Ok(vec_to_tree_into::<Menu, Info>(&parent_id, &infos))
}

pub async fn get_user_slide_menu_trees(
    client: &Database,
    user_id: i32,
    menu_type: Option<Vec<MenuType>>,
) -> Result<Vec<UserMenu>> {
    let infos = get_user_menus(client, user_id, menu_type)
        .await?
        .into_iter()
        .filter(|x| !x.is_hide)
        .collect::<Vec<Info>>();
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    Ok(vec_to_tree_into::<UserMenu, Info>(&parent_id, &infos))
}

pub async fn get_user_menus_by_menu_ids(
    client: &Database,
    user_id: i32,
    menu_ids: Vec<i32>,
) -> Result<Vec<Info>> {
    Ok(match menu_ids.is_empty() {
        true => vec![],
        false => get_menus_by_user_id(client, user_id)
            .await?
            .into_iter()
            .filter(|x| menu_ids.clone().into_iter().any(|z| x.id.eq(&z)))
            .collect::<Vec<Info>>(),
    })
}

pub(crate) fn filter_menu_types(menu_type: Option<Vec<MenuType>>, x: Vec<Info>) -> Vec<Info> {
    match menu_type {
        Some(t) => x
            .into_iter()
            .filter(|x| t.contains(&x.r#type))
            .collect::<Vec<Info>>(),
        None => x,
    }
}
pub(crate) async fn get_menu_by_role(
    client: &Database,
    role: Option<system_role::Data>,
) -> Result<Vec<Info>> {
    Ok(match role {
        Some(role) => match role.sign.as_str() {
            ADMIN_ROLE_SIGN => get_menus(client).await?,
            _ => sys_role_menu::get_role_menus(client, role.id).await?,
        },
        None => vec![],
    })
}

async fn get_user_menus(
    client: &Database,
    user_id: i32,
    menu_type: Option<Vec<MenuType>>,
) -> Result<Vec<Info>> {
    get_menus_by_user_id(client, user_id)
        .await
        .map(|x| filter_menu_types(menu_type, x))
}

async fn get_menus(client: &Database) -> Result<Vec<Info>> {
    Ok(client
        .system_menu()
        .find_many(vec![system_menu::deleted_at::equals(None)])
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

async fn get_menus_by_user_id(client: &Database, user_id: i32) -> Result<Vec<Info>> {
    let user_permission = sys_user::get_current_user_info(client, user_id).await?;
    get_menu_by_role(client, user_permission.role).await
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MenuType {
    /// 1.菜单
    Menu = 1,
    /// 2.重定向
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
    /// 菜单名称
    pub title: String,
    /// 图标
    pub icon: String,
    /// 路由名称
    pub router_name: String,
    /// 组件地址
    pub router_component: String,
    /// 重定向
    pub redirect: String,
    /// 外链地址
    pub link: String,
    /// 内嵌地址
    pub iframe: String,
    /// 是否隐藏
    pub is_hide: bool,
    /// 是否开启keep_alive
    pub is_keep_alive: bool,
    /// 是否固定
    pub is_affix: bool,
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
            id: value.id,
            parent_id: value.parent_id,
            title: value.title,
            icon: value.icon,
            router_name: value.router_name,
            router_component: value.router_component,
            redirect: value.redirect,
            link: value.link,
            iframe: value.iframe,
            is_hide: value.is_hide,
            is_keep_alive: value.is_keep_alive,
            is_affix: value.is_affix,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    #[serde(flatten)]
    info: Info,
    children: Vec<Menu>,
}

impl Tree<Menu> for Menu {
    fn set_child(&mut self, data: Vec<Menu>) {
        self.children = data;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    /// 菜单ID
    pub id: i32,
    /// 父级ID
    pub parent_id: i32,
    /// 菜单类型：1.菜单，2.重定向，3.外链，4.嵌套，5.按钮权限，6.接口权限
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
    pub is_hide: bool,
    /// 是否开启keep_alive
    pub is_keep_alive: bool,
    /// 是否固定
    pub is_affix: bool,
    /// 排序
    pub sort: i32,
    pub created_at: String,
    pub updated_time: String,
}

impl From<system_menu::Data> for Info {
    fn from(value: system_menu::Data) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            r#type: value.r#type.into(),
            title: value.title,
            icon: value.icon,
            router_name: value.router_name,
            router_component: value.router_component,
            router_path: value.router_path,
            redirect: value.redirect,
            link: value.link,
            iframe: value.iframe,
            btn_auth: value.btn_auth,
            api_url: value.api_url,
            api_method: value.api_method,
            is_hide: value.is_hide,
            is_keep_alive: value.is_keep_alive,
            is_affix: value.is_affix,
            sort: value.sort,
            created_at: to_local_string(value.created_at),
            updated_time: to_local_string(value.updated_at),
        }
    }
}

impl TreeInfo for Info {
    fn get_parent_id(&self) -> i32 {
        self.parent_id
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
system_menu::partial_unchecked!(MenuCreateParams {
    parent_id
    r#type
    icon
    router_name
    router_component
    router_path
    redirect
    link
    iframe
    btn_auth
    api_url
    api_method
    is_hide
    is_keep_alive
    is_affix
    sort
});

impl From<Menu> for MenuCreateParams {
    fn from(value: Menu) -> Self {
        Self {
            parent_id: Some(value.info.parent_id),
            r#type: Some(value.info.r#type.into()),
            icon: Some(value.info.icon),
            router_name: Some(value.info.router_name),
            router_component: Some(value.info.router_component),
            router_path: Some(value.info.router_path),
            redirect: Some(value.info.redirect),
            link: Some(value.info.link),
            iframe: Some(value.info.iframe),
            btn_auth: Some(value.info.btn_auth),
            api_url: Some(value.info.api_url),
            api_method: Some(value.info.api_method),
            is_hide: Some(value.info.is_hide),
            is_keep_alive: Some(value.info.is_keep_alive),
            is_affix: Some(value.info.is_affix),
            sort: Some(value.info.sort),
        }
    }
}

system_menu::partial_unchecked!(MenuUpdateParams {
    parent_id
    r#type
    title
    icon
    router_name
    router_component
    router_path
    redirect
    link
    iframe
    btn_auth
    api_url
    api_method
    is_hide
    is_keep_alive
    is_affix
    sort
});
