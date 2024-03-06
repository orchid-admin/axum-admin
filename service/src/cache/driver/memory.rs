use super::Driver;
use crate::{cache::Info, Result, ServiceError};
use serde::Serialize;
use utils::datetime::now_timestamp;

#[derive(Default)]
pub struct CacheDriverMemory {
    data: Vec<Info>,
}

#[async_trait::async_trait]
impl Driver for CacheDriverMemory {
    async fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: i32,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info> {
        let info = Info {
            r#type,
            key: key.to_owned(),
            value: serde_json::to_string(&value).unwrap(),
            valid_time_length,
            attach,
            create_time: now_timestamp(None),
        };
        self.data.push(info.clone());
        Ok(info)
    }

    async fn first(&self, r#type: i32, key: &str, default: Option<Info>) -> Result<Option<Info>> {
        let info = self
            .data
            .clone()
            .into_iter()
            .find(|x| x.r#type.eq(&r#type) && x.key.eq(key));
        if info.is_none() {
            return Ok(default);
        }
        Ok(info)
    }
    async fn pull(&mut self, r#type: i32, key: &str) -> Result<Info> {
        let info = self
            .first(r#type.clone(), key, None)
            .await?
            .ok_or(ServiceError::CacheNotFound)?;
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.r#type.ne(&r#type) && x.key.ne(key))
            .collect::<Vec<Info>>();
        Ok(info)
    }

    async fn flush(&mut self, r#type: Option<i32>) -> Result<i64> {
        if let Some(cache_type) = r#type {
            let count = self
                .data
                .clone()
                .into_iter()
                .filter(|x| x.r#type.eq(&cache_type))
                .count() as i64;
            self.data = self
                .data
                .clone()
                .into_iter()
                .filter(|x| x.r#type.ne(&cache_type))
                .collect::<Vec<Info>>();
            return Ok(count);
        }
        let count = self.data.len() as i64;
        self.data = vec![];
        Ok(count)
    }
}
