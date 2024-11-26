use error::QueryResult;
use lexer::tokenize;
use parser::{ast::Expression, parse};

pub mod cursor;
pub mod error;
pub mod lexer;
pub mod parser;

pub fn parse_query(query: &str) -> QueryResult<Expression> {
    return Ok(parse(tokenize(query)?)?);
}
