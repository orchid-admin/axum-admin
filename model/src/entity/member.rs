use crate::{
    connect::DbConnect as Connect, entity::_pagination::Paginate, schema::members, Error, Result,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::{scoped_futures::*, AsyncConnection, RunQueryDsl, SaveChangesDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// define Entity
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = crate::schema::members)]
pub struct Entity {
    pub id: i32,
    pub unique_code: String,
    pub email: String,
    pub mobile: String,
    pub nickname: String,
    pub avatar: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub salt: String,
    pub sex: i32,
    pub balance: f64,
    integral: i32,
    pub remark: String,
    pub status: i32,
    pub is_promoter: i32,
    pub last_login_ip: String,
    pub last_login_time: Option<chrono::NaiveDateTime>,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}

/// impl Entity method
impl Entity {
    /// query find
    pub async fn find<F: Into<Filter>>(conn: &mut Connect, filter: F) -> Result<Option<Self>> {
        let filter: Filter = filter.into();
        let table = members::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(members::name.eq(_keyword));
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
        let table = members::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(members::name.eq(_keyword));
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
        let table = members::table;
        // filter condition
        if let Some(_keyword) = &filter.keyword {
            // let _ = table.filter(members::name.eq(_keyword));
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
        Ok(insert_into(members::dsl::members)
            .values(params)
            .get_results(conn)
            .await?)
    }
    /// create method
    pub async fn create(conn: &mut Connect, param: &FormParamsForCreate) -> Result<Self> {
        Ok(insert_into(members::dsl::members)
            .values(param)
            .get_result(conn)
            .await?)
    }
    /// update mthod
    pub async fn update(conn: &mut Connect, id: i32, params: FormParamsForUpdate) -> Result<Self> {
        Ok(update(members::dsl::members.filter(members::id.eq(id)))
            .set(params)
            .get_result(conn)
            .await?)
    }
    /// soft_delete method
    pub async fn soft_delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(update(members::dsl::members.filter(members::id.eq(id)))
            .set(members::deleted_at.eq(Some(SystemTime::now())))
            .get_result(conn)
            .await?)
    }
    /// delete method
    pub async fn delete(conn: &mut Connect, id: i32) -> Result<Self> {
        Ok(delete(members::dsl::members.filter(members::id.eq(id)))
            .get_result(conn)
            .await?)
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
    pub async fn set_last_login(
        conn: &mut Connect,
        entity: &mut Entity,
        login_ip: &str,
    ) -> Result<Self> {
        entity.last_login_ip = login_ip.to_owned();
        entity.last_login_time = Some(chrono::Local::now().naive_local());
        let result = entity.save_changes(conn).await?;
        Ok(result)
    }

    pub async fn get_by_unique_code(
        conn: &mut Connect,
        unique_code: &str,
        filter_id: Option<i32>,
    ) -> Result<Option<Self>> {
        let mut filter = Filter {
            unique_code: Some(unique_code.to_owned()),
            ..Default::default()
        };
        if let Some(id) = filter_id {
            filter.id_not = Some(id);
        }
        Self::find(conn, filter).await
    }

    pub async fn get_by_email(
        conn: &mut Connect,
        email: &str,
        filter_id: Option<i32>,
    ) -> Result<Option<Self>> {
        let mut filter = Filter {
            email: Some(email.to_owned()),
            ..Default::default()
        };
        if let Some(id) = filter_id {
            filter.id_not = Some(id);
        }
        Self::find(conn, filter).await
    }

    /// increment method
    pub async fn increment_transaction(
        conn: &mut Connect,
        id: i32,
        balance: Option<f64>,
        integral: Option<i32>,
    ) -> Result<Option<Self>> {
        use super::member_bill;
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::find(
                        conn,
                        Filter {
                            id: Some(id),
                            ..Default::default()
                        },
                    )
                    .await?;
                    if let Some(mut info) = info {
                        let mut form: FormParamsForUpdateBalanceAndIntegral = info.clone().into();
                        if let Some(balance) = balance {
                            form.balance += balance;
                            member_bill::Entity::create(
                                conn,
                                &member_bill::FormParamsForCreate {
                                    member_id: id,
                                    r#type: member_bill::BillType::Balance.into(),
                                    pm: member_bill::BillPm::Increment.into(),
                                    number: balance,
                                },
                            )
                            .await?;
                        }
                        if let Some(integral) = integral {
                            form.integral += integral;
                            member_bill::Entity::create(
                                conn,
                                &member_bill::FormParamsForCreate {
                                    member_id: id,
                                    r#type: member_bill::BillType::Integral.into(),
                                    pm: member_bill::BillPm::Increment.into(),
                                    number: integral as f64,
                                },
                            )
                            .await?;
                        }
                        info = update(members::dsl::members.filter(members::id.eq(id)))
                            .set(form)
                            .get_result(conn)
                            .await?;
                        return Ok(Some(info));
                    }
                    Ok(None)
                }
                .scope_boxed()
            })
            .await?;
        Ok(info)
    }

    /// decrement method
    pub async fn decrement_transaction(
        conn: &mut Connect,
        id: i32,
        balance: Option<f64>,
        integral: Option<i32>,
    ) -> Result<Option<Self>> {
        use super::member_bill;
        let info = conn
            .transaction::<_, Error, _>(|conn| {
                async move {
                    let info = Self::find(
                        conn,
                        Filter {
                            id: Some(id),
                            ..Default::default()
                        },
                    )
                    .await?;
                    if let Some(mut info) = info {
                        let mut form: FormParamsForUpdateBalanceAndIntegral = info.clone().into();
                        if let Some(balance) = balance {
                            form.balance -= balance;
                            member_bill::Entity::create(
                                conn,
                                &member_bill::FormParamsForCreate {
                                    member_id: id,
                                    r#type: member_bill::BillType::Balance.into(),
                                    pm: member_bill::BillPm::Decrement.into(),
                                    number: balance,
                                },
                            )
                            .await?;
                        }
                        if let Some(integral) = integral {
                            form.integral -= integral;
                            member_bill::Entity::create(
                                conn,
                                &member_bill::FormParamsForCreate {
                                    member_id: id,
                                    r#type: member_bill::BillType::Integral.into(),
                                    pm: member_bill::BillPm::Decrement.into(),
                                    number: integral as f64,
                                },
                            )
                            .await?;
                        }
                        info = update(members::dsl::members.filter(members::id.eq(id)))
                            .set(form)
                            .get_result(conn)
                            .await?;
                        return Ok(Some(info));
                    }
                    Ok(None)
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
    pub id_not: Option<i32>,
    pub unique_code: Option<String>,
    pub email: Option<String>,
    pub sex: Option<i32>,
    pub is_promoter: Option<i32>,
}
/// define Forms Param
#[derive(Debug, Default, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::members)]
pub struct FormParamsForCreate {
    pub unique_code: String,
    pub email: String,
    pub mobile: String,
    pub nickname: String,
    pub avatar: String,
    pub password: String,
    pub salt: String,
    pub sex: i32,
    pub balance: f64,
    pub integral: i32,
    pub remark: String,
    pub status: i32,
    pub is_promoter: i32,
}

pub type FormParamsForUpdate = FormParamsForCreate;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::members)]
pub struct FormParamsForUpdateBalanceAndIntegral {
    balance: f64,
    integral: i32,
}

impl From<Entity> for FormParamsForUpdateBalanceAndIntegral {
    fn from(value: Entity) -> Self {
        Self {
            balance: value.balance,
            integral: value.integral,
        }
    }
}
