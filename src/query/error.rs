use super::lexer::error::LexerError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum QueryError {
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError)
}

#[cfg(feature = "ffi")]
impl crate::error::IntoFfiErrorCode for QueryError {
    fn to_error_code(&self) -> libc::intptr_t {
        return match self {
            Self::LexerError(lexer_error) => lexer_error.to_error_code()
        };
    }
}

pub type QueryResult<T> = Result<T, QueryError>;
