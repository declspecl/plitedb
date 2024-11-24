use crate::query::{error::QueryError, lexer::error::LexerError};

#[derive(Debug, thiserror::Error)]
pub enum PliteDbError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
    #[error("Query error: {0}")]
    QueryError(#[from] QueryError)
}

#[cfg(feature = "ffi")] 
pub trait IntoFfiErrorCode {
    fn to_error_code(&self) -> libc::intptr_t;
}

#[cfg(feature = "ffi")]
impl IntoFfiErrorCode for PliteDbError {
    fn to_error_code(&self) -> libc::intptr_t {
        return match self {
            Self::IoError(_) => 1,
            Self::LexerError(lexer_error) => lexer_error.to_error_code(),
            Self::QueryError(query_error) => query_error.to_error_code(),
        };
    }
}

pub type PliteDbResult<T> = Result<T, PliteDbError>;
