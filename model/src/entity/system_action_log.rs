use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_action_logs,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::system_action_logs)]
pub struct Entity {
    id: i32,
    user_id: i32,
    menu_id: i32,
    menu_names: String,
    ip_address: String,
    ip_address_name: String,
    browser_agent: String,
    created_at: SystemTime,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Option<Self>> {
        let filter: Filter = filter.into();
        let table = system_action_logs::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_action_logs::name.eq(_keyword));
        }

        let info = table
            .select(Entity::as_select())
            .first::<Entity>(conn)
            .await
            .optional()?;
        Ok(info)
    }
    /// query method
    pub async fn query<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Vec<Self>> {
        let filter: Filter = filter.into();
        let table = system_action_logs::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_action_logs::name.eq(_keyword));
        }

        let infos = table
            .select(Entity::as_select())
            .load::<Entity>(conn)
            .await?;
        Ok(infos)
    }
    /// paginate method
    pub async fn paginate<F: Into<Filter>>(
        conn: &mut Connect,
        page: i64,
        limit: i64,
        filter: F,
    ) -> Result<(Vec<Self>, i64)> {
        let filter: Filter = filter.into();
        let table = system_action_logs::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_action_logs::name.eq(_keyword));
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
        Ok(insert_into(system_action_logs::dsl::system_action_logs)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_action_logs::dsl::system_action_logs)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(update(
            system_action_logs::dsl::system_action_logs.filter(system_action_logs::id.eq(id)),
        )
        .set(params)
        .get_result(conn)
        .await?)
    }

    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(delete(
            system_action_logs::dsl::system_action_logs.filter(system_action_logs::id.eq(id)),
        )
        .get_result(conn)
        .await?)
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
    pub user_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub date: Option<String>,
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_action_logs)]
pub struct FormParamsForCreate {
    pub user_id: i32,
    pub menu_id: i32,
    pub menu_names: String,
    pub ip_address: String,
    pub ip_address_name: String,
    pub browser_agent: String,
}

pub type FormParamsForUpdate = FormParamsForCreate;
