#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LeftParenthesis,
    RightParenthesis,
    SingleQuote,
    DoubleQuote,
    Colon,
    Semicolon,

    Get,
    Put,

    Number(String),
    Identifier(String)
}
