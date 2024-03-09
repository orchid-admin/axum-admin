use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<State>;

pub struct State {
    pub db: model::connect::DbConnectPool,
    pub cache: Mutex<service::cache::Cache<service::cache::DatabaseDriver>>,
}

impl State {
    pub fn build(db: model::connect::DbConnectPool) -> AppState {
        Arc::new(Self {
            db: db.clone(),
            cache: Mutex::new(service::cache::CacheDriver::new_database(db)),
        })
    }
}
