pub mod error;
pub mod token;

use super::cursor::{CursorTrackable, Location, PeekingCursor};

use error::{LexerError, LexerResult};
use token::{Keyword, Token, TokenType};

impl CursorTrackable for char {
    fn next_location(
        &self,
        location: Location
    ) -> Location {
        let column = if *self == '\n' { 0 } else { location.column + 1 };
        let line = if *self == '\n' { location.line + 1 } else { location.line };

        return Location { line, column };
    }
}

pub fn tokenize(haystack: &str) -> LexerResult<Vec<Token>> {
    let mut tokens = Vec::with_capacity(32);

    let mut chars = PeekingCursor::new(haystack.chars());
    while let Some(char) = chars.peek() {
        if char.is_whitespace() {
            chars.next();
            continue;
        };

        if char.is_numeric() {
            tokens.push(Token::new(
                TokenType::Number(String::from_iter(
                    chars.peek_and_take_while(|next| next.is_numeric() || *next == '.')
                )),
                chars.loc()
            ));
        }
        else if char.is_alphabetic() || *char == '_' {
            let string = String::from_iter(chars.peek_and_take_while(|next| next.is_alphanumeric() || *next == '_'));
            let token_type = match string.len() {
                3 => match &string[..] {
                    "GET" => TokenType::Keyword(Keyword::Get),
                    "PUT" => TokenType::Keyword(Keyword::Put),
                    _ => TokenType::Identifier(string)
                },
                4 => match &string[..] {
                    "true" => TokenType::Keyword(Keyword::True),
                    "false" => TokenType::Keyword(Keyword::False),
                    _ => TokenType::Identifier(string)
                },
                5 => match &string[..] {
                    "WHERE" => TokenType::Keyword(Keyword::Where),
                    _ => TokenType::Identifier(string)
                },
                _ => TokenType::Identifier(string)
            };

            tokens.push(Token::new(token_type, chars.loc()));
        }
        else if *char == '\'' || *char == '"' {
            let quote = chars.next().unwrap();
            let string = String::from_iter(chars.peek_and_take_while(|next| *next != quote));

            if let Some(closing_quote) = chars.next() {
                if closing_quote != quote {
                    return Err(LexerError::UnexpectedCharacter(closing_quote));
                }
            }
            else {
                return Err(LexerError::UnexpectedEndOfInput);
            }

            tokens.push(Token::new(TokenType::String(string), chars.loc()));
        }
        else {
            let next = chars.next().unwrap();

            let token_type = match next {
                '(' => TokenType::LeftParenthesis,
                ')' => TokenType::RightParenthesis,
                '{' => TokenType::LeftCurlyBrace,
                '}' => TokenType::RightCurlyBrace,
                ':' => TokenType::Colon,
                ';' => TokenType::Semicolon,
                ',' => TokenType::Comma,
                '.' => TokenType::Period,
                '>' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenType::GreaterThanOrEqual
                    },
                    _ => TokenType::GreaterThan
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenType::LessThanOrEqual
                    },
                    _ => TokenType::LessThan
                },
                '=' => TokenType::Equal,
                '!' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        TokenType::NotEqual
                    },
                    _ => return Err(LexerError::UnexpectedCharacter(next))
                },
                '*' => TokenType::Asterisk,
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '/' => TokenType::Slash,
                '%' => TokenType::Percent,
                '^' => TokenType::Caret,
                _ => return Err(LexerError::UnexpectedCharacter(next))
            };

            tokens.push(Token::new(token_type, chars.loc()));
        }
    }

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use crate::query::cursor::Location;

    use super::*;

    #[test]
    fn tokenize_put_statement() {
        let input = "PUT users { userId: 'abcd123', name: 'Alice', age: 30, isRegistered: true }";

        let expected_types = vec![
            TokenType::Keyword(Keyword::Put),
            TokenType::Identifier(String::from("users")),
            TokenType::LeftCurlyBrace,
            TokenType::Identifier(String::from("userId")),
            TokenType::Colon,
            TokenType::String(String::from("abcd123")),
            TokenType::Comma,
            TokenType::Identifier(String::from("name")),
            TokenType::Colon,
            TokenType::String(String::from("Alice")),
            TokenType::Comma,
            TokenType::Identifier(String::from("age")),
            TokenType::Colon,
            TokenType::Number(String::from("30")),
            TokenType::Comma,
            TokenType::Identifier(String::from("isRegistered")),
            TokenType::Colon,
            TokenType::Keyword(Keyword::True),
            TokenType::RightCurlyBrace,
        ];

        let expected: Vec<Token> = expected_types
            .iter()
            .zip(1..)
            .map(|(token_type, column)| Token::new(token_type.clone(), Location { line: 1, column }))
            .collect();

        assert_eq!(tokenize(input).unwrap(), expected);
    }
}
