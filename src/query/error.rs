use super::lexer::error::LexerError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum QueryError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError)
}

pub type QueryResult<T> = Result<T, QueryError>;
