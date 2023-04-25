use async_recursion::async_recursion;
pub async fn init() -> service::Result<()> {
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Data {
        code: i32,
        r#type: String,
        data: Vec<service::sys_menu::MenuInfo>,
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
    info: service::sys_menu::MenuInfo,
) -> service::Result<()> {
    let params = service::sys_menu::MenuCreateParams {
        parent_id,
        r#type: None,
        router_name: Some(info.name),
        component: Some(info.component),
        is_link: Some(info.meta.is_link.is_some()),
        path: Some(info.path),
        redirect: info.redirect,
        btn_power: None,
        sort: None,
        meta_icon: info.meta.icon,
        meta_is_hide: Some(info.meta.is_hide),
        meta_is_keep_alive: Some(info.meta.is_keep_alive),
        meta_is_affix: Some(info.meta.is_affix),
        meta_link: info.meta.is_link,
        meta_is_iframe: Some(info.meta.is_iframe),
    };
    let data = service::sys_menu::create(client, &info.meta.title, params).await?;
    if let Some(children) = info.children {
        for child in children {
            let res = insert(client, Some(data.id.unwrap()), child).await;
            if let Err(err) = res {
                println!("error: {:#?}", err);
            }
        }
    }
    Ok(())
}
