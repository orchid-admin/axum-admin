use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, Instant};

use crate::error::{ErrorCode, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i64,
    exp: f64,
}

#[allow(dead_code)]
impl Claims {
    const SECRET: &str = "secret";
    pub fn decode(token: String) -> Result<Self> {
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&Self::SECRET.as_ref()),
            &Validation::default(),
        ) {
            Ok(claims) => Ok(claims.claims),
            Err(_) => Err(ErrorCode::TokenParse),
        }
    }

    pub fn to_token(user_id: i64) -> Result<String> {
        let exp = match Instant::now().checked_add(Duration::days(1)) {
            Some(times) => Ok(times.elapsed().as_seconds_f64()),
            None => Err(ErrorCode::GenerateToken),
        }?;
        let claims = Claims { user_id, exp };
        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&Self::SECRET.as_ref()),
        ) {
            Ok(token) => Ok(token),
            Err(_) => Err(ErrorCode::GenerateToken),
        }
    }

    pub fn check(&self) -> bool {
        self.exp > Instant::now().elapsed().as_seconds_f64()
    }
}
