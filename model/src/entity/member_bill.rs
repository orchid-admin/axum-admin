use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::member_bills, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Getters)]
#[diesel(table_name = crate::schema::member_bills)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    member_id: i32,
    #[diesel(column_name = "type_")]
    r#type: i32,
    pm: i32,
    number: f64,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted_at: Option<SystemTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find(conn: &mut Connect, filter: &Filter) -> Result<Option<Self>> {
        let table = member_bills::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(member_bills::name.eq(_keyword));
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
        let table = member_bills::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(member_bills::name.eq(_keyword));
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
        let table = member_bills::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(member_bills::name.eq(_keyword));
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
        Ok(insert_into(member_bills::dsl::member_bills)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(member_bills::dsl::member_bills)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(member_bills::dsl::member_bills.filter(member_bills::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(member_bills::dsl::member_bills.filter(member_bills::id.eq(id)))
                .set(member_bills::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(member_bills::dsl::member_bills.filter(member_bills::id.eq(id)))
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
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::member_bills)]
pub struct FormParamsForCreate {
    pub member_id: i32,
    #[diesel(column_name = "type_")]
    pub r#type: i32,
    pub pm: i32,
    pub number: f64,
}

pub type FormParamsForUpdate = FormParamsForCreate;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum BillType {
    Balance = 1,
    Integral = 2,
}
impl From<i32> for BillType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Balance,
            2 => Self::Integral,
            _ => Self::Balance,
        }
    }
}
impl From<BillType> for i32 {
    fn from(value: BillType) -> Self {
        match value {
            BillType::Balance => 1,
            BillType::Integral => 2,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum BillPm {
    Increment = 1,
    Decrement = 0,
}
impl From<i32> for BillPm {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Increment,
            0 => Self::Decrement,
            _ => Self::Increment,
        }
    }
}
impl From<BillPm> for i32 {
    fn from(value: BillPm) -> Self {
        match value {
            BillPm::Increment => 1,
            BillPm::Decrement => 0,
        }
    }
}
