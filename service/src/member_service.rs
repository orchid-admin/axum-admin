use std::ops::{Add, Sub};

use crate::{
    member_bill_service,
    prisma::{member, SortOrder},
    Database, Result, ServiceError,
};
use prisma_client_rust::{bigdecimal::BigDecimal, or};
use serde::Serialize;
use utils::{
    datetime::{now_time, to_local_string},
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(
    db: &Database,
    unique_code: &str,
    email: &str,
    params: CreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .member()
        .create_unchecked(unique_code.to_owned(), email.to_owned(), params.to_params())
        .exec()
        .await?
        .into())
}

pub async fn update(db: &Database, id: i32, params: UpdateParams) -> Result<Info> {
    Ok(db
        .client
        .member()
        .update_unchecked(member::id::equals(id), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn delete(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .member()
        .update(
            member::id::equals(id),
            vec![member::deleted_at::set(Some(now_time()))],
        )
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .member()
        .find_unique(member::id::equals(id))
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}
pub async fn get_by_unique_code(
    db: &Database,
    unique_code: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut params = vec![
        member::unique_code::equals(unique_code.to_owned()),
        member::deleted_at::equals(None),
    ];
    if let Some(id) = filter_id {
        params.push(member::id::not(id));
    }
    Ok(db
        .client
        .member()
        .find_first(params)
        .exec()
        .await?
        .map(|x| x.into()))
}
pub async fn get_by_email(
    db: &Database,
    email: &str,
    filter_id: Option<i32>,
) -> Result<Option<Info>> {
    let mut params = vec![
        member::email::equals(email.to_owned()),
        member::deleted_at::equals(None),
    ];
    if let Some(id) = filter_id {
        params.push(member::id::not(id));
    }
    Ok(db
        .client
        .member()
        .find_first(params)
        .exec()
        .await?
        .map(|x| x.into()))
}
pub async fn all(db: &Database) -> Result<Vec<Info>> {
    Ok(db
        .client
        .member()
        .find_many(vec![member::deleted_at::equals(None)])
        .order_by(member::id::order(SortOrder::Asc))
        .exec()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Info>>())
}

pub async fn paginate(db: &Database, params: &SearchParams) -> Result<PaginateResult<Vec<Info>>> {
    let (data, total) = db
        .client
        ._batch((
            db.client
                .member()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(member::id::order(SortOrder::Desc)),
            db.client.member().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub async fn increment(
    db: &Database,
    user_id: i32,
    balance: Option<BigDecimal>,
    integral: Option<i32>,
) -> Result<Info> {
    let result = db
        .client
        ._transaction()
        .run::<ServiceError, _, _, _>(|_| async move {
            let user = info(db, user_id).await?;
            let mut params = UpdateParams {
                email: None,
                mobile: None,
                nickname: None,
                avatar: None,
                password: None,
                sex: None,
                balance: None,
                integral: None,
                remark: None,
                status: None,
                is_promoter: None,
            };
            if let Some(balance_num) = balance {
                params.balance = Some(user.balance.add(balance_num.clone()));
                member_bill_service::create(
                    db,
                    user_id,
                    member_bill_service::BillType::Balance,
                    member_bill_service::CreateParams {
                        pm: Some(member_bill_service::BillPm::Increment.into()),
                        number: Some(balance_num),
                    },
                )
                .await?;
            }
            if let Some(integral_num) = integral {
                params.integral = Some(user.integral + integral_num);
                member_bill_service::create(
                    db,
                    user_id,
                    member_bill_service::BillType::Integral,
                    member_bill_service::CreateParams {
                        pm: Some(member_bill_service::BillPm::Increment.into()),
                        number: Some(integral_num.try_into().unwrap()),
                    },
                )
                .await?;
            }
            let result = update(db, user_id, params).await?;
            Ok(result)
        })
        .await?;
    Ok(result)
}

pub async fn decrement(
    db: &Database,
    user_id: i32,
    balance: Option<BigDecimal>,
    integral: Option<i32>,
) -> Result<Info> {
    let result = db
        .client
        ._transaction()
        .run::<ServiceError, _, _, _>(|_| async move {
            let user = info(db, user_id).await?;
            let mut params = UpdateParams {
                email: None,
                mobile: None,
                nickname: None,
                avatar: None,
                password: None,
                sex: None,
                balance: None,
                integral: None,
                remark: None,
                status: None,
                is_promoter: None,
            };
            if let Some(balance_num) = balance {
                params.balance = Some(user.balance.sub(balance_num.clone()));
                member_bill_service::create(
                    db,
                    user_id,
                    member_bill_service::BillType::Integral,
                    member_bill_service::CreateParams {
                        pm: Some(member_bill_service::BillPm::Decrement.into()),
                        number: Some(balance_num),
                    },
                )
                .await?;
            }
            if let Some(integral_num) = integral {
                params.integral = Some(user.integral - integral_num);
                member_bill_service::create(
                    db,
                    user_id,
                    member_bill_service::BillType::Integral,
                    member_bill_service::CreateParams {
                        pm: Some(member_bill_service::BillPm::Decrement.into()),
                        number: Some(integral_num.try_into().unwrap()),
                    },
                )
                .await?;
            }
            let result = update(db, user_id, params).await?;
            Ok(result)
        })
        .await?;
    Ok(result)
}

#[async_recursion::async_recursion]
pub async fn generate_code(db: &Database, code_length: usize) -> Result<String> {
    let unique_code: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(code_length)
        .collect();
    match get_by_unique_code(db, &unique_code, None).await? {
        Some(_) => generate_code(db, code_length).await,
        None => Ok(unique_code),
    }
}
pub struct SearchParams {
    keyword: Option<String>,
    sex: Option<i32>,
    status: Option<i32>,
    is_promoter: Option<i32>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<member::WhereParam> {
        let mut params = vec![member::deleted_at::equals(None)];
        if let Some(keyword) = &self.keyword {
            params.push(or!(
                member::unique_code::contains(keyword.to_string()),
                member::email::contains(keyword.to_string()),
                member::mobile::contains(keyword.to_string()),
                member::nickname::contains(keyword.to_string()),
                member::remark::contains(keyword.to_string()),
            ));
        }
        if let Some(status) = self.status {
            params.push(member::status::equals(status));
        }
        if let Some(sex) = self.sex {
            params.push(member::sex::equals(sex));
        }
        if let Some(is_promoter) = self.is_promoter {
            params.push(member::is_promoter::equals(is_promoter));
        }
        params
    }

    pub fn new(
        keyword: Option<String>,
        sex: Option<i32>,
        status: Option<i32>,
        is_promoter: Option<i32>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            keyword,
            sex,
            status,
            is_promoter,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    unique_code: String,
    email: String,
    mobile: String,
    nickname: String,
    avatar: String,
    sex: i32,
    balance: BigDecimal,
    integral: i32,
    remark: String,
    status: i32,
    is_promoter: i32,
    last_login_ip: String,
    last_login_time: Option<String>,
    created_at: String,
}

impl From<member::Data> for Info {
    fn from(value: member::Data) -> Self {
        Self {
            id: value.id,
            unique_code: value.unique_code,
            email: value.email,
            mobile: value.mobile,
            nickname: value.nickname,
            avatar: value.avatar,
            sex: value.sex,
            balance: value.balance,
            integral: value.integral,
            remark: value.remark,
            status: value.status,
            is_promoter: value.is_promoter,
            last_login_ip: value.last_login_ip,
            last_login_time: value.last_login_time.map(to_local_string),
            created_at: to_local_string(value.created_at),
        }
    }
}
member::partial_unchecked!(CreateParams {
    mobile
    nickname
    avatar
    password
    sex
    balance
    integral
    remark
    status
    is_promoter
});
member::partial_unchecked!(UpdateParams {
    email
    mobile
    nickname
    avatar
    password
    sex
    balance
    integral
    remark
    status
    is_promoter
});
