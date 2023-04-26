use async_recursion::async_recursion;
pub async fn init() -> service::Result<()> {
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Data {
        code: i32,
        r#type: String,
        data: Vec<service::sys_menu::MenuTreeInfo>,
    }
    let dir = std::env::current_dir().unwrap();
    let path = dir.join("data").join("adminMenu.json");
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = vec![];
    std::io::Read::read_to_end(&mut file, &mut contents).unwrap();
    let content = unsafe { String::from_utf8_unchecked(contents) };
    let data: Data = serde_json::from_str(&content).unwrap();
    let client = service::new_client().await?;

    for info in data.data.into_iter() {
        let res = insert(&client, None, info).await;
        if let Err(err) = res {
            println!("error: {:#?}", err);
        }
    }
    Ok(())
}

#[async_recursion]
async fn insert(
    client: &service::Database,
    parent_id: Option<i32>,
    info: service::sys_menu::MenuTreeInfo,
) -> service::Result<()> {
    let params = service::sys_menu::MenuCreateParams {
        parent_id,
        r#type: None,
        router_name: Some(info.base_info.name),
        component: Some(info.base_info.component),
        is_link: Some(info.base_info.meta.is_link.is_some()),
        path: Some(info.base_info.path),
        redirect: Some(info.base_info.redirect),
        btn_power: None,
        sort: None,
        meta_icon: info.base_info.meta.icon,
        meta_is_hide: Some(info.base_info.meta.is_hide),
        meta_is_keep_alive: Some(info.base_info.meta.is_keep_alive),
        meta_is_affix: Some(info.base_info.meta.is_affix),
        meta_link: info.base_info.meta.is_link,
        meta_is_iframe: Some(info.base_info.meta.is_iframe),
    };
    let data = service::sys_menu::create(client, &info.base_info.meta.title, params).await?;
    for child in info.children {
        let res = insert(client, Some(data.id), child).await;
        if let Err(err) = res {
            println!("error: {:#?}", err);
        }
    }
    Ok(())
}
