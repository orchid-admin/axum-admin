use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_users, Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{RunQueryDsl, SaveChangesDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::system_users)]
pub struct Entity {
    pub id: i32,
    pub username: String,
    nickname: String,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    phone: String,
    email: String,
    sex: i32,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub salt: String,
    describe: String,
    expire_time: Option<SystemTime>,
    status: i32,
    last_login_ip: String,
    last_login_time: Option<SystemTime>,
    created_at: SystemTime,
    updated_at: Option<SystemTime>,
    deleted_at: Option<SystemTime>,
}
impl Entity {
    pub async fn find<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Option<Self>> {
        let filter = filter.into();
        let table = system_users::table;

        if let Some(id) = filter.id {
            let _ = table.filter(system_users::id.eq(id));
        }
        if let Some(username) = &filter.username {
            let _ = table.filter(system_users::username.eq(username));
        }
        if let Some(phone) = &filter.phone {
            let _ = table.filter(system_users::phone.eq(phone));
        }
        let info = table
            .select(Entity::as_select())
            .first::<Entity>(conn)
            .await
            .optional()?;
        Ok(info)
    }

    pub async fn query<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Vec<Self>> {
        let filter: Filter = filter.into();
        let table = system_users::table;

        if let Some(id) = filter.id {
            let _ = table.filter(system_users::id.eq(id));
        }
        if let Some(username) = &filter.username {
            let _ = table.filter(system_users::username.eq(username));
        }
        if let Some(phone) = &filter.phone {
            let _ = table.filter(system_users::phone.eq(phone));
        }
        let infos = table
            .select(Entity::as_select())
            .load::<Entity>(conn)
            .await?;
        Ok(infos)
    }

    pub async fn paginate<F: Into<Filter>>(
        conn: &mut Connect,
        page: i64,
        limit: i64,
        filter: F,
    ) -> Result<(Vec<Self>, i64)> {
        let filter: Filter = filter.into();
        let table = system_users::table;

        if let Some(id) = filter.id {
            let _ = table.filter(system_users::id.eq(id));
        }
        if let Some(username) = &filter.username {
            let _ = table.filter(system_users::username.eq(username));
        }
        if let Some(phone) = &filter.phone {
            let _ = table.filter(system_users::phone.eq(phone));
        }
        Ok(table
            .select(Entity::as_select())
            .paginate(page)
            .per_page(limit)
            .load_and_count_pages::<Entity>(conn)
            .await?)
    }

    pub async fn insert<C: Into<FormParamsForCreate>>(
        conn: &mut Connect,
        params: Vec<C>,
    ) -> Result<Vec<Self>> {
        Ok(insert_into(system_users::dsl::system_users)
            .values(
                params
                    .into_iter()
                    .map(|param| param.into())
                    .collect::<Vec<FormParamsForCreate>>(),
            )
            .get_results(conn)
            .await?)
    }

    pub async fn create<C: Into<FormParamsForCreate>>(
        conn: &mut Connect,
        param: C,
    ) -> Result<Self> {
        Ok(insert_into(system_users::dsl::system_users)
            .values(param.into())
            .get_result(conn)
            .await?)
    }

    pub async fn update<U: Into<FormParamsForCreate>>(
        conn: &mut Connect,
        id: i32,
        param: U,
    ) -> Result<Self> {
        Ok(
            update(system_users::dsl::system_users.filter(system_users::id.eq(id)))
                .set(param.into())
                .get_result(conn)
                .await?,
        )
    }

    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            update(system_users::dsl::system_users.filter(system_users::id.eq(id)))
                .set(system_users::deleted_at.eq(Some(SystemTime::now())))
                .get_result(conn)
                .await?,
        )
    }

    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(
            delete(system_users::dsl::system_users.filter(system_users::id.eq(id)))
                .get_result(conn)
                .await?,
        )
    }

    pub async fn set_last_login(
        conn: &mut Connect,
        entity: &mut Entity,
        login_ip: &str,
    ) -> Result<Self> {
        entity.last_login_ip = login_ip.to_owned();
        entity.last_login_time = Some(SystemTime::now());
        let result = entity.save_changes(conn).await?;
        Ok(result)
    }
    pub async fn batch_set_dept(
        conn: &mut Connect,
        dept_id: Option<i32>,
        user_ids: Vec<i32>,
    ) -> Result<Self> {
        let result =
            update(system_users::dsl::system_users.filter(system_users::id.eq_any(user_ids)))
                .set(system_users::dept_id.eq(dept_id))
                .get_result(conn)
                .await?;
        Ok(result)
    }

    pub async fn update_password(conn: &mut Connect, id: i32, password: &str) -> Result<Self> {
        Ok(
            update(system_users::dsl::system_users.filter(system_users::id.eq(id)))
                .set(system_users::password.eq(password))
                .get_result(conn)
                .await?,
        )
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub keyword: Option<String>,
    pub status: Option<i32>,
    pub id: Option<i32>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
}

#[derive(Debug, Default, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::system_users)]
pub struct FormParamsForCreate {
    pub username: String,
    pub nickname: String,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub phone: String,
    pub email: String,
    pub sex: i32,
    pub password: String,
    pub salt: String,
    pub describe: String,
    pub expire_time: Option<SystemTime>,
    pub status: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
