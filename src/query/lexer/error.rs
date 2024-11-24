#[derive(Debug, Clone, thiserror::Error)]
pub enum LexerError {
    #[error("Unexpected character '{0}'")]
    UnexpectedCharacter(char),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput
}

#[cfg(feature = "ffi")]
impl crate::error::IntoFfiErrorCode for LexerError {
    fn to_error_code(&self) -> libc::intptr_t {
        return match self {
            Self::UnexpectedCharacter(_) => 1101,
            Self::UnexpectedEndOfInput => 1102
        };
    }
}

pub type LexerResult<T> = Result<T, LexerError>;
