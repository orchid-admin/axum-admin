use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use async_recursion::async_recursion;
use service::system_menu_service;

async fn init_database() -> service::Result<service::Database> {
    service::Database::new(service::DatabaseConfig::default()).await
}

fn get_file_path() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("data")
        .join("menu.json")
}
pub async fn export() -> service::Result<()> {
    let client = init_database().await?;
    let menus = system_menu_service::get_menus(&client).await?;
    let parent_id = utils::tree::get_tree_start_parent_id::<system_menu_service::Info>(&menus);
    let menu_tree = utils::tree::vec_to_tree_into::<
        system_menu_service::Menu,
        system_menu_service::Info,
    >(&parent_id, &menus);

    let content = serde_json::to_string(&menu_tree).expect("数据序列化错误");
    let file_path = get_file_path();
    let mut file = fs::File::create(file_path).expect("./data/menu.json文件不存在");
    file.write_all(content.as_bytes()).expect("数据写入失败");
    Ok(())
}

pub async fn import() -> service::Result<()> {
    let client = init_database().await?;
    let file_path = get_file_path();
    let mut file = fs::File::open(file_path).expect("./data/menu.json文件不存在");
    let mut menu_string = String::new();
    file.read_to_string(&mut menu_string)
        .expect("读取文件数据失败");
    let menus: Vec<system_menu_service::Menu> =
        serde_json::from_str(&menu_string).expect("数据反序列化错误");
    insert(&client, menus, None).await?;
    Ok(())
}

#[async_recursion]
async fn insert(
    client: &service::Database,
    mut menus: Vec<system_menu_service::Menu>,
    parent_info: Option<system_menu_service::Info>,
) -> service::Result<()> {
    if let Some(parent) = parent_info {
        menus = menus
            .iter_mut()
            .map(|x| {
                x.set_parent_id(parent.id);
                x.clone()
            })
            .collect::<Vec<system_menu_service::Menu>>();
    }
    for menu in menus {
        let children = menu.clone().get_children();
        let info =
            system_menu_service::create(client, &menu.clone().get_title(), menu.into()).await?;

        if !children.is_empty() {
            insert(client, children, Some(info)).await?
        }
    }
    Ok(())
}
