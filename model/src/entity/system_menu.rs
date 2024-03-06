use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_menus, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::system_menus)]
pub struct Entity {
    pub id: i32,
    pub parent_id: i32,
    #[diesel(column_name = "type_")]
    pub r#type: i32,
    pub title: String,
    pub icon: String,
    pub router_name: String,
    pub router_component: String,
    pub router_path: String,
    pub redirect: String,
    pub link: String,
    pub iframe: String,
    pub btn_auth: String,
    pub api_url: String,
    pub api_method: String,
    pub is_hide: i32,
    pub is_keep_alive: i32,
    pub is_affix: i32,
    pub sort: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    deleted_at: Option<SystemTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find(conn: &mut Connect, filter: &Filter) -> Result<Option<Self>> {
        let table = system_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_menus::name.eq(_keyword));
        }

        let info = table
            .select(Entity::as_select())
            .first::<Entity>(conn)
            .await
            .optional()?;
        Ok(info)
    }
    /// query method
    pub async fn query(conn: &mut Connect, filter: &Filter) -> Result<Vec<Self>> {
        let table = system_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_menus::name.eq(_keyword));
        }

        let infos = table
            .select(Entity::as_select())
            .load::<Entity>(conn)
            .await?;
        Ok(infos)
    }
    /// paginate method
    pub async fn paginate(
        conn: &mut Connect,
        page: i64,
        limit: i64,
        filter: &Filter,
    ) -> Result<(Vec<Self>, i64)> {
        let table = system_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_menus::name.eq(_keyword));
        }

        Ok(table
            .select(Entity::as_select())
            .paginate(page)
            .per_page(limit)
            .load_and_count_pages::<Entity>(conn)
            .await?)
    }
    /// insert method
    pub async fn insert(conn: &mut Connect, params: Vec<FormParamsForCreate>) -> Result<Vec<Self>> {
        Ok(insert_into(system_menus::dsl::system_menus)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_menus::dsl::system_menus)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_menus::dsl::system_menus.filter(system_menus::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_menus::dsl::system_menus.filter(system_menus::id.eq(id)))
                .set(system_menus::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_menus::dsl::system_menus.filter(system_menus::id.eq(id)))
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete_transaction method
    pub async fn soft_delete_transaction(conn: &mut Connect, id: i32) -> Result<Self> {
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::soft_delete(conn, id).await?;
                    Ok(info)
                    // other action
                }
                .scope_boxed()
            })
            .await?;
        Ok(info)
    }
    // others methods
}

/// define Filter
#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    // other fields
    pub id: Option<i32>,
    pub ids: Option<Vec<i32>>,
    pub api_method: Option<String>,
    pub api_url: Option<String>,
}
/// define Forms Param
#[derive(Debug, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_menus)]
pub struct FormParamsForCreate {
    pub parent_id: i32,
    #[diesel(column_name = "type_")]
    pub r#type: i32,
    pub title: String,
    pub icon: String,
    pub router_name: String,
    pub router_component: String,
    pub router_path: String,
    pub redirect: String,
    pub link: String,
    pub iframe: String,
    pub btn_auth: String,
    pub api_url: String,
    pub api_method: String,
    pub is_hide: i32,
    pub is_keep_alive: i32,
    pub is_affix: i32,
    pub sort: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;

impl From<Entity> for FormParamsForCreate {
    fn from(value: Entity) -> Self {
        Self {
            parent_id: value.parent_id,
            r#type: value.r#type,
            title: value.title,
            icon: value.icon,
            router_name: value.router_name,
            router_component: value.router_component,
            router_path: value.router_path,
            redirect: value.redirect,
            link: value.link,
            iframe: value.iframe,
            btn_auth: value.btn_auth,
            api_url: value.api_url,
            api_method: value.api_method,
            is_hide: value.is_hide,
            is_keep_alive: value.is_keep_alive,
            is_affix: value.is_affix,
            sort: value.sort,
        }
    }
}
