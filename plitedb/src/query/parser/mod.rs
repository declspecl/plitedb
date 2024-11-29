use ast::Statement;
use error::ParserResult;
use statement::parse_statement;

use super::{cursor::PeekingCursor, lexer::token::Token};

pub mod ast;
pub mod error;
pub mod expression;
pub mod statement;

pub fn parse<I: IntoIterator<Item = Token>>(tokens: I) -> ParserResult<Statement> {
    let mut tokens = PeekingCursor::new(tokens.into_iter());

    return parse_statement(&mut tokens);
}
