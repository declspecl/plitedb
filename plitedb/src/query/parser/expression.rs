use crate::query::cursor::PeekingCursor;
use crate::query::lexer::token::{Keyword, Token};

use super::ast::{BinaryOperator, Comparison, Expression, HasPrecedence, MathematicalOperation, UnaryOperation, UnaryOperator, Value};
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

    return match token {
        Token::String(value) => Ok(Expression::Literal(Value::String(value))),
        Token::Keyword(Keyword::True) => Ok(Expression::Literal(Value::Boolean(true))),
        Token::Keyword(Keyword::False) => Ok(Expression::Literal(Value::Boolean(false))),
        Token::Number(number) => {
            if let Ok(int_value) = number.parse::<i64>() {
                Ok(Expression::Literal(Value::Integer(int_value)))
            }
            else if let Ok(float_value) = number.parse::<f64>() {
                Ok(Expression::Literal(Value::Float(float_value)))
            }
            else {
                Err(ParserError::InvalidValue)
            }
        },
        Token::Identifier(value) => Ok(Expression::Identifier(value)),
        Token::LeftParenthesis => {
            let expression = parse_expression(tokens, 0)?;

            match tokens.next() {
                Some(Token::RightParenthesis) => (),
                _ => return Err(ParserError::MissingParenthesis)
            }

            Ok(expression)
        },
        Token::Minus => {
            let expression = parse_primary(tokens)?;

            Ok(Expression::UnaryOperation(UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(expression)
            }))
        },
        _ => Err(ParserError::UnexpectedToken(token))
    };
}

fn parse_binary_operation<I: Iterator<Item = Token>>(
    tokens: &mut PeekingCursor<I>,
    left: Expression,
    operator: BinaryOperator
) -> ParserResult<Expression> {
    let precedence = operator.precedence();
    let right = parse_expression(tokens, precedence)?;

    return match operator {
        BinaryOperator::Mathematical(operator) => Ok(Expression::BinaryOperation(MathematicalOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        })),
        BinaryOperator::Comparison(operator) => Ok(Expression::Comparison(Comparison {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        }))
    };
}

#[cfg(test)]
mod tests {
    use crate::query::parser::ast::{MathematicalOperation, MathematicalOperator};

    use super::*;

    #[test]
    fn parses_basic_math_equation() {
        // 1 + 2 + 3
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::Plus,
            Token::Number("2".to_string()),
            Token::Plus,
            Token::Number("3".to_string()),
        ];

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(MathematicalOperation {
                left: Box::new(Expression::BinaryOperation(MathematicalOperation {
                    left: Box::new(Expression::Literal(Value::Integer(1))),
                    operator: MathematicalOperator::Add,
                    right: Box::new(Expression::Literal(Value::Integer(2)))
                })),
                operator: MathematicalOperator::Add,
                right: Box::new(Expression::Literal(Value::Integer(3)))
            })
        );
    }

    #[test]
    fn parses_basic_parenthetical_math_equation() {
        // 1 * (2 + 3)
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::Asterisk,
            Token::LeftParenthesis,
            Token::Number("2".to_string()),
            Token::Plus,
            Token::Number("3".to_string()),
            Token::RightParenthesis,
        ];

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(MathematicalOperation {
                left: Box::new(Expression::Literal(Value::Integer(1))),
                operator: MathematicalOperator::Multiply,
                right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                    left: Box::new(Expression::Literal(Value::Integer(2))),
                    operator: MathematicalOperator::Add,
                    right: Box::new(Expression::Literal(Value::Integer(3)))
                }))
            })
        );
    }

    #[test]
    fn parses_unary_expression() {
        // 1 + -1 - (1 * -5)
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::Plus,
            Token::Minus,
            Token::Number("1".to_string()),
            Token::Minus,
            Token::LeftParenthesis,
            Token::Number("1".to_string()),
            Token::Asterisk,
            Token::Minus,
            Token::Number("5".to_string()),
            Token::RightParenthesis,
        ];

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(MathematicalOperation {
                left: Box::new(Expression::Literal(Value::Integer(1))),
                operator: MathematicalOperator::Add,
                right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                    left: Box::new(Expression::UnaryOperation(UnaryOperation {
                        operator: UnaryOperator::Negate,
                        operand: Box::new(Expression::Literal(Value::Integer(1)))
                    })),
                    operator: MathematicalOperator::Subtract,
                    right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                        left: Box::new(Expression::Literal(Value::Integer(1))),
                        operator: MathematicalOperator::Multiply,
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
        let tokens = vec![
            Token::LeftParenthesis,
            Token::LeftParenthesis,
            Token::Number("5".to_string()),
            Token::Minus,
            Token::Number("2".to_string()),
            Token::RightParenthesis,
            Token::Slash,
            Token::Number("2".to_string()),
            Token::RightParenthesis,
            Token::Plus,
            Token::LeftParenthesis,
            Token::Number("2".to_string()),
            Token::Plus,
            Token::LeftParenthesis,
            Token::Number("9".to_string()),
            Token::Asterisk,
            Token::Number("4".to_string()),
            Token::Minus,
            Token::Number("2".to_string()),
            Token::RightParenthesis,
            Token::Slash,
            Token::Number("2".to_string()),
            Token::RightParenthesis,
        ];

        let mut cursor = PeekingCursor::new(tokens.into_iter());
        let ast = parse_expression(&mut cursor, 0).unwrap();

        assert_eq!(
            ast,
            Expression::BinaryOperation(MathematicalOperation {
                left: Box::new(Expression::BinaryOperation(MathematicalOperation {
                    left: Box::new(Expression::BinaryOperation(MathematicalOperation {
                        left: Box::new(Expression::Literal(Value::Integer(5))),
                        operator: MathematicalOperator::Subtract,
                        right: Box::new(Expression::Literal(Value::Integer(2)))
                    })),
                    operator: MathematicalOperator::Divide,
                    right: Box::new(Expression::Literal(Value::Integer(2)))
                })),
                operator: MathematicalOperator::Add,
                right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                    left: Box::new(Expression::Literal(Value::Integer(2))),
                    operator: MathematicalOperator::Add,
                    right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                        left: Box::new(Expression::BinaryOperation(MathematicalOperation {
                            left: Box::new(Expression::BinaryOperation(MathematicalOperation {
                                left: Box::new(Expression::Literal(Value::Integer(9))),
                                operator: MathematicalOperator::Multiply,
                                right: Box::new(Expression::Literal(Value::Integer(4)))
                            })),
                            operator: MathematicalOperator::Subtract,
                            right: Box::new(Expression::Literal(Value::Integer(2)))
                        })),
                        operator: MathematicalOperator::Divide,
                        right: Box::new(Expression::Literal(Value::Integer(2)))
                    }))
                }))
            })
        );
    }
}
