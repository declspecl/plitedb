use crate::query::{
    cursor::PeekingCursor,
    lexer::token::{Keyword, Token}
};

use super::{
    ast::{Assignment, Comparison, ComparisonOperator, Statement},
    error::{ParserError, ParserResult},
    expression::parse_expression
};

pub fn parse_statement<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let statement = match tokens.next() {
        Some(Token::Keyword(Keyword::Get)) => parse_get_item(tokens),
        Some(Token::Keyword(Keyword::Put)) => parse_put_item(tokens),
        Some(token) => Err(ParserError::UnexpectedToken(token, "GET or PUT".to_string())),
        None => Err(ParserError::UnexpectedEndOfInput)
    };

    return Ok(statement?);
}

fn parse_get_item<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let store_name = parse_store_name(tokens)?;

    expect_where(tokens)?;
    expect_left_curly_brace(tokens)?;

    let comparisons = parse_comparisons(tokens)?;

    expect_right_curly_brace(tokens)?;

    return Ok(Statement::GetItem { store_name, comparisons });
}

fn parse_put_item<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Statement> {
    let store_name = parse_store_name(tokens)?;

    expect_left_curly_brace(tokens)?;

    let assignments = parse_assignments(tokens)?;

    expect_right_curly_brace(tokens)?;

    return Ok(Statement::PutItem { store_name, assignments });
}

fn parse_comparisons<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Vec<Comparison>> {
    let mut comparisons = Vec::new();

    loop {
        let name = parse_attribute_name(tokens)?;
        let operator = parse_comparison_operator(tokens)?;
        let value = parse_expression(tokens, 0)?;

        comparisons.push(Comparison { name, operator, value });

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            },
            Some(_) => break,
            None => return Err(ParserError::UnexpectedEndOfInput)
        }
    }

    return Ok(comparisons);
}

fn parse_assignments<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Vec<Assignment>> {
    let mut assignments = Vec::new();

    loop {
        let name = parse_attribute_name(tokens)?;

        expect_colon(tokens)?;

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

fn parse_store_name<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<String> {
    return match tokens.next() {
        Some(Token::Identifier(name)) => Ok(name),
        Some(token) => Err(ParserError::UnexpectedToken(token, "store name".to_string())),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn parse_attribute_name<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<String> {
    return match tokens.next() {
        Some(Token::Identifier(name)) => Ok(name),
        Some(Token::String(name)) => Ok(name),
        Some(token) => Err(ParserError::UnexpectedToken(
            token,
            "attribute name, as a string or identifier".to_string()
        )),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn parse_comparison_operator<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<ComparisonOperator> {
    return match tokens.next() {
        Some(token) => match ComparisonOperator::try_from(&token) {
            Ok(operator) => Ok(operator),
            Err(_) => Err(ParserError::InvalidComparisonOperator(token))
        },
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn expect_colon<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<()> {
    return match tokens.next() {
        Some(Token::Colon) => Ok(()),
        Some(token) => Err(ParserError::UnexpectedToken(token, ":".to_string())),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn expect_where<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<()> {
    return match tokens.next() {
        Some(Token::Keyword(Keyword::Where)) => Ok(()),
        Some(token) => Err(ParserError::UnexpectedToken(token, "WHERE".to_string())),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn expect_left_curly_brace<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<()> {
    return match tokens.next() {
        Some(Token::LeftCurlyBrace) => Ok(()),
        Some(token) => Err(ParserError::UnexpectedToken(token, "{".to_string())),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}

fn expect_right_curly_brace<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<()> {
    return match tokens.next() {
        Some(Token::RightCurlyBrace) => Ok(()),
        Some(token) => Err(ParserError::UnexpectedToken(token, "}".to_string())),
        None => Err(ParserError::UnexpectedEndOfInput)
    };
}
