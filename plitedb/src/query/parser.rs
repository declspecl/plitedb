use super::{cursor::PeekingCursor, lexer::token::Token};

pub fn parse_tokens<I: IntoIterator<Item = Token>>(tokens: I) {
    let mut tokens = PeekingCursor::new(tokens.into_iter());
    while let Some(_token) = tokens.peek() {
        println!("{:#?}", tokens.next().unwrap());
    }
}
