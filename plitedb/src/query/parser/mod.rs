use ast::{Assignment, ComparisonOperator, Condition, Expression, Statement, Value};
use error::{ParserError, ParserResult};

use super::{
    cursor::PeekingCursor,
    lexer::token::{Keyword, Token}
};

pub mod ast;
pub mod error;

pub fn parse<I: IntoIterator<Item = Token>>(tokens: I) -> ParserResult<Statement> {
    let mut tokens = PeekingCursor::new(tokens.into_iter());

    return match tokens.next() {
        Some(Token::Keyword(Keyword::Put)) => parse_put_item(&mut tokens),
        Some(Token::Keyword(Keyword::Get)) => parse_get_item(&mut tokens),
        Some(token) => Err(ParserError::UnexpectedToken(token)),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn parse_put_item<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let store_name = match tokens.next() {
        Some(Token::Identifier(name)) => name,
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    };

    let mut assignments = Vec::new();
    while let Ok(assignment) = parse_assignment(tokens) {
        assignments.push(assignment);

        match tokens.next() {
            Some(Token::Comma) => continue,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => break
        }
    }

    return Ok(Statement::PutItem { store_name, assignments });
}

fn parse_assignment<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Assignment> {
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => name,
        Some(Token::String(name)) => name,
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    };

    match tokens.next() {
        Some(Token::Colon) => (),
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    }

    let value = parse_expression(tokens)?;

    return Ok(Assignment { name, value });
}

fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Expression> {}
