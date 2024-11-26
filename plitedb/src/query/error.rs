use super::{lexer::error::LexerError, parser::error::ParserError};

#[derive(Debug, Clone, thiserror::Error)]
pub enum QueryError {
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
    #[error("Parser error: {0}")]
    ParserError(#[from] ParserError)
}

pub type QueryResult<T> = Result<T, QueryError>;
