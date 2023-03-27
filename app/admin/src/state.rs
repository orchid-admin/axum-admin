use crate::{captcha::Captcha, jwt::Jwt};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<State>;

pub struct State {
    pub captcha: Mutex<Captcha>,
    pub jwt: Mutex<Jwt>,
}

impl State {
    pub fn build() -> AppState {
        use tokio::time::{interval, Duration};
        let state = Arc::new(Self::new());
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
    fn new() -> Self {
        Self {
            captcha: Mutex::new(Captcha::new(2, 10 * 60)),
            jwt: Mutex::new(Jwt::new("secret", 2, 7)),
        }
    }
}
