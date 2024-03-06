use async_recursion::async_recursion;
use config::Config;
use service::system_menu;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

fn get_file_path() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("data")
        .join("menu.json")
}
pub async fn export() -> service::Result<()> {
    let config = Config::load();
    let database_connect_pool = model::connect::DbConnectPool::new(&config.database_url())?;
    let menus = system_menu::get_menus(&database_connect_pool).await?;
    let parent_id = utils::tree::get_tree_start_parent_id::<system_menu::Info>(&menus);
    let menu_tree =
        utils::tree::vec_to_tree_into::<system_menu::Menu, system_menu::Info>(&parent_id, &menus);

    let content = serde_json::to_string(&menu_tree).expect("数据序列化错误");
    let file_path = get_file_path();
    let mut file = fs::File::create(file_path).expect("./data/menu.json文件不存在");
    file.write_all(content.as_bytes()).expect("数据写入失败");
    Ok(())
}

pub async fn import() -> service::Result<()> {
    let config = Config::load();
    let database_connect_pool = model::connect::DbConnectPool::new(&config.database_url())?;
    let file_path = get_file_path();
    let mut file = fs::File::open(file_path).expect("./data/menu.json文件不存在");
    let mut menu_string = String::new();
    file.read_to_string(&mut menu_string)
        .expect("读取文件数据失败");
    let menus: Vec<system_menu::Menu> =
        serde_json::from_str(&menu_string).expect("数据反序列化错误");
    insert(&database_connect_pool, menus, None).await?;
    Ok(())
}

#[async_recursion]
async fn insert(
    database_connect_pool: &model::connect::DbConnectPool,
    mut menus: Vec<system_menu::Menu>,
    parent_info: Option<system_menu::Info>,
) -> service::Result<()> {
    if let Some(parent) = parent_info {
        menus = menus
            .iter_mut()
            .map(|x| {
                x.set_parent_id(parent.id());
                x.clone()
            })
            .collect::<Vec<system_menu::Menu>>();
    }
    for menu in menus {
        let children = menu.clone().children.clone();
        let info = system_menu::create(database_connect_pool, menu.into()).await?;

        if !children.is_empty() {
            insert(database_connect_pool, children, Some(info)).await?
        }
    }
    Ok(())
}
