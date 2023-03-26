use argon2::{Algorithm, Argon2, Params, Version};
use password_hash::{
    rand_core::OsRng, Output, ParamsString, PasswordHash, PasswordHasher, PasswordVerifier, Salt,
    SaltString,
};

use crate::error::{ErrorCode, Result};
pub struct Password;

#[allow(dead_code)]
impl Password {
    /// new Params
    fn build_params() -> Result<Params> {
        Params::new(
            Params::MIN_M_COST,
            Params::MIN_T_COST,
            Params::MIN_P_COST,
            None,
        )
        .map_err(|_| ErrorCode::GeneratePassword)
    }

    /// new Argon2
    fn build() -> Result<Argon2<'static>> {
        Ok(Argon2::new(
            Algorithm::default(),
            Version::V0x13,
            Self::build_params()?,
        ))
    }

    /// generate hash and salt for password
    pub fn generate_hash_salt(password: &[u8]) -> Result<(String, String)> {
        let argon2 = Self::build()?;

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(|_| ErrorCode::GeneratePassword)?;

        let hash = match password_hash.hash {
            Some(hash) => Ok(hash.to_string()),
            None => Err(ErrorCode::GeneratePassword),
        }?;
        let salt = match password_hash.salt {
            Some(salt) => Ok(salt.to_string()),
            None => Err(ErrorCode::GeneratePassword),
        }?;
        Ok((hash, salt))
    }

    /// verify input password
    pub fn verify_password(hash: String, salt: String, input_password: &[u8]) -> Result<bool> {
        let argon2 = Self::build()?;

        let password_hash = PasswordHash {
            algorithm: Algorithm::default().ident(),
            version: Some(Version::V0x13.into()),
            params: ParamsString::try_from(&Self::build_params()?)
                .map_err(|_| ErrorCode::GeneratePassword)?,
            salt: Some(Salt::from_b64(&salt).map_err(|_| ErrorCode::GeneratePassword)?),
            hash: Some(Output::b64_decode(&hash).map_err(|_| ErrorCode::GeneratePassword)?),
        };
        Ok(argon2
            .verify_password(input_password, &password_hash)
            .is_ok())
    }
}
