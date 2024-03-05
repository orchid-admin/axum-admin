use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_menus, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Getters, Setters,
)]
#[diesel(table_name = crate::schema::system_menus)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub", set = "pub")]
    parent_id: i32,
    #[getset(get = "pub")]
    #[diesel(column_name = "type_")]
    r#type: i32,
    #[getset(get = "pub")]
    title: String,
    #[getset(get = "pub")]
    icon: String,
    #[getset(get = "pub")]
    router_name: String,
    #[getset(get = "pub")]
    router_component: String,
    #[getset(get = "pub")]
    router_path: String,
    #[getset(get = "pub")]
    redirect: String,
    #[getset(get = "pub")]
    link: String,
    #[getset(get = "pub")]
    iframe: String,
    #[getset(get = "pub")]
    btn_auth: String,
    #[getset(get = "pub")]
    api_url: String,
    #[getset(get = "pub")]
    api_method: String,
    #[getset(get = "pub")]
    is_hide: i32,
    #[getset(get = "pub")]
    is_keep_alive: i32,
    #[getset(get = "pub")]
    is_affix: i32,
    /// 排序
    sort: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
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
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_menus)]
pub struct FormParamsForCreate {
    parent_id: i32,
    #[diesel(column_name = "type_")]
    r#type: i32,
    title: String,
    icon: String,
    router_name: String,
    router_component: String,
    router_path: String,
    redirect: String,
    link: String,
    iframe: String,
    btn_auth: String,
    api_url: String,
    api_method: String,
    is_hide: i32,
    is_keep_alive: i32,
    is_affix: i32,
    sort: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
