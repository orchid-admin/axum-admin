use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use async_recursion::async_recursion;
use service::sys_menu;

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
    let menus = sys_menu::get_menus(&client).await?;
    let parent_id = utils::tree::get_tree_start_parent_id::<sys_menu::Info>(&menus);
    let menu_tree =
        utils::tree::vec_to_tree_into::<sys_menu::Menu, sys_menu::Info>(&parent_id, &menus);

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
    let menus: Vec<sys_menu::Menu> = serde_json::from_str(&menu_string).expect("数据反序列化错误");
    insert(&client, menus, None).await?;
    Ok(())
}

#[async_recursion]
async fn insert(
    client: &service::Database,
    mut menus: Vec<sys_menu::Menu>,
    parent_info: Option<sys_menu::Info>,
) -> service::Result<()> {
    if let Some(parent) = parent_info {
        menus = menus
            .iter_mut()
            .map(|x| {
                x.set_parent_id(parent.id);
                x.clone()
            })
            .collect::<Vec<sys_menu::Menu>>();
    }
    for menu in menus {
        let children = menu.clone().get_children();
        let info = sys_menu::create(client, &menu.clone().get_title(), menu.into()).await?;

        if !children.is_empty() {
            insert(client, children, Some(info)).await?
        }
    }
    Ok(())
}
