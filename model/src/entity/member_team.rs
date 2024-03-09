use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::member_teams, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::member_teams)]
pub struct Entity {
    id: i32,
    owner_uid: i32,
    parent_uid: i32,
    member_id: i32,
    level: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Option<Self>> {
        let filter: Filter = filter.into();
        let table = member_teams::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(member_teams::name.eq(_keyword));
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
        let table = member_teams::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(member_teams::name.eq(_keyword));
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
        let table = member_teams::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(member_teams::name.eq(_keyword));
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
        Ok(insert_into(member_teams::dsl::member_teams)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(member_teams::dsl::member_teams)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(member_teams::dsl::member_teams.filter(member_teams::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(member_teams::dsl::member_teams.filter(member_teams::id.eq(id)))
                .set(member_teams::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(member_teams::dsl::member_teams.filter(member_teams::id.eq(id)))
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
    pub date: Option<String>,
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::member_teams)]
pub struct FormParamsForCreate {
    owner_uid: i32,
    parent_uid: i32,
    member_id: i32,
    level: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
