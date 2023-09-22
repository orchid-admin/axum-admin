#[derive(Debug)]
pub enum ErrorType {
    Argon2(argon2::Error),
    Hash(password_hash::Error),
}
type Result<T> = std::result::Result<T, ErrorType>;
pub struct Password;

impl Password {
    /// new Params
    fn build_params() -> Result<argon2::Params> {
        argon2::Params::new(
            argon2::Params::MIN_M_COST,
            argon2::Params::MIN_T_COST,
            argon2::Params::MIN_P_COST,
            None,
        )
        .map_err(ErrorType::Argon2)
    }

    /// new Argon2
    fn build() -> Result<argon2::Argon2<'static>> {
        Ok(argon2::Argon2::new(
            argon2::Algorithm::default(),
            argon2::Version::V0x13,
            Self::build_params()?,
        ))
    }

    /// generate hash and salt for password
    pub fn generate_hash_salt(password: &[u8]) -> Result<(String, String)> {
        use password_hash::PasswordHasher;

        let argon2 = Self::build()?;

        let salt = password_hash::SaltString::generate(&mut password_hash::rand_core::OsRng);
        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(ErrorType::Hash)?;

        let hash = password_hash.hash.unwrap().to_string();
        let salt = password_hash.salt.unwrap().to_string();
        Ok((hash, salt))
    }

    /// verify input password
    pub fn verify_password(hash: &str, salt: &str, input_password: &[u8]) -> Result<bool> {
        use password_hash::PasswordVerifier;

        let argon2 = Self::build()?;

        let password_hash = password_hash::PasswordHash {
            algorithm: argon2::Algorithm::default().ident(),
            version: Some(argon2::Version::V0x13.into()),
            params: password_hash::ParamsString::try_from(&Self::build_params()?)
                .map_err(ErrorType::Hash)?,
            salt: Some(password_hash::Salt::from_b64(salt).map_err(ErrorType::Hash)?),
            hash: Some(password_hash::Output::b64_decode(hash).map_err(ErrorType::Hash)?),
        };
        Ok(argon2
            .verify_password(input_password, &password_hash)
            .is_ok())
    }
}
