use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::system_users, Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{RunQueryDsl, SaveChangesDsl};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset, Serialize, Getters)]
#[diesel(table_name = crate::schema::system_users)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub")]
    username: String,
    nickname: String,
    #[getset(get = "pub")]
    role_id: Option<i32>,
    #[getset(get = "pub")]
    dept_id: Option<i32>,
    phone: String,
    email: String,
    sex: i32,
    #[serde(skip)]
    #[getset(get = "pub")]
    password: String,
    #[serde(skip)]
    #[getset(get = "pub")]
    salt: String,
    describe: String,
    expire_time: Option<SystemTime>,
    status: i32,
    last_login_ip: String,
    last_login_time: Option<SystemTime>,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted_at: Option<SystemTime>,
}
impl Entity {
    pub async fn find(conn: &mut Connect, filter: &Filter) -> Result<Option<Self>> {
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

    pub async fn query(conn: &mut Connect, filter: &Filter) -> Result<Vec<Self>> {
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

    pub async fn paginate(
        conn: &mut Connect,
        page: i64,
        limit: i64,
        filter: &Filter,
    ) -> Result<(Vec<Self>, i64)> {
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

    pub async fn insert(conn: &mut Connect, params: Vec<FormParamsForCreate>) -> Result<Vec<Self>> {
        Ok(insert_into(system_users::dsl::system_users)
            .values(params)
            .get_results(conn)
            .await?)
    }

    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(system_users::dsl::system_users)
            .values(param)
            .get_result(conn)
            .await?)
    }

    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForCreate) -> Result<Self> {
        Ok(
            update(system_users::dsl::system_users.filter(system_users::id.eq(id)))
                .set(params)
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
}

#[derive(Debug, Default, Deserialize)]
pub struct Filter {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
}

#[derive(Debug, Insertable, AsChangeset)]
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
    expire_time: Option<SystemTime>,
    pub status: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;
