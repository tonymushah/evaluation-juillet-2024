use std::num::ParseIntError;

use diesel::r2d2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Pool(#[from] r2d2::Error),
    #[error("{0}")]
    PoolConnection(#[from] r2d2::PoolError),
    #[error("{0}")]
    DieselConnection(#[from] diesel::ConnectionError),
    #[error("{0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("The env variable {0} is not found")]
    EnvNotFound(String),
    #[error("{0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("{0}")]
    JWT(#[from] jwt::Error),
    #[error("You cannot access this ressource")]
    Forbidden,
    #[error("The upsert result is not found ")]
    UpsertNotFound,
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("can not find the capture name {0}")]
    RegexCaptureNameNotFound(String),
    #[error("An unexpected `std::num::TryFromIntError` was caught")]
    TryFromInt,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
