use super::Driver;
use crate::{cache::Info, Result, ServiceError};
use model::{connect::DbConnectPool, system_cache};
use serde::Serialize;

pub struct Database(pub DbConnectPool);

#[async_trait::async_trait]
impl Driver for Database {
    async fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: i32,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> Result<Info> {
        let mut conn = self.0.conn().await?;
        Ok(system_cache::Entity::create(
            &mut conn,
            &system_cache::FormParamsForCreate {
                key: key.to_owned(),
                r#type,
                value: serde_json::to_string(&value)?,
                attach: attach.unwrap_or_default(),
                valid_time_length: valid_time_length.map(|x| x as i32),
            },
        )
        .await?
        .into())
    }

    async fn first(&self, r#type: i32, key: &str, default: Option<Info>) -> Result<Option<Info>> {
        let mut conn = self.0.conn().await?;
        let info = system_cache::Entity::find(
            &mut conn,
            &system_cache::Filter {
                r#type: Some(r#type),
                key: Some(key.to_owned()),
                ..Default::default()
            },
        )
        .await?
        .map(|x| x.into());
        if info.is_none() {
            return Ok(default);
        }
        Ok(info)
    }
    async fn pull(&mut self, r#type: i32, key: &str) -> Result<Info> {
        let mut conn = self.0.conn().await?;
        let info = system_cache::Entity::find(
            &mut conn,
            &system_cache::Filter {
                key: Some(key.to_owned()),
                r#type: Some(r#type),
                ..Default::default()
            },
        )
        .await?
        .ok_or(ServiceError::CacheNotFound)?;
        Ok(system_cache::Entity::soft_delete(&mut conn, info.id)
            .await?
            .into())
    }

    async fn flush(&mut self, r#type: Option<i32>) -> Result<i64> {
        let mut conn = self.0.conn().await?;
        let infos = system_cache::Entity::soft_delete_transaction(&mut conn, r#type).await?;
        Ok(infos.len() as i64)
    }
}

impl From<system_cache::Entity> for Info {
    fn from(value: system_cache::Entity) -> Self {
        let mut attach = None;
        if value.attach.is_empty() {
            attach = Some(value.attach);
        }
        Self {
            key: value.key,
            r#type: value.r#type,
            value: value.value,
            attach,
            valid_time_length: value.valid_time_length.map(|x| x as i64),
            create_time: value
                .created_at
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        }
    }
}
