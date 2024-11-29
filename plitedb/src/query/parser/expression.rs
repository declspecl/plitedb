use crate::query::cursor::PeekingCursor;
use crate::query::lexer::token::{Keyword, Token, TokenType};

use super::ast::{BinaryOperation, BinaryOperator, Expression, UnaryOperation, UnaryOperator, Value};
use super::error::{ParserError, ParserResult};

pub fn parse_expression<I: Iterator<Item = Token>>(
    tokens: &mut PeekingCursor<I>,
    precedence: u8
) -> ParserResult<Expression> {
    let mut left = parse_primary(tokens)?;

    loop {
        let binary_operator = match tokens.peek() {
            Some(token) => match BinaryOperator::try_from(token) {
                Ok(operator) => operator,
                Err(_) => break
            },
            _ => break
        };

        if binary_operator.precedence() < precedence {
            break;
        }

        tokens.next();
        left = parse_binary_operation(tokens, left, binary_operator)?;
    }

    return Ok(left);
}

fn parse_primary<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Expression> {
    let token = tokens.next().ok_or(ParserError::UnexpectedEndOfInput)?;

    return match token.token_type {
        TokenType::String(value) => Ok(Expression::Literal(Value::String(value))),
        TokenType::Keyword(Keyword::True) => Ok(Expression::Literal(Value::Boolean(true))),
        TokenType::Keyword(Keyword::False) => Ok(Expression::Literal(Value::Boolean(false))),
        TokenType::Number(number) => {
            if let Ok(int_value) = number.parse::<i64>() {
                Ok(Expression::Literal(Value::Integer(int_value)))
            }
            else if let Ok(float_value) = number.parse::<f64>() {
                Ok(Expression::Literal(Value::Float(float_value)))
            }
            else {
                Err(ParserError::InvalidNumericalValue(number))
            }
        },
        TokenType::Identifier(value) => Ok(Expression::Identifier(value)),
        TokenType::LeftParenthesis => {
            let expression = parse_expression(tokens, 0)?;

            let next = tokens.next();
            match next {
                Some(token) => match token.token_type {
                    TokenType::RightParenthesis => (),
                    _ => return Err(ParserError::UnexpectedToken(token, ")".to_string()))
                },
                None => return Err(ParserError::UnexpectedEndOfInput)
            }

            Ok(expression)
        },
        TokenType::Minus => {
            let expression = parse_primary(tokens)?;

            Ok(Expression::UnaryOperation(UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(expression)
            }))
        },
        _ => Err(ParserError::UnexpectedToken(token, "value, keyword, or identifier".to_string()))
    };
}

fn parse_binary_operation<I: Iterator<Item = Token>>(
    tokens: &mut PeekingCursor<I>,
    left: Expression,
    operator: BinaryOperator
) -> ParserResult<Expression> {
    let precedence = operator.precedence();
    let right = parse_expression(tokens, precedence)?;

    return Ok(Expression::BinaryOperation(BinaryOperation {
        left: Box::new(left),
        operator,
        right: Box::new(right)
    }));
}

#[cfg(test)]
mod tests {
    use crate::query::{cursor::Location, lexer::token::TokenType};

    use super::*;

    #[test]
    fn parses_basic_math_equation() {
        // 1 + 2 + 3
        let token_types = vec![
            TokenType::Number("1".to_string()),
            TokenType::Plus,
            TokenType::Number("2".to_string()),
            TokenType::Plus,
            TokenType::Number("3".to_string()),
        ];

        let tokens: Vec<Token> = token_types
            .iter()
            .zip(1..)
            .map(|(token_type, column)| Token::new(token_type.clone(), Location { line: 1, column }))
            .collect();

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(BinaryOperation {
                left: Box::new(Expression::BinaryOperation(BinaryOperation {
                    left: Box::new(Expression::Literal(Value::Integer(1))),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Literal(Value::Integer(2)))
                })),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Literal(Value::Integer(3)))
            })
        );
    }

    #[test]
    fn parses_basic_parenthetical_math_equation() {
        // 1 * (2 + 3)
        let token_types = vec![
            TokenType::Number("1".to_string()),
            TokenType::Asterisk,
            TokenType::LeftParenthesis,
            TokenType::Number("2".to_string()),
            TokenType::Plus,
            TokenType::Number("3".to_string()),
            TokenType::RightParenthesis,
        ];

        let tokens: Vec<Token> = token_types
            .iter()
            .zip(1..)
            .map(|(token_type, column)| Token::new(token_type.clone(), Location { line: 1, column }))
            .collect();

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(BinaryOperation {
                left: Box::new(Expression::Literal(Value::Integer(1))),
                operator: BinaryOperator::Multiply,
                right: Box::new(Expression::BinaryOperation(BinaryOperation {
                    left: Box::new(Expression::Literal(Value::Integer(2))),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Literal(Value::Integer(3)))
                }))
            })
        );
    }

    #[test]
    fn parses_unary_expression() {
        // 1 + -1 - (1 * -5)
        let token_types = vec![
            TokenType::Number("1".to_string()),
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Number("1".to_string()),
            TokenType::Minus,
            TokenType::LeftParenthesis,
            TokenType::Number("1".to_string()),
            TokenType::Asterisk,
            TokenType::Minus,
            TokenType::Number("5".to_string()),
            TokenType::RightParenthesis,
        ];

        let tokens: Vec<Token> = token_types
            .iter()
            .zip(1..)
            .map(|(token_type, column)| Token::new(token_type.clone(), Location { line: 1, column }))
            .collect();

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(BinaryOperation {
                left: Box::new(Expression::Literal(Value::Integer(1))),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::BinaryOperation(BinaryOperation {
                    left: Box::new(Expression::UnaryOperation(UnaryOperation {
                        operator: UnaryOperator::Negate,
                        operand: Box::new(Expression::Literal(Value::Integer(1)))
                    })),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(Expression::BinaryOperation(BinaryOperation {
                        left: Box::new(Expression::Literal(Value::Integer(1))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(Expression::UnaryOperation(UnaryOperation {
                            operator: UnaryOperator::Negate,
                            operand: Box::new(Expression::Literal(Value::Integer(5)))
                        }))
                    }))
                }))
            })
        );
    }

    #[test]
    fn parses_complex_expression() {
        // ( (5 - 2) / 2) + ( 2 + ( 9 * 4 - 2 ) / 2 )
        let token_types = vec![
            TokenType::LeftParenthesis,
            TokenType::LeftParenthesis,
            TokenType::Number("5".to_string()),
            TokenType::Minus,
            TokenType::Number("2".to_string()),
            TokenType::RightParenthesis,
            TokenType::Slash,
            TokenType::Number("2".to_string()),
            TokenType::RightParenthesis,
            TokenType::Plus,
            TokenType::LeftParenthesis,
            TokenType::Number("2".to_string()),
            TokenType::Plus,
            TokenType::LeftParenthesis,
            TokenType::Number("9".to_string()),
            TokenType::Asterisk,
            TokenType::Number("4".to_string()),
            TokenType::Minus,
            TokenType::Number("2".to_string()),
            TokenType::RightParenthesis,
            TokenType::Slash,
            TokenType::Number("2".to_string()),
            TokenType::RightParenthesis,
        ];

        let tokens: Vec<Token> = token_types
            .iter()
            .zip(1..)
            .map(|(token_type, column)| Token::new(token_type.clone(), Location { line: 1, column }))
            .collect();

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(BinaryOperation {
                left: Box::new(Expression::BinaryOperation(BinaryOperation {
                    left: Box::new(Expression::BinaryOperation(BinaryOperation {
                        left: Box::new(Expression::Literal(Value::Integer(5))),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(Expression::Literal(Value::Integer(2)))
                    })),
                    operator: BinaryOperator::Divide,
                    right: Box::new(Expression::Literal(Value::Integer(2)))
                })),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::BinaryOperation(BinaryOperation {
                    left: Box::new(Expression::Literal(Value::Integer(2))),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::BinaryOperation(BinaryOperation {
                        left: Box::new(Expression::BinaryOperation(BinaryOperation {
                            left: Box::new(Expression::BinaryOperation(BinaryOperation {
                                left: Box::new(Expression::Literal(Value::Integer(9))),
                                operator: BinaryOperator::Multiply,
                                right: Box::new(Expression::Literal(Value::Integer(4)))
                            })),
                            operator: BinaryOperator::Subtract,
                            right: Box::new(Expression::Literal(Value::Integer(2)))
                        })),
                        operator: BinaryOperator::Divide,
                        right: Box::new(Expression::Literal(Value::Integer(2)))
                    }))
                }))
            })
        );
    }
}
