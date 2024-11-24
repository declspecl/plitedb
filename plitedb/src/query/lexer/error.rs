#[derive(Debug, Clone, thiserror::Error)]
pub enum LexerError {
    #[error("Unexpected character '{0}'")]
    UnexpectedCharacter(char),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput
}

pub type LexerResult<T> = Result<T, LexerError>;
