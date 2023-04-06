mod prisma;

use prisma_client_rust::WhereQuery;
#[derive(Debug)]
struct User<'a> {
    username: &'a str,
}

impl<'a> WhereQuery<'a> for User<'a> {}
