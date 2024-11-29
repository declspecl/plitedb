use crate::query::lexer::token::Token;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token '{0:?}'. Expected '{1}' instead")]
    UnexpectedToken(Token, String),
    #[error("Invalid numerical value '{0}'. Must be a valid integer or float")]
    InvalidNumericalValue(String),
    #[error("Invalid comparison operator '{0:?}'")]
    InvalidComparisonOperator(Token),
    #[error("Invalid mathematical operator '{0:?}'")]
    InvalidMathematicalOperator(Token),
    #[error("Invalid unary operator '{0:?}'")]
    InvalidUnaryOperator(Token),
    #[error("Missing identifier. Expected '{0}'")]
    MissingIdentifier(String),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput
}

pub type ParserResult<T> = Result<T, ParserError>;
