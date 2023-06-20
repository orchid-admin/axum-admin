use crate::{
    member_service,
    prisma::{member, member_bill, SortOrder},
    Database, Result, ServiceError,
};
use prisma_client_rust::{or, prisma_models::parse_datetime};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use utils::{
    datetime::to_local_string,
    paginate::{PaginateParams, PaginateResult},
};

pub async fn create(
    db: &Database,
    user_id: i32,
    r#type: BillType,
    params: CreateParams,
) -> Result<Info> {
    Ok(db
        .client
        .member_bill()
        .create_unchecked(user_id, r#type.into(), params.to_params())
        .exec()
        .await?
        .into())
}
pub async fn info(db: &Database, id: i32) -> Result<Info> {
    Ok(db
        .client
        .member_bill()
        .find_unique(member_bill::id::equals(id))
        .exec()
        .await?
        .ok_or(ServiceError::DataNotFound)?
        .into())
}
pub async fn paginate(db: &Database, params: &SearchParams) -> Result<PaginateResult<Vec<Info>>> {
    let (data, total) = db
        .client
        ._batch((
            db.client
                .member_bill()
                .find_many(params.to_params())
                .skip(params.paginate.get_skip())
                .take(params.paginate.get_limit())
                .order_by(member_bill::id::order(SortOrder::Desc)),
            db.client.member_bill().count(params.to_params()),
        ))
        .await?;
    Ok(PaginateResult {
        total,
        data: data.into_iter().map(|x| x.into()).collect::<Vec<Info>>(),
    })
}

pub struct SearchParams {
    r#type: Option<BillType>,
    pm: Option<BillPm>,
    keyword: Option<String>,
    date: Option<String>,
    paginate: PaginateParams,
}
impl SearchParams {
    fn to_params(&self) -> Vec<member_bill::WhereParam> {
        let mut params = vec![];
        if let Some(t) = &self.r#type {
            params.push(member_bill::r#type::equals(t.clone().into()));
        }
        if let Some(pm) = &self.pm {
            params.push(member_bill::pm::equals(pm.clone().into()));
        }
        if let Some(keyword) = &self.keyword {
            let user_search = vec![or!(
                member::unique_code::contains(keyword.to_string()),
                member::email::contains(keyword.to_string()),
                member::mobile::contains(keyword.to_string()),
                member::nickname::contains(keyword.to_string()),
            )];
            params.push(or!(member_bill::user::is(user_search),));
        }
        if let Some(date) = &self.date {
            params.push(member_bill::created_at::equals(
                parse_datetime(date).unwrap(),
            ));
        }
        params
    }

    pub fn new(
        r#type: Option<BillType>,
        pm: Option<BillPm>,
        keyword: Option<String>,
        date: Option<String>,
        paginate: PaginateParams,
    ) -> Self {
        Self {
            r#type,
            pm,
            keyword,
            date,
            paginate,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Info {
    id: i32,
    user_id: i32,
    user: Option<member_service::Info>,
    r#type: BillType,
    pm: BillPm,
    number: f64,
    created_at: String,
}

impl From<member_bill::Data> for Info {
    fn from(value: member_bill::Data) -> Self {
        Self {
            id: value.id,
            user_id: value.uid,
            user: match value.user() {
                Ok(x) => Some(x.clone().into()),
                Err(_) => None,
            },
            r#type: value.r#type.into(),
            pm: value.pm.into(),
            number: value.number,
            created_at: to_local_string(value.created_at),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
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
member_bill::partial_unchecked!(CreateParams {
    pm
    number
});
