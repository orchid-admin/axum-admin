use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Pool(PoolError),
    PoolBuild(BuildError),
    Diesel(diesel::result::Error),
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
