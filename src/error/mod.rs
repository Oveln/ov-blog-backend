use std::fmt::Display;

pub type OvResult<T> = Result<T, OvError>;

#[derive(Debug)]
pub enum OvError {
    DatabaseError(String),
    DatabaseDataFormatError(String),

    UserAleadyExist,

    InvalidObjectId,

    NotFound,
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

impl From<mongodb::bson::document::ValueAccessError> for OvError {
    fn from(err: mongodb::bson::document::ValueAccessError) -> Self {
        OvError::DatabaseDataFormatError(err.to_string())
    }
}
