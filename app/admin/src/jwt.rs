use crate::error::{ErrorCode, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use time::{Duration, OffsetDateTime};

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
#[allow(dead_code)]
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
    pub fn generate<T>(&mut self, use_type: &str, claims: T) -> Result<String>
    where
        T: Serialize,
    {
        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret.as_ref()),
        ) {
            Ok(token) => Ok(token),
            Err(_) => Err(ErrorCode::GenerateToken),
        }?;
        match self.add(use_type, &token)? {
            true => Ok(token),
            false => Err(ErrorCode::GenerateToken),
        }
    }

    /// get items
    pub fn get_items(&self, use_type: &str) -> Vec<JwtItem> {
        self.data
            .clone()
            .into_iter()
            .filter(|x| x.use_type.eq(use_type))
            .collect::<Vec<JwtItem>>()
    }

    /// get item by key
    pub fn get_item(&self, use_type: &str, token: &str) -> Option<JwtItem> {
        self.data
            .clone()
            .into_iter()
            .find(|x| x.token.eq(&token) && x.use_type.eq(use_type))
    }

    /// decode token
    pub fn decode<T: DeserializeOwned>(&self, token: String) -> Result<T> {
        match decode::<T>(
            &token,
            &DecodingKey::from_secret(&self.secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(claims) => Ok(claims.claims),
            Err(_) => Err(ErrorCode::TokenParse),
        }
    }

    /// remove valid captcha cache
    pub fn remove_valid_items(&mut self) {
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.clone().check())
            .collect::<Vec<JwtItem>>();
    }

    /// add item
    fn add(&mut self, use_type: &str, token: &str) -> Result<bool> {
        Ok(match self.get_item(use_type, token) {
            Some(_) => false,
            None => {
                let exp = match OffsetDateTime::now_utc()
                    .checked_add(Duration::seconds(self.valid_seconds))
                {
                    Some(times) => Ok(times.unix_timestamp_nanos()),
                    None => Err(ErrorCode::GenerateCaptcha),
                }?;
                self.data.push(JwtItem {
                    use_type: use_type.to_owned(),
                    token: token.to_owned(),
                    exp,
                });
                true
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct JwtItem {
    use_type: String,
    token: String,
    exp: i128,
}

impl JwtItem {
    pub fn check(&self) -> bool {
        self.exp > OffsetDateTime::now_utc().unix_timestamp_nanos()
    }
}
