use crate::captcha::Captcha;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<State>;

pub struct State {
    pub captcha: Mutex<Captcha>,
}

impl State {
    pub fn build() -> AppState {
        let state = Arc::new(Self::new());
        let state_clone = state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
            loop {
                state_clone.captcha.lock().await.remove_valid_items();
                interval.tick().await;
            }
        });
        state
    }
    fn new() -> Self {
        Self {
            captcha: Mutex::new(Captcha::default()),
        }
    }
}
