use crate::error::{ErrorCode, Result};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone)]
pub struct Captcha {
    data: Vec<CaptchaItem>,
    interval: u64,
    valid_seconds: i64,
}

impl Default for Captcha {
    fn default() -> Self {
        Self {
            data: vec![],
            interval: 1,
            valid_seconds: 10 * 60,
        }
    }
}
#[allow(dead_code)]
impl Captcha {
    /// new with interval
    pub fn new(interval: u64, valid_seconds: i64) -> Self {
        Self {
            data: vec![],
            interval,
            valid_seconds,
        }
    }

    /// get interval
    pub fn get_interval(&self) -> u64 {
        self.interval
    }
    /// genrate captcha
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

        let key = OffsetDateTime::now_utc().unix_timestamp_nanos().to_string();
        match self.add(use_type, &key, &captcha.text)? {
            true => Ok((key, captcha.to_base64())),
            false => Err(ErrorCode::GenerateCaptcha),
        }
    }

    /// get items by use_type
    pub fn get_items(&self, use_type: &str) -> Vec<CaptchaItem> {
        self.data
            .clone()
            .into_iter()
            .filter(|x| x.use_type.eq(use_type))
            .collect::<Vec<CaptchaItem>>()
    }

    /// get item by key
    pub fn get_item(&self, use_type: &str, key: &str) -> Option<CaptchaItem> {
        self.data
            .clone()
            .into_iter()
            .find(|x| x.key.eq(&key) && x.use_type.eq(use_type))
    }

    /// remvoe captcha by key
    pub fn remove_item_by_key(&mut self, use_type: &str, key: &str) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| !x.key.eq(&key) && !x.use_type.eq(use_type))
            .collect::<Vec<CaptchaItem>>();
    }

    /// remove valid captcha cache
    pub fn remove_valid_items(&mut self) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.clone().check())
            .collect::<Vec<CaptchaItem>>();
    }

    /// add captcha cache
    fn add(&mut self, use_type: &str, key: &str, text: &str) -> Result<bool> {
        Ok(match self.get_item(use_type, &key) {
            Some(_) => false,
            None => {
                let exp = match OffsetDateTime::now_utc()
                    .checked_add(Duration::seconds(self.valid_seconds))
                {
                    Some(times) => Ok(times.unix_timestamp_nanos()),
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
}

#[derive(Debug, Clone)]
pub struct CaptchaItem {
    use_type: String,
    key: String,
    text: String,
    exp: i128,
}

#[allow(dead_code)]
impl CaptchaItem {
    /// get text
    pub fn get_text(self) -> String {
        self.text
    }

    /// check captcha is can use
    pub fn check(self) -> bool {
        self.exp > OffsetDateTime::now_utc().unix_timestamp_nanos()
    }
}