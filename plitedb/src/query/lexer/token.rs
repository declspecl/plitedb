#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LeftParenthesis,
    RightParenthesis,

    Colon,
    Semicolon,

    Comma,
    Period,

    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equals,
    NotEquals,

    Asterisk,
    Plus,
    Minus,
    Slash,
    Caret,

    Keyword(Keyword),

    Number(String),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Get,
    Put,
    Where,
    True,
    False
}