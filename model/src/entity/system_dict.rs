use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_dicts, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, query_builder::BoxedSelectStatement, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::system_dicts)]
pub struct Entity {
    id: i32,
    name: String,
    sign: String,
    remark: String,
    status: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Option<Self>> {
        let filter: Filter = filter.into();
        let mut table = system_dicts::table.into_boxed();
        if let Some(sign) = &filter.sign {
            table = table.filter(system_dicts::name.eq(sign));
        }
        if let Some(id) = &filter.id_ne {
            table = table.filter(system_dicts::id.ne(id));
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
        let table = system_dicts::table.into_boxed();
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_dicts::name.eq(_keyword));
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
        let mut table = system_dicts::table.into_boxed();
        // filter condition
        // if let Some(sign) = &filter.sign {
        //     table = table.filter(system_dicts::name.eq(sign));
        // }
        // if let Some(id) = &filter.ne_id {
        //     table = table.filter(system_dicts::id.ne(id));
        // }
        // if filter.is_deleted {
        //     table = table.filter(system_dicts::deleted_at.is_not_null());
        // }

        let (data, count) = table
            .select(Entity::as_select())
            .paginate(page)
            // .per_page(limit)
            .load_and_count_pages::<Entity>(conn)
            .await?;
        Ok((data, count))
    }
    /// insert method
    pub async fn insert(conn: &mut Connect, params: Vec<FormParamsForCreate>) -> Result<Vec<Self>> {
        Ok(insert_into(system_dicts::dsl::system_dicts)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_dicts::dsl::system_dicts)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_dicts::dsl::system_dicts.filter(system_dicts::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_dicts::dsl::system_dicts.filter(system_dicts::id.eq(id)))
                .set(system_dicts::deleted_at.eq(Some(chrono::Local::now().naive_local())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_dicts::dsl::system_dicts.filter(system_dicts::id.eq(id)))
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
    pub sign: Option<String>,
    pub id_ne: Option<i32>,
    // pub is_deleted: bool,
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_dicts)]
pub struct FormParamsForCreate {
    pub name: String,
    pub sign: String,
    pub remark: String,
    pub status: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
