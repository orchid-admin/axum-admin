use time::{Duration, OffsetDateTime};

use crate::error::{ErrorCode, Result};

#[derive(Debug, Default, Clone)]
pub struct Captcha {
    data: Vec<CaptchaItem>,
}

#[allow(dead_code)]
impl Captcha {
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

    pub fn get_items(&self, use_type: &str) -> Vec<CaptchaItem> {
        let mut array = vec![];
        for item in self.data.clone().into_iter() {
            if item.use_type.eq(use_type) {
                array.push(item);
            }
        }
        array
    }

    pub fn get_item(&self, use_type: &str, key: &str) -> Option<CaptchaItem> {
        for item in self.data.clone().into_iter() {
            if item.key.eq(&key) && item.use_type.eq(use_type) {
                return Some(item);
            }
        }
        None
    }

    pub fn remove_item_by_key(&mut self, use_type: &str, key: &str) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| !x.key.eq(&key) && !x.use_type.eq(use_type))
            .collect::<Vec<CaptchaItem>>();
    }

    fn add(&mut self, use_type: &str, key: &str, text: &str) -> Result<bool> {
        Ok(match self.get_item(use_type, &key) {
            Some(_) => false,
            None => {
                let exp = match OffsetDateTime::now_utc().checked_add(Duration::seconds(5)) {
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

    pub fn remove_valid_items(&mut self) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.clone().check())
            .collect::<Vec<CaptchaItem>>();
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
    pub fn get_text(self) -> String {
        self.text
    }

    pub fn check(self) -> bool {
        self.exp > OffsetDateTime::now_utc().unix_timestamp_nanos()
    }
}
