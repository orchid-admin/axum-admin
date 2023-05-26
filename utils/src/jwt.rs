pub enum ErrorType {
    JsonwebToken(jsonwebtoken::errors::Error),
    GenerateFail,
}
type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug, Clone)]
pub struct Jwt {
    data: Vec<JwtItem>,
    secret: String,
    interval: u64,
    valid_seconds: i64,
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            data: vec![],
            secret: "secret".to_owned(),
            interval: 1,
            valid_seconds: 7 * 24 * 60,
        }
    }
}

impl Jwt {
    /// new
    pub fn new(secret: &str, interval: u64, valid_seconds: i64) -> Self {
        Self {
            data: vec![],
            secret: secret.to_owned(),
            interval,
            valid_seconds,
        }
    }

    /// get interval
    pub fn get_interval(&self) -> u64 {
        self.interval
    }

    /// generate token
    pub fn generate<T>(&mut self, use_type: UseType, claims: T) -> Result<String>
    where
        T: serde::Serialize,
    {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(ErrorType::JsonwebToken)?;
        self.add(use_type, &token).map(|_| token)
    }

    /// get item by key
    pub fn get_item(&self, use_type: &UseType, token: &str) -> Option<JwtItem> {
        self.data
            .clone()
            .into_iter()
            .find(|x| x.token.eq(token) && x.use_type.eq(use_type))
    }

    /// decode token
    pub fn decode<T>(&self, token: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        jsonwebtoken::decode::<T>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map(|x| x.claims)
        .map_err(ErrorType::JsonwebToken)
    }

    /// remove valid captcha cache
    pub fn remove_valid_items(&mut self) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.check())
            .collect::<Vec<JwtItem>>();
    }

    /// add item
    fn add(&mut self, use_type: UseType, token: &str) -> Result<JwtItem> {
        match self.get_item(&use_type, token) {
            Some(_) => Err(ErrorType::GenerateFail),
            None => {
                let exp = time::OffsetDateTime::now_utc()
                    .checked_add(time::Duration::seconds(self.valid_seconds))
                    .map(|x| x.unix_timestamp_nanos())
                    .ok_or(ErrorType::GenerateFail)?;
                let item = JwtItem {
                    use_type,
                    token: token.to_owned(),
                    exp,
                };
                self.data.push(item.clone());
                Ok(item)
            }
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseType {
    Admin,
    User,
    Merchant,
}

#[derive(Debug, Clone)]
pub struct JwtItem {
    use_type: UseType,
    token: String,
    exp: i128,
}

impl JwtItem {
    pub fn check(&self) -> bool {
        self.exp > time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    }
}
