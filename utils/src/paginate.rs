#[serde_with::serde_as]
#[derive(Debug, serde::Deserialize)]
pub struct PaginateParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    page: i64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    limit: i64,
}

impl PaginateParams {
    pub fn get_page(&self) -> i64 {
        match self.page.ge(&0) {
            true => self.page,
            false => self.page,
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
