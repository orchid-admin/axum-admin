use crate::connect::DbConnect as Connect;
use diesel::{
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment, QueryId},
    sql_types::BigInt,
    QueryResult,
};
use diesel_async::{methods::LoadQuery, RunQueryDsl};

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }
}
impl<T: Query> Paginated<T> {
    pub fn load_and_count_pages<'a, U>(
        self,
        conn: &'a mut Connect,
    ) -> impl std::future::Future<Output = QueryResult<(Vec<U>, i64)>> + Send + 'a
    where
        Self: LoadQuery<'a, Connect, (U, i64)> + 'a,
        U: std::marker::Send + 'a,
        T: 'a,
    {
        #![allow(clippy::get_first)]
        let results = self.load::<(U, i64)>(conn);
        async move {
            let results = results.await?;
            let total = results.get(0).map(|x| x.1).unwrap_or(0);
            let records = results.into_iter().map(|x| x.0).collect();
            Ok((records, total))
        }
    }
}

// impl<T: Query> Paginated<T> {
//     pub fn load_and_count_total<'a, U>(
//         self,
//         conn: &'a mut AsyncPgConnection,
//     ) -> impl std::future::Future<Output = QueryResult<(Vec<U>, i64)>> + Send + 'a
//     where
//         Self: LoadQuery<'a, AsyncPgConnection, (U, i64)>,
//         U: Send + 'a,
//         T: 'a,
//     {
//         // Ignore those linting errors. `get(0)` cannot be replaced with `first()`.
//         #![allow(clippy::get_first)]

//         let results = self.load::<(U, i64)>(conn);

//         async move {
//             let results = results.await?;
//             let total = results.get(0).map(|x| x.1).unwrap_or(0);
//             let records = results.into_iter().map(|x| x.0).collect();
//             Ok((records, total))
//         }
//     }
// }
impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") as paged_query_with LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
