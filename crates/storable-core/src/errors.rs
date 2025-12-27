use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorableError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("conflict")]
    Conflict,

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal error")]
    Internal,
}
