use std::fmt::Display;

use mongodb::bson::document::ValueAccessError;

pub type OvResult<T> = Result<T, OvError>;

#[derive(Debug)]
pub enum OvError {
    DatabaseError(String),
    DatabaseDataFormatError(String),

    UserAleadyExist,
}

impl Display for OvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<mongodb::error::Error> for OvError {
    fn from(err: mongodb::error::Error) -> Self {
        OvError::DatabaseError(err.to_string())
    }
}

impl From<ValueAccessError> for OvError {
    fn from(err: ValueAccessError) -> Self {
        OvError::DatabaseDataFormatError(err.to_string())
    }
}