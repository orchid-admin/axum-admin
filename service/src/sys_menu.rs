use crate::{now_time, prisma::system_menu, Database, Result};

system_menu::partial_unchecked!(MenuCreate { parent_id r#type is_link meta_is_hide meta_is_keep_alive meta_is_keep_alive meta_is_affix meta_link meta_is_iframe meta_roles btn_power sort});

pub async fn create(client: Database, params: MenuCreateParams) -> Result<system_menu::Data> {
    let now_time = now_time();

    let _params = vec![];
    Ok(client
        .system_menu()
        .create_unchecked(
            params.router_name,
            params.component_alias,
            params.path,
            params.redirect,
            params.title,
            params.icon,
            now_time,
            now_time,
            vec![],
        )
        .exec()
        .await?)
}

pub struct MenuCreateParams {
    pub parent_id: Option<i32>,
    pub r#type: String,
    pub router_name: String,
    pub component_alias: String,
    pub is_link: bool,
    pub path: String,
    pub redirect: String,
    pub btn_power: String,
    pub sort: i64,
    pub title: String,
    pub icon: String,
    pub is_hide: bool,
    pub is_keep_alive: bool,
    pub is_affix: bool,
    pub link: String,
    pub is_iframe: bool,
}
