use crate::query::lexer::token::Token;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token '{:?}'", .0)]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Missing store name")]
    MissingStore,
    #[error("Missing parenthesis")]
    MissingParenthesis,
    #[error("Invalid field name")]
    InvalidFieldName,
    #[error("Missing colon")]
    MissingColon,
    #[error("Invalid value")]
    InvalidValue,
    #[error("Invalid operator token '{:?}'", .0)]
    InvalidOperatorValue(Token)
}

pub type ParserResult<T> = Result<T, ParserError>;
