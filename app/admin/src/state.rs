use service::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<State>;

pub struct State {
    pub captcha: Mutex<utils::captcha::Captcha>,
    pub jwt: Mutex<utils::jwt::Jwt>,
    pub db: Database,
}

impl State {
    pub fn build(captcha: utils::captcha::Captcha, jwt: utils::jwt::Jwt, db: Database) -> AppState {
        use tokio::time::{interval, Duration};
        let state = Arc::new(Self {
            captcha: Mutex::new(captcha),
            jwt: Mutex::new(jwt),
            db,
        });

        let captcha_state = state.clone();
        let jwt_state = state.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                captcha_state.captcha.lock().await.get_interval(),
            ));
            loop {
                captcha_state.captcha.lock().await.remove_valid_items();
                interval.tick().await;
            }
        });
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                jwt_state.jwt.lock().await.get_interval(),
            ));
            loop {
                jwt_state.jwt.lock().await.remove_valid_items();
                interval.tick().await;
            }
        });
        state
    }
}
