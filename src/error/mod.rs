use std::fmt::Display;

pub type OvResult<T> = Result<T, OvError>;

#[derive(Debug)]
pub enum OvError {
    DatabaseError(String),
    DatabaseDataFormatError,

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
