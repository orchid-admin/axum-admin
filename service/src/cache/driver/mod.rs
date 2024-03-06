use crate::{cache::Info, Result};
use serde::Serialize;

mod database;
mod memory;

#[async_trait::async_trait]
pub trait Driver {
    /// Storing Items In The Cache
    async fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: i32,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info>;

    /// Retrieving Items From The Cache
    async fn first(&self, r#type: i32, key: &str, default: Option<Info>) -> Result<Option<Info>>;

    /// Retrieve & Delete
    async fn pull(&mut self, r#type: i32, key: &str) -> Result<Info>;

    /// clear the entire cache
    async fn flush(&mut self, r#type: Option<i32>) -> Result<i64>;
}
