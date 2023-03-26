use crate::captcha::Captcha;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<State>;

pub struct State {
    pub captcha: Mutex<Captcha>,
}

impl State {
    pub fn build() -> AppState {
        Arc::new(Self::new())
    }
    fn new() -> Self {
        Self {
            captcha: Mutex::new(Captcha::default()),
        }
    }
}
