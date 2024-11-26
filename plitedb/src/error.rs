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

pub type PliteDbResult<T> = Result<T, PliteDbError>;
