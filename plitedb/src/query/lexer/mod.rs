pub mod error;
pub mod token;

use super::cursor::PeekingCursor;

use error::{LexerError, LexerResult};
use token::{Keyword, Token};

pub fn tokenize(haystack: &str) -> LexerResult<Vec<Token>> {
    let mut tokens = Vec::with_capacity(32);

    let mut chars = PeekingCursor::new(haystack.chars());
    while let Some(char) = chars.peek() {
        if char.is_whitespace() {
            chars.next();
            continue;
        };

        if char.is_numeric() {
            tokens.push(Token::Number(String::from_iter(
                chars.peek_and_take_while(|next| next.is_numeric() || *next == '.')
            )));
        }
        else if char.is_alphabetic() || *char == '_' {
            let string = String::from_iter(chars.peek_and_take_while(|next| next.is_alphanumeric() || *next == '_'));
            let token = match string.len() {
                3 => match &string[..] {
                    "GET" => Token::Keyword(Keyword::Get),
                    "PUT" => Token::Keyword(Keyword::Put),
                    _ => Token::Identifier(string)
                },
                4 => match &string[..] {
                    "true" => Token::Keyword(Keyword::True),
                    "false" => Token::Keyword(Keyword::False),
                    _ => Token::Identifier(string)
                },
                5 => match &string[..] {
                    "WHERE" => Token::Keyword(Keyword::Where),
                    _ => Token::Identifier(string)
                },
                _ => Token::Identifier(string)
            };

            tokens.push(token);
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
                return Err(LexerError::UnexpectedEndOfInput)
            }

            tokens.push(Token::String(string));
        }
        else {
            let next = chars.next().unwrap();

            match next {
                '(' => tokens.push(Token::LeftParenthesis),
                ')' => tokens.push(Token::RightParenthesis),
                ':' => tokens.push(Token::Colon),
                ';' => tokens.push(Token::Semicolon),
                ',' => tokens.push(Token::Comma),
                '.' => tokens.push(Token::Period),
                '>' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::GreaterThanOrEqual);
                    },
                    _ => tokens.push(Token::GreaterThan)
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::LessThanOrEqual);
                    },
                    _ => tokens.push(Token::LessThan)
                },
                '=' => tokens.push(Token::Equals),
                '!' => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::NotEquals);
                    },
                    _ => return Err(LexerError::UnexpectedCharacter(next))
                },
                '*' => tokens.push(Token::Asterisk),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '/' => tokens.push(Token::Slash),
                '^' => tokens.push(Token::Caret),
                _ => return Err(LexerError::UnexpectedCharacter(next))
            }
        }
    }

    return Ok(tokens);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_put_statement() {
        let input = "PUT users (userId: 'abcd123', name: 'Alice', age: 30, isRegistered: true)";

        let expected = vec![
            Token::Keyword(Keyword::Put),
            Token::Identifier(String::from("users")),
            Token::LeftParenthesis,
            Token::Identifier(String::from("userId")),
            Token::Colon,
            Token::String(String::from("abcd123")),
            Token::Comma,
            Token::Identifier(String::from("name")),
            Token::Colon,
            Token::String(String::from("Alice")),
            Token::Comma,
            Token::Identifier(String::from("age")),
            Token::Colon,
            Token::Number(String::from("30")),
            Token::Comma,
            Token::Identifier(String::from("isRegistered")),
            Token::Colon,
            Token::Keyword(Keyword::True),
            Token::RightParenthesis
        ];

        assert_eq!(tokenize(input).unwrap(), expected);
    }
}
