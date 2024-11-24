pub mod cursor;
pub mod error;
pub mod token;

use cursor::PeekingCursor;
use error::{LexerError, LexerResult};
use token::Token;

pub fn tokenize(haystack: &str) -> LexerResult<Vec<Token>> {
    let mut tokens = Vec::with_capacity(32);

    let mut chars = PeekingCursor::new(haystack);
    while let Some(char) = chars.peek() {
        if char.is_whitespace() {
            chars.next();
            continue;
        };

        if char.is_numeric() {
            tokens.push(Token::Number(
                chars.peek_and_take_while(|next| next.is_numeric() || *next == '.')
            ));
        }
        else if char.is_alphabetic() {
            let string = chars.peek_and_take_while(|next| next.is_alphanumeric());
            let token = match string.len() {
                3 => match &string[..] {
                    "GET" => Token::Get,
                    "PUT" => Token::Put,
                    _ => Token::Identifier(string)
                },
                _ => Token::Identifier(string)
            };

            tokens.push(token);
        }
        else {
            let next = chars.next().unwrap();

            match next {
                '(' => tokens.push(Token::LeftParenthesis),
                ')' => tokens.push(Token::RightParenthesis),
                '\'' => tokens.push(Token::SingleQuote),
                '"' => tokens.push(Token::DoubleQuote),
                ':' => tokens.push(Token::Colon),
                ';' => tokens.push(Token::Semicolon),
                _ => return Err(LexerError::UnexpectedCharacter(next))
            }
        }
    }

    return Ok(tokens);
}
