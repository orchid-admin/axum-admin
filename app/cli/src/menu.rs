use async_recursion::async_recursion;
use service::sys_menu::MenuCreateParams;
pub async fn init() -> service::Result<()> {
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Data {
        code: i32,
        r#type: String,
        data: Vec<service::sys_menu::Menu>,
    }
    let dir = std::env::current_dir().unwrap();
    let path = dir.join("data").join("adminMenu.json");
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = vec![];
    std::io::Read::read_to_end(&mut file, &mut contents).unwrap();
    let content = unsafe { String::from_utf8_unchecked(contents) };
    let data: Data = serde_json::from_str(&content).unwrap();
    let client = service::Database::new(service::DatabaseConfig::default()).await?;

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
    info: service::sys_menu::Menu,
) -> service::Result<()> {
    let mut params: MenuCreateParams = info.clone().into();
    params.parent_id = parent_id;
    let data = service::sys_menu::create(client, &info.clone().get_title(), params).await?;
    for child in info.get_children() {
        let res = insert(client, Some(data.id), child).await;
        if let Err(err) = res {
            println!("error: {:#?}", err);
        }
    }
    Ok(())
}
