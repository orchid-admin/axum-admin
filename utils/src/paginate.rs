#[serde_with::serde_as]
#[derive(Debug, serde::Deserialize)]
pub struct PaginateParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    page: i64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    limit: i64,
}

impl PaginateParams {
    pub fn get_skip(&self) -> i64 {
        match self.page > 0 {
            true => (self.page - 1) * self.limit,
            false => self.limit,
        }
    }

    pub fn get_limit(&self) -> i64 {
        self.limit
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PaginateResult<T: serde::Serialize> {
    pub total: i64,
    pub data: T,
}
