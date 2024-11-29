use crate::query::lexer::token::Token;

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
        return match value {
            Token::GreaterThan => Ok(ComparisonOperator::GreaterThan),
            Token::GreaterThanOrEqual => Ok(ComparisonOperator::GreaterThanOrEqual),
            Token::LessThan => Ok(ComparisonOperator::LessThan),
            Token::LessThanOrEqual => Ok(ComparisonOperator::LessThanOrEqual),
            Token::Equal => Ok(ComparisonOperator::Equal),
            Token::NotEqual => Ok(ComparisonOperator::NotEqual),
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
        return match value {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Subtract),
            Token::Asterisk => Ok(BinaryOperator::Multiply),
            Token::Slash => Ok(BinaryOperator::Divide),
            Token::Percent => Ok(BinaryOperator::Modulus),
            Token::Caret => Ok(BinaryOperator::Exponentiate),
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
        return match value {
            Token::Minus => Ok(UnaryOperator::Negate),
            _ => Err(ParserError::InvalidUnaryOperator(value.to_owned()))
        };
    }
}
