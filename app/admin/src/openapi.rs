macro_rules! api_doc_tag {
    ($name:literal, $description:literal) => {
        utoipa::openapi::tag::TagBuilder::new()
            .name($name)
            .description(Some($description))
            .build()
    };
}
#[macro_export]
macro_rules! api_doc_path {
    ($($name:ident), *) => {
        {
            let mut temp_vec = Vec::new();
            $({
                temp_vec.push(($name::path(), $name::path_item(None)));
            })*
            temp_vec
        }
    };
}
#[macro_export]
macro_rules! api_doc_schema {
    ($($name:ident), *) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($name::schema());)*
            temp_vec
        }
    };
}

pub fn openapi(
    path_schemas: Vec<(
        Vec<(&str, utoipa::openapi::PathItem)>,
        Vec<(&str, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>,
    )>,
) -> utoipa::openapi::OpenApi {
    let mut paths = utoipa::openapi::Paths::new();
    let mut components = utoipa::openapi::Components::new();
    for (path_items, schemas) in path_schemas {
        for (key, item) in path_items {
            paths.paths.insert(key.to_owned(), item);
        }
        for (key, item) in schemas {
            components.schemas.insert(key.to_owned(), item);
        }
    }

    utoipa::openapi::OpenApiBuilder::new()
        .paths(paths)
        .components(Some(components))
        .tags(Some(vec![
            api_doc_tag!("auth", "授权"),
            api_doc_tag!("user", "用户"),
            api_doc_tag!("role", "角色"),
            api_doc_tag!("menu", "菜单"),
        ]))
        .build()
}
