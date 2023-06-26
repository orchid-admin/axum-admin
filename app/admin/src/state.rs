use service::{cache_service, Database};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<State>;

pub struct State {
    pub db: Database,
    pub cache: Mutex<cache_service::Cache<cache_service::CacheDriverMemory>>,
}

impl State {
    pub fn build(db: Database) -> AppState {
        Arc::new(Self {
            db,
            cache: Mutex::new(cache_service::Cache::new(
                cache_service::CacheDriverMemory::default(),
            )),
        })
    }
}
