use crate::{schema::system_users, Connect, Result};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;
use getset::Getters;
use serde::Serialize;

#[derive(Debug, Serialize, Getters, Queryable, Selectable)]
#[diesel(table_name = crate::schema::system_users)]
pub struct Entity {
    #[getset(get = "pub")]
    id: i32,
    #[getset(get = "pub")]
    username: String,
    nickname: String,
    role_id: Option<i32>,
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
    // expire_time: Option<String>,
    status: i32,
    // created_at: String,
    last_login_ip: String,
    // last_login_time: Option<String>,
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

    pub async fn insert(conn: &mut Connect, params: &CreateForm) -> Result<usize> {
        Ok(insert_into(system_users::dsl::system_users)
            .values(params)
            .execute(conn)
            .await?)
    }

    pub async fn update(conn: &mut Connect, id: i32, params: CreateForm) -> Result<usize> {
        let result = update(system_users::dsl::system_users.filter(system_users::id.eq(id)))
            .set(params)
            .execute(conn)
            .await?;
        Ok(result)
    }

    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<usize> {
        // let result = delete(system_users::dsl::system_users.filter(system_users::id.eq(id)))
        //     .execute(conn)
        //     .await?;
        let result = update(system_users::dsl::system_users.filter(system_users::id.eq(id)))
            .set(system_users::deleted_at.eq(Some(std::time::SystemTime::now())))
            .execute(conn)
            .await?;
        Ok(result)
    }

    pub async fn delete(conn: &mut Connect, id: i32) -> Result<usize> {
        let result = delete(system_users::dsl::system_users.filter(system_users::id.eq(id)))
            .execute(conn)
            .await?;
        Ok(result)
    }

    pub async fn batch_set_dept(
        conn: &mut Connect,
        dept_id: Option<i32>,
        user_ids: Vec<i32>,
    ) -> Result<usize> {
        let result =
            update(system_users::dsl::system_users.filter(system_users::id.eq_any(user_ids)))
                .set(system_users::dept_id.eq(dept_id))
                .execute(conn)
                .await?;
        Ok(result)
    }
}

#[derive(Debug, Default)]
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
pub struct CreateForm {
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
    // expire_time: Option<String>,
    pub status: i32,
}
