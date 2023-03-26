use time::{Duration, Instant};

use crate::error::{ErrorCode, Result};

#[derive(Debug, Default, Clone)]
pub struct Captcha {
    data: Vec<CaptchaItem>,
}

#[allow(dead_code)]
impl Captcha {
    pub async fn run_check(&mut self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            if let Some(items) = self.get_valid_items() {
                for (_, index) in items.into_iter().enumerate() {
                    self.data.remove(index);
                }
            }
            interval.tick().await;
        }
    }

    pub fn generate(
        &mut self,
        use_type: &str,
        length: usize,
        width: u32,
        height: u32,
        dark_mode: bool,
        complexity: u32,
    ) -> Result<(String, String)> {
        let captcha = captcha_rs::CaptchaBuilder::new()
            .length(length)
            .width(width)
            .height(height)
            .dark_mode(dark_mode)
            .complexity(complexity)
            .compression(40)
            .build();

        let key = Instant::now().elapsed().whole_nanoseconds().to_string();
        match self.add(use_type, &key, &captcha.text)? {
            true => Ok((key, captcha.to_base64())),
            false => Err(ErrorCode::GenerateCaptcha),
        }
    }

    pub fn get_item(&self, use_type: &str, key: &str) -> Option<CaptchaItem> {
        for (_index, item) in self.data.clone().into_iter().enumerate() {
            if item.key.eq(&key) && item.use_type.eq(use_type) {
                return Some(item);
            }
        }
        None
    }

    pub fn remove_item_by_key(&mut self, use_type: &str, key: &str) -> bool {
        for (index, item) in self.data.clone().into_iter().enumerate() {
            if item.key.eq(&key) && item.use_type.eq(use_type) {
                self.data.remove(index);
                return true;
            }
        }
        false
    }

    fn add(&mut self, use_type: &str, key: &str, text: &str) -> Result<bool> {
        Ok(match self.get_item(use_type, &key) {
            Some(_) => false,
            None => {
                let exp = match Instant::now().checked_add(Duration::minutes(10)) {
                    Some(times) => Ok(times.elapsed().as_seconds_f64()),
                    None => Err(ErrorCode::GenerateCaptcha),
                }?;
                self.data.push(CaptchaItem {
                    use_type: use_type.to_owned(),
                    key: key.to_owned(),
                    text: text.to_owned(),
                    exp,
                });
                true
            }
        })
    }

    fn get_valid_items(&self) -> Option<Vec<usize>> {
        let mut array = vec![];
        for (index, item) in self.data.clone().into_iter().enumerate() {
            if !item.check() {
                array.push(index);
            }
        }

        if !array.is_empty() {
            return Some(array);
        }
        None
    }
}

#[derive(Debug, Clone)]
struct CaptchaItem {
    use_type: String,
    key: String,
    text: String,
    exp: f64,
}

#[allow(dead_code)]
impl CaptchaItem {
    pub fn get_text(self) -> String {
        self.text
    }

    pub fn check(self) -> bool {
        self.exp > Instant::now().elapsed().as_seconds_f64()
    }
}
