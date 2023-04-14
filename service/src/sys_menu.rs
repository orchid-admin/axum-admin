use crate::{now_time, prisma::system_menu, Database, Result};

system_menu::partial_unchecked!(MenuCreate { parent_id r#type is_link meta_is_hide meta_is_keep_alive meta_is_keep_alive meta_is_affix meta_link meta_is_iframe meta_roles btn_power sort});

pub async fn create(client: Database, params: MenuCreateParams) -> Result<system_menu::Data> {
    let now_time = now_time();

    let _params = params.clone().to_params();
    Ok(client
        .system_menu()
        .create(
            params.router_name,
            params.component_alias,
            params.path,
            params.redirect,
            params.title,
            params.icon,
            now_time,
            now_time,
            _params,
        )
        .exec()
        .await?)
}

#[derive(Debug, Clone)]
pub struct MenuCreateParams {
    pub parent_id: Option<i32>,
    pub r#type: String,
    pub router_name: String,
    pub component_alias: String,
    pub is_link: bool,
    pub path: String,
    pub redirect: String,
    pub btn_power: String,
    pub sort: i32,
    pub title: String,
    pub icon: String,
    pub is_hide: bool,
    pub is_keep_alive: bool,
    pub is_affix: bool,
    pub link: String,
    pub is_iframe: bool,
}

impl MenuCreateParams {
    pub fn to_params(self) -> Vec<system_menu::SetParam> {
        let mut params = vec![
            system_menu::r#type::set(self.r#type),
            system_menu::is_link::set(self.is_link),
            system_menu::btn_power::set(self.btn_power),
            system_menu::sort::set(self.sort),
            system_menu::meta_is_hide::set(self.is_hide),
            system_menu::meta_is_keep_alive::set(self.is_keep_alive),
            system_menu::meta_link::set(self.link),
            system_menu::meta_is_iframe::set(self.is_iframe),
        ];
        if let Some(parent_id) = self.parent_id {
            params.push(system_menu::parent_id::set(parent_id));
        }
        params
    }
}
