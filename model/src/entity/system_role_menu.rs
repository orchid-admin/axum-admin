use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_role_menus, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl, SaveChangesDsl};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Getters)]
#[diesel(table_name = crate::schema::system_role_menus)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    role_id: i32,
    menu_id: i32,
    deleted_at: Option<SystemTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find(conn: &mut Connect, filter: &Filter) -> Result<Option<Self>> {
        let table = system_role_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_role_menus::name.eq(_keyword));
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
        let table = system_role_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_role_menus::name.eq(_keyword));
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
        let table = system_role_menus::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_role_menus::name.eq(_keyword));
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
        Ok(insert_into(system_role_menus::dsl::system_role_menus)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_role_menus::dsl::system_role_menus)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_role_menus::dsl::system_role_menus.filter(system_role_menus::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_role_menus::dsl::system_role_menus.filter(system_role_menus::id.eq(id)))
                .set(system_role_menus::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_role_menus::dsl::system_role_menus.filter(system_role_menus::id.eq(id)))
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
    pub async fn get_menu_ids_by_role_id(conn: &mut Connect, role_id: i32) -> Result<Vec<i32>> {
        Ok(Self::query(
            conn,
            &Filter {
                role_id: Some(role_id),
                ..Default::default()
            },
        )
        .await?
        .into_iter()
        .map(|x| x.menu_id)
        .collect::<Vec<i32>>())
    }
    pub async fn get_role_menus(
        conn: &mut Connect,
        role_id: i32,
    ) -> Result<Vec<super::system_menu::Entity>> {
        let menu_ids = Self::get_menu_ids_by_role_id(conn, role_id).await?;
        if menu_ids.len().eq(&0) {
            return Ok(vec![]);
        }
        Ok(super::system_menu::Entity::query(
            conn,
            &super::system_menu::Filter {
                ids: Some(menu_ids),
                ..Default::default()
            },
        )
        .await?)
    }

    pub async fn delete_by_role_id(conn: &mut Connect, role_id: i32) -> Result<Vec<Self>> {
        Ok(update(
            system_role_menus::dsl::system_role_menus
                .filter(system_role_menus::role_id.eq(role_id)),
        )
        .set(system_role_menus::deleted_at.eq(Some(SystemTime::now())))
        .get_results(conn)
        .await?)
    }

    pub async fn delete_by_role_id_menu_id(
        conn: &mut Connect,
        role_id: i32,
        menu_id: i32,
    ) -> Result<Vec<Self>> {
        Ok(update(
            system_role_menus::dsl::system_role_menus.filter(
                system_role_menus::role_id
                    .eq(role_id)
                    .and(system_role_menus::menu_id.eq(menu_id)),
            ),
        )
        .set(system_role_menus::deleted_at.eq(Some(SystemTime::now())))
        .get_results(conn)
        .await?)
    }
}
/// define Filter
#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    // other fields
    pub role_id: Option<i32>,
}
/// define Forms Param
#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_role_menus)]
pub struct FormParamsForCreate {
    pub role_id: i32,
    pub menu_id: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
