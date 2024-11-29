#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
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
pub enum Keyword {
    Get,
    Put,
    Where,
    True,
    False
}
