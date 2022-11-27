pub mod ask;
pub mod action;

use crate::{ClipError, DataError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Clip Error: {0}")]
    Clip(#[from] ClipError),
    #[error("Database Error: {0}")]
    Data(DataError),
    #[error("Not Found")]
    NotFound,
    #[error("Permission Denied: {0}")]
    PermissionError(String)
}
impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
            DataError::DataBase(d) => match d {
                sqlx::Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataError::DataBase(other))
            }
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::DataBase(other))
        }
    }
}
