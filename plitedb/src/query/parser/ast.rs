use crate::query::lexer::token::Token;

use super::error::ParserError;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    PutItem { store_name: String, assignments: Vec<Assignment> },
    GetItem { store_name: String, conditions: Vec<Comparison> }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: String,
    pub value: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Value),
    Identifier(String),
    Comparison(Comparison),
    BinaryOperation(MathematicalOperation),
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
pub struct Comparison {
    pub left: Box<Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<Expression>
}

pub trait HasPrecedence {
    fn precedence(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual
}

impl HasPrecedence for ComparisonOperator {
    fn precedence(&self) -> u8 {
        return 1;
    }
}

impl TryFrom<&Token> for ComparisonOperator {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        return match value {
            Token::GreaterThan => Ok(ComparisonOperator::GreaterThan),
            Token::GreaterThanOrEqual => Ok(ComparisonOperator::GreaterThanOrEqual),
            Token::LessThan => Ok(ComparisonOperator::LessThan),
            Token::LessThanOrEqual => Ok(ComparisonOperator::LessThanOrEqual),
            Token::Equals => Ok(ComparisonOperator::Equal),
            Token::NotEquals => Ok(ComparisonOperator::NotEqual),
            _ => Err(ParserError::InvalidValue)
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathematicalOperation {
    pub left: Box<Expression>,
    pub operator: MathematicalOperator,
    pub right: Box<Expression>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathematicalOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    Exponentiate
}

impl HasPrecedence for MathematicalOperator {
    fn precedence(&self) -> u8 {
        return match self {
            MathematicalOperator::Add | MathematicalOperator::Subtract => 2,
            MathematicalOperator::Multiply | MathematicalOperator::Divide | MathematicalOperator::Modulus => 3,
            MathematicalOperator::Exponentiate => 4
        };
    }
}

impl TryFrom<&Token> for MathematicalOperator {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        return match value {
            Token::Plus => Ok(MathematicalOperator::Add),
            Token::Minus => Ok(MathematicalOperator::Subtract),
            Token::Asterisk => Ok(MathematicalOperator::Multiply),
            Token::Slash => Ok(MathematicalOperator::Divide),
            Token::Percent => Ok(MathematicalOperator::Modulus),
            Token::Caret => Ok(MathematicalOperator::Exponentiate),
            _ => Err(ParserError::InvalidOperatorValue(value.to_owned()))
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Comparison(ComparisonOperator),
    Mathematical(MathematicalOperator)
}

impl HasPrecedence for BinaryOperator {
    fn precedence(&self) -> u8 {
        return match self {
            BinaryOperator::Comparison(operator) => operator.precedence(),
            BinaryOperator::Mathematical(operator) => operator.precedence()
        };
    }
}

impl TryFrom<&Token> for BinaryOperator {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        return if let Ok(operator) = ComparisonOperator::try_from(value) {
            Ok(BinaryOperator::Comparison(operator))
        }
        else if let Ok(operator) = MathematicalOperator::try_from(value) {
            Ok(BinaryOperator::Mathematical(operator))
        }
        else {
            Err(ParserError::InvalidOperatorValue(value.to_owned()))
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOperator {
    Negate
}

impl HasPrecedence for UnaryOperator {
    fn precedence(&self) -> u8 {
        return 5;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>
}
