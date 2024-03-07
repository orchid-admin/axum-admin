use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Pool(PoolError),
    PoolBuild(BuildError),
    Diesel(diesel::result::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Error::Pool(err) => err.to_string(),
            Error::PoolBuild(err) => err.to_string(),
            Error::Diesel(err) => err.to_string(),
        };
        write!(f, "{}", err)
    }
}

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Self {
        Error::Pool(err)
    }
}
impl From<BuildError> for Error {
    fn from(err: BuildError) -> Self {
        Error::PoolBuild(err)
    }
}
impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}
