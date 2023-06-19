use crate::{
    prisma::{system_menu, SortOrder},
    sys_role, sys_role_menu, sys_user, Database, Result, ServiceError,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::{
    datetime::{now_time, to_local_string},
    tree::{get_tree_start_parent_id, vec_to_tree_into, Tree, TreeInfo},
};

pub async fn create(db: &Database, title: &str, params: CreateParams) -> Result<Info> {
    Ok(db
        .client
        .system_menu()
        .create_unchecked(title.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: UpdateParams) -> Result<Info> {
    Ok(db
        .client
        .system_menu()
        .update_unchecked(system_menu::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .system_menu()
        .update(
            system_menu::id::equals(id),
            vec![system_menu::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}

pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
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
    db: &Database,
    user_id: i32,
    query_params: &SearchParams,
) -> Result<Vec<Menu>> {
    let infos = get_user_menus(db, user_id, query_params).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    Ok(vec_to_tree_into::<Menu, Info>(&parent_id, &infos))
}

pub async fn get_user_slide_menu_trees(
    db: &Database,
    user_id: i32,
    query_params: &SearchParams,
) -> Result<Vec<UserMenu>> {
    let infos = get_user_menus(db, user_id, query_params).await?;
    let parent_id = get_tree_start_parent_id::<Info>(&infos);
    Ok(vec_to_tree_into::<UserMenu, Info>(&parent_id, &infos))
}

pub async fn get_user_menus_by_menu_ids(
    db: &Database,
    user_id: i32,
    menu_ids: Vec<i32>,
) -> Result<Vec<Info>> {
    Ok(match menu_ids.is_empty() {
        true => vec![],
        false => get_menus_by_user_id(db, user_id)
            .await?
            .into_iter()
            .filter(|x| menu_ids.clone().into_iter().any(|z| x.id.eq(&z)))
            .collect::<Vec<Info>>(),
    })
}

pub fn filter_menu_types(menu_type: Option<Vec<MenuType>>, x: Vec<Info>) -> Vec<Info> {
    match menu_type {
        Some(t) => x
            .into_iter()
            .filter(|x| t.contains(&x.r#type))
            .collect::<Vec<Info>>(),
        None => x,
    }
}
pub async fn get_menu_by_role(db: &Database, role: Option<sys_role::Info>) -> Result<Vec<Info>> {
    Ok(match role {
        Some(role) => {
            if role.get_sign().as_str().eq(&db.config.admin_role_sign) {
                get_menus(db).await?
            } else {
                sys_role_menu::get_role_menus(db, role.get_id()).await?
            }
        }
        None => vec![],
    })
}

pub async fn get_menu_id_by_api_request(
    db: &Database,
    method: &str,
    path: &str,
) -> Result<Option<(i32, String)>> {
    let info: Option<Info> = db
        .client
        .system_menu()
        .find_first(vec![
            system_menu::api_method::equals(method.to_owned()),
            system_menu::api_url::equals(path.to_owned()),
        ])
        .exec()
        .await?
        .map(|x| x.into());
    if let Some(x) = info {
        let mut parent_names = get_parent_names(db, x.parent_id).await?;
        parent_names.push(x.title);
        return Ok(Some((x.id, parent_names.join("/"))));
    }
    Ok(None)
}

async fn get_user_menus(
    db: &Database,
    user_id: i32,
    query_params: &SearchParams,
) -> Result<Vec<Info>> {
    get_menus_by_user_id(db, user_id)
        .await
        .map(|x| filter_menu_by_search(query_params, x))
}

fn filter_menu_by_search(query_params: &SearchParams, x: Vec<Info>) -> Vec<Info> {
    let type_filters = match &query_params.menu_types {
        Some(t) => x
            .into_iter()
            .filter(|x| t.contains(&x.r#type))
            .collect::<Vec<Info>>(),
        None => x,
    };
    match &query_params.keyword {
        Some(keyword) => {
            if !keyword.is_empty() {
                return type_filters
                    .into_iter()
                    .filter(|x| {
                        let k = keyword.to_owned();
                        x.title.contains(&k)
                            || x.router_name.contains(&k)
                            || x.router_component.contains(&k)
                            || x.router_path.contains(&k)
                            || x.redirect.contains(&k)
                            || x.link.contains(&k)
                            || x.iframe.contains(&k)
                            || x.btn_auth.contains(&k)
                            || x.api_url.contains(&k)
                            || x.api_method.contains(&k)
                    })
                    .collect::<Vec<Info>>();
            }
            type_filters
        }
        None => type_filters,
    }
}

pub async fn get_menus(db: &Database) -> Result<Vec<Info>> {
    Ok(db
        .client
        .system_menu()
        .find_many(vec![system_menu::deleted_at::equals(None)])
        .order_by(system_menu::sort::order(SortOrder::Asc))
        .order_by(system_menu::id::order(SortOrder::Asc))
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

async fn get_menus_by_user_id(db: &Database, user_id: i32) -> Result<Vec<Info>> {
    let user_permission = sys_user::get_current_user_info(db, user_id).await?;
    get_menu_by_role(db, user_permission.get_role()).await
}

#[async_recursion::async_recursion]
async fn get_parent_names(db: &Database, menu_id: i32) -> Result<Vec<String>> {
    let mut parent_names = vec![];
    let menu = db
        .client
        .system_menu()
        .find_unique(system_menu::id::equals(menu_id))
        .exec()
        .await?;
    if let Some(x) = menu {
        parent_names = get_parent_names(db, x.parent_id).await?;
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
            id: value.id,
            parent_id: value.parent_id,
            router_name: match value.router_name.is_empty() {
                false => value.router_name,
                true => value.router_path.replace('/', "_"),
            },
            router_component: value.router_component,
            router_path: match value.r#type {
                MenuType::Menu => value.router_path,
                _ => format!("/{}", value.title.replace('.', "_")),
            },
            redirect: value.redirect,
            meta: UserMenuMeta {
                title: value.title,
                icon: value.icon,
                is_link: match value.r#type {
                    MenuType::Link => value.link,
                    MenuType::Iframe => value.iframe.clone(),
                    _ => "".to_owned(),
                },
                is_iframe: !value.iframe.is_empty() && value.r#type.eq(&MenuType::Iframe),
                is_hide: value.is_hide,
                is_keep_alive: value.is_keep_alive,
                is_affix: value.is_affix,
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
    pub is_iframe: bool,
    /// 是否隐藏
    #[serde(rename = "isHide")]
    pub is_hide: bool,
    /// 是否开启keep_alive
    #[serde(rename = "isKeepAlive")]
    pub is_keep_alive: bool,
    /// 是否固定
    #[serde(rename = "isAffix")]
    pub is_affix: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    #[serde(flatten)]
    info: Info,
    children: Vec<Menu>,
}

impl Menu {
    pub fn get_children(self) -> Vec<Menu> {
        self.children
    }

    pub fn get_title(self) -> String {
        self.info.title
    }

    pub fn set_parent_id(&mut self, parent_id: i32) {
        self.info.parent_id = parent_id;
    }
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

pub struct SearchParams {
    keyword: Option<String>,
    menu_types: Option<Vec<MenuType>>,
}

impl SearchParams {
    pub fn new(keyword: Option<String>, menu_types: Option<Vec<MenuType>>) -> Self {
        Self {
            keyword,
            menu_types,
        }
    }
}

system_menu::partial_unchecked!(CreateParams {
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

impl From<Menu> for CreateParams {
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

system_menu::partial_unchecked!(UpdateParams {
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
