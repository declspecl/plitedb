use crate::query::cursor::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,

    Colon,
    Semicolon,

    Comma,
    Period,

    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,

    Asterisk,
    Plus,
    Minus,
    Slash,
    Percent,
    Caret,

    Keyword(Keyword),

    Number(String),
    String(String),
    Identifier(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location
}

impl Token {
    pub fn new(
        token_type: TokenType,
        location: Location
    ) -> Self {
        return Token { token_type, location };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Get,
    Put,
    Where,
    True,
    False
}
