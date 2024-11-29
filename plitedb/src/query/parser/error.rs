use crate::query::lexer::token::Token;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token '{:?}' at {}. Expected '{}' instead", .0.token_type, .0.location, .1)]
    UnexpectedToken(Token, String),
    #[error("Invalid numerical value '{0}'. Must be a valid integer or float")]
    InvalidNumericalValue(String),
    #[error("Invalid comparison operator '{:?}' at {}", .0.token_type, .0.location)]
    InvalidComparisonOperator(Token),
    #[error("Invalid mathematical operator '{:?}' at {}", .0.token_type, .0.location)]
    InvalidMathematicalOperator(Token),
    #[error("Invalid unary operator '{:?}' at {}", .0.token_type, .0.location)]
    InvalidUnaryOperator(Token),
    #[error("Missing identifier. Expected '{0}'")]
    MissingIdentifier(String),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput
}

pub type ParserResult<T> = Result<T, ParserError>;
