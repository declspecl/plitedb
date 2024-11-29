use crate::query::{
    cursor::PeekingCursor,
    lexer::token::{Keyword, Token}
};

use super::{
    ast::{Assignment, Comparison, Expression, Statement},
    error::{ParserError, ParserResult},
    expression::parse_expression
};

pub fn parse_statement<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let statement = match tokens.next() {
        Some(Token::Keyword(Keyword::Get)) => parse_get_item(tokens),
        Some(Token::Keyword(Keyword::Put)) => parse_put_item(tokens),
        Some(token) => Err(ParserError::UnexpectedToken(token)),
        None => Err(ParserError::UnexpectedEndOfInput)
    };

    return Ok(statement?);
}

fn parse_get_item<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let store_name = match tokens.next() {
        Some(Token::Identifier(name)) => name,
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    };

    match tokens.next() {
        Some(Token::Keyword(Keyword::Where)) => (),
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    }

    match tokens.next() {
        Some(Token::LeftCurlyBrace) => (),
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    }

    let conditions = parse_conditions(tokens)?;

    match tokens.next() {
        Some(Token::RightCurlyBrace) => (),
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    }

    return Ok(Statement::GetItem { store_name, conditions });
}

fn parse_conditions<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Vec<Comparison>> {
    let mut conditions = Vec::new();

    loop {
        let comparison_expression = parse_expression(tokens, 0)?;

        if let Expression::Comparison(comparison) = comparison_expression {
            conditions.push(comparison);
        }
        else {
            eprintln!("EHRE);");
            return Err(ParserError::InvalidFieldName);
        }

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            },
            Some(_) => break,
            None => return Err(ParserError::UnexpectedEndOfInput)
        }
    }

    return Ok(conditions);
}

fn parse_put_item<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let store_name = match tokens.next() {
        Some(Token::Identifier(name)) => name,
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    };

    match tokens.next() {
        Some(Token::LeftCurlyBrace) => (),
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    }

    let assignments = parse_assignments(tokens)?;

    match tokens.next() {
        Some(Token::RightCurlyBrace) => (),
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => return Err(ParserError::UnexpectedEndOfInput)
    }

    return Ok(Statement::PutItem { store_name, assignments });
}

fn parse_assignments<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Vec<Assignment>> {
    let mut assignments = Vec::new();

    loop {
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

        let value = parse_expression(tokens, 0)?;

        assignments.push(Assignment { name, value });

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            },
            Some(_) => break,
            None => return Err(ParserError::UnexpectedEndOfInput)
        }
    }

    return Ok(assignments);
}
