use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_roles, Error,
    Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::system_roles)]
pub struct Entity {
    pub id: i32,
    name: String,
    sign: String,
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
        let table = system_roles::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_roles::name.eq(filter.keyword));
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
        let table = system_roles::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_roles::name.eq(filter.keyword));
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
        let table = system_roles::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(system_roles::name.eq(filter.keyword));
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
        Ok(insert_into(system_roles::dsl::system_roles)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_roles::dsl::system_roles)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_roles::dsl::system_roles.filter(system_roles::id.eq(id)))
                .set(params)
                .get_result(conn)
                .await?,
        )
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_roles::dsl::system_roles.filter(system_roles::id.eq(id)))
                .set(system_roles::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_roles::dsl::system_roles.filter(system_roles::id.eq(id)))
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

                    // other action
                    super::system_role_menu::Entity::delete_by_role_id(conn, id).await?;
                    Ok(info)
                }
                .scope_boxed()
            })
            .await?;
        Ok(info)
    }
    // others methods
    pub async fn create_with_menus(
        conn: &mut Connect,
        param: &FormParamsForCreate,
        menu_ids: Vec<i32>,
    ) -> Result<Self> {
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::create(conn, param).await?;

                    // other action
                    if !menu_ids.is_empty() {
                        let role_menu_ids = menu_ids
                            .into_iter()
                            .map(|x| super::system_role_menu::FormParamsForCreate {
                                role_id: info.id,
                                menu_id: x,
                            })
                            .collect::<Vec<super::system_role_menu::FormParamsForCreate>>();
                        super::system_role_menu::Entity::insert(conn, role_menu_ids).await?;
                    }

                    Ok(info)
                }
                .scope_boxed()
            })
            .await?;
        Ok(info)
    }

    pub async fn update_with_menus(
        conn: &mut Connect,
        id: i32,
        param: FormParamsForUpdate,
        menu_ids: Vec<i32>,
    ) -> Result<Self> {
        use super::system_role_menu;
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::update(conn, id, param).await?;

                    // other action
                    let current_menus =
                        system_role_menu::Entity::get_menu_ids_by_role_id(conn, id).await?;
                    if !menu_ids.is_empty() {
                        let wait_creates = menu_ids
                            .clone()
                            .into_iter()
                            .filter(|x| match current_menus.is_empty() {
                                false => !current_menus.contains(x),
                                true => true,
                            })
                            .map(|x| system_role_menu::FormParamsForCreate {
                                role_id: id,
                                menu_id: x,
                            })
                            .collect::<Vec<system_role_menu::FormParamsForCreate>>();
                        let wait_deletes = match current_menus.is_empty() {
                            false => current_menus
                                .clone()
                                .into_iter()
                                .filter(|x| !menu_ids.contains(x))
                                .map(|x| (id, x))
                                .collect::<Vec<(i32, i32)>>(),
                            true => vec![],
                        };

                        if !wait_deletes.is_empty() {
                            for wait_delete in wait_deletes {
                                system_role_menu::Entity::delete_by_role_id_menu_id(
                                    conn,
                                    wait_delete.0,
                                    wait_delete.1,
                                )
                                .await?;
                            }
                        }

                        if !wait_creates.is_empty() {
                            system_role_menu::Entity::insert(conn, wait_creates).await?;
                        }
                    } else if !current_menus.is_empty() {
                        system_role_menu::Entity::delete_by_role_id(conn, id).await?;
                    }

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
