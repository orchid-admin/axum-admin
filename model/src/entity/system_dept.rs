use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_depts, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Getters)]
#[diesel(table_name = crate::schema::system_depts)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub")]
    parent_id: i32,
    name: String,
    person_name: String,
    person_phone: String,
    person_email: String,
    describe: String,
    status: i32,
    sort: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted_at: Option<SystemTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find(conn: &mut Connect, filter: &Filter) -> Result<Option<Self>> {
        let table = system_depts::table;
        // filter condition
        if let Some(id) = filter.id {
            let _ = table.filter(system_depts::id.eq(id));
        }
        if let Some(name) = &filter.name {
            let _ = table.filter(system_depts::name.like(name));
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
        let table = system_depts::table;
        // filter condition
        if let Some(id) = filter.id {
            let _ = table.filter(system_depts::id.eq(id));
        }
        if let Some(name) = &filter.name {
            let _ = table.filter(system_depts::name.like(name));
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
        let table = system_depts::table;
        // filter condition
        if let Some(id) = filter.id {
            let _ = table.filter(system_depts::id.eq(id));
        }
        if let Some(name) = &filter.name {
            let _ = table.filter(system_depts::name.like(name));
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
        Ok(insert_into(system_depts::dsl::system_depts)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_depts::dsl::system_depts)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_depts::dsl::system_depts.filter(system_depts::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_depts::dsl::system_depts.filter(system_depts::id.eq(id)))
                .set(system_depts::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_depts::dsl::system_depts.filter(system_depts::id.eq(id)))
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete_transaction method
    pub async fn soft_delete_transaction(conn: &mut Connect, id: i32) -> Result<Self> {
        use super::system_user;
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::soft_delete(conn, id).await?;
                    let user_ids = system_user::Entity::query(
                        conn,
                        &system_user::Filter {
                            dept_id: Some(id),
                            ..Default::default()
                        },
                    )
                    .await?
                    .into_iter()
                    .map(|x| *x.id())
                    .collect::<Vec<i32>>();
                    system_user::Entity::batch_set_dept(conn, None, user_ids).await?;
                    Ok(info)
                }
                .scope_boxed()
            })
            .await?;
        Ok(info)
    }
}
/// define Filter
#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    // other fields
    pub id: Option<i32>,
    pub name: Option<String>,
    pub person_name: Option<String>,
    pub person_phone: Option<String>,
    pub person_email: Option<i32>,
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_depts)]
pub struct FormParamsForCreate {
    parent_id: i32,
    name: String,
    person_name: String,
    person_phone: String,
    person_email: String,
    describe: String,
    status: i32,
    sort: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
