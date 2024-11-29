use ast::Statement;
use error::ParserResult;
use statement::parse_statement;

use super::{
    cursor::{CursorTrackable, Location, PeekingCursor},
    lexer::token::Token
};

pub mod ast;
pub mod error;
pub mod expression;
pub mod statement;

impl CursorTrackable for Token {
    fn next_location(
        &self,
        _: Location
    ) -> Location {
        return self.location;
    }
}

pub fn parse<I: IntoIterator<Item = Token>>(tokens: I) -> ParserResult<Statement> {
    let mut tokens = PeekingCursor::new(tokens.into_iter());

    return parse_statement(&mut tokens);
}
