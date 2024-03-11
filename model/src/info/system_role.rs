use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub name: String,
    pub sign: String,
    pub describe: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InfoForUserInfo {
    pub id: i32,
    pub name: String,
}

impl From<Info> for InfoForUserInfo {
    fn from(value: Info) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    // other fields
    pub id: Option<i32>,
    pub sign: Option<String>,
}

/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_roles)]
pub struct FormParamsForCreate {
    pub name: String,
    pub sign: String,
    pub describe: String,
    pub status: i32,
    pub sort: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
