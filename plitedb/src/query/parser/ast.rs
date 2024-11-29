use crate::query::lexer::token::{Token, TokenType};

use super::error::ParserError;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    PutItem { store_name: String, assignments: Vec<Assignment> },
    GetItem { store_name: String, comparisons: Vec<Comparison> }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Expression
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual
}

impl TryFrom<&Token> for ComparisonOperator {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        return match value.token_type {
            TokenType::GreaterThan => Ok(ComparisonOperator::GreaterThan),
            TokenType::GreaterThanOrEqual => Ok(ComparisonOperator::GreaterThanOrEqual),
            TokenType::LessThan => Ok(ComparisonOperator::LessThan),
            TokenType::LessThanOrEqual => Ok(ComparisonOperator::LessThanOrEqual),
            TokenType::Equal => Ok(ComparisonOperator::Equal),
            TokenType::NotEqual => Ok(ComparisonOperator::NotEqual),
            _ => Err(ParserError::InvalidComparisonOperator(value.to_owned()))
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub name: String,
    pub operator: ComparisonOperator,
    pub value: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Value),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(bool)
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponentiate
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        return match self {
            BinaryOperator::Exponentiate => 3,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulus => 2,
            BinaryOperator::Add | BinaryOperator::Subtract => 1
        };
    }
}

impl TryFrom<&Token> for BinaryOperator {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        return match value.token_type {
            TokenType::Plus => Ok(BinaryOperator::Add),
            TokenType::Minus => Ok(BinaryOperator::Subtract),
            TokenType::Asterisk => Ok(BinaryOperator::Multiply),
            TokenType::Slash => Ok(BinaryOperator::Divide),
            TokenType::Percent => Ok(BinaryOperator::Modulus),
            TokenType::Caret => Ok(BinaryOperator::Exponentiate),
            _ => Err(ParserError::InvalidMathematicalOperator(value.to_owned()))
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOperator {
    Negate
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>
}

impl TryFrom<&Token> for UnaryOperator {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        return match value.token_type {
            TokenType::Minus => Ok(UnaryOperator::Negate),
            _ => Err(ParserError::InvalidUnaryOperator(value.to_owned()))
        };
    }
}
