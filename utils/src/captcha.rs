pub enum ErrorType {
    JsonwebToken(jsonwebtoken::errors::Error),
    GenerateFail,
}
type Result<T> = std::result::Result<T, ErrorType>;

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
        use_type: UseType,
        length: usize,
        width: u32,
        height: u32,
        dark_mode: bool,
        complexity: u32,
    ) -> Result<CaptchaContent> {
        let captcha = captcha_rs::CaptchaBuilder::new()
            .length(length)
            .width(width)
            .height(height)
            .dark_mode(dark_mode)
            .complexity(complexity)
            .compression(40)
            .build();

        let key = crate::datetime::timestamp_nanos_string(None);
        self.add(use_type, &key, &captcha.text)
            .map(|_| CaptchaContent {
                key,
                text: captcha.text.to_owned(),
                image: captcha.to_base64(),
            })
    }

    /// get item by key
    pub fn get_item(&self, use_type: &UseType, key: &str) -> Option<CaptchaItem> {
        self.data
            .clone()
            .into_iter()
            .find(|x| x.key.eq(&key) && x.use_type.eq(use_type))
    }

    /// remove valid captcha cache
    pub fn remove_valid_items(&mut self) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.check())
            .collect::<Vec<CaptchaItem>>();
    }

    /// add captcha cache
    fn add(&mut self, use_type: UseType, key: &str, text: &str) -> Result<CaptchaItem> {
        match self.get_item(&use_type, key) {
            Some(_) => Err(ErrorType::GenerateFail),
            None => {
                let exp = crate::datetime::timestamp_nanos(Some(self.valid_seconds));
                let item = CaptchaItem {
                    use_type,
                    key: key.to_owned(),
                    text: text.to_owned(),
                    exp,
                };
                self.data.push(item.clone());
                Ok(item)
            }
        }
    }

    pub fn remove_item(&mut self, item: &CaptchaItem) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| !x.eq(item))
            .collect::<Vec<CaptchaItem>>();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseType {
    AdminLogin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptchaItem {
    use_type: UseType,
    key: String,
    text: String,
    exp: i64,
}

#[allow(unused)]
impl CaptchaItem {
    /// check captcha is can use
    pub fn check(&self) -> bool {
        self.exp > crate::datetime::timestamp_nanos(None)
    }

    /// verify text by lowercase
    pub fn verify_lowercase(&self, text: &str) -> bool {
        let origin_text = self.text.to_lowercase();
        let input_text = text.to_lowercase();
        self.check() && origin_text.eq(&input_text)
    }

    pub fn verify(&self, text: &str) -> bool {
        self.check() && self.text.eq(text)
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptchaContent {
    pub key: String,
    pub text: String,
    pub image: String,
}
