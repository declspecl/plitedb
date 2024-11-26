use ast::{BinaryOperator, Comparison, Expression, HasPrecedence, MathematicalOperation, Value};
use error::{ParserError, ParserResult};

use super::{
    cursor::PeekingCursor,
    lexer::token::{Keyword, Token}
};

pub mod ast;
pub mod error;

pub fn parse<I: IntoIterator<Item = Token>>(tokens: I) -> ParserResult<Expression> {
    let mut tokens = PeekingCursor::new(tokens.into_iter());

    return parse_expression(&mut tokens, 0);
}

fn parse_expression<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>, precedence: u8) -> ParserResult<Expression> {
    let mut left = parse_primary(tokens)?;


    loop {
        let binary_operator = match tokens.peek() {
            Some(token) => Some(BinaryOperator::try_from(token)?),
            _ => None
        };

        if binary_operator.is_none() || precedence >= binary_operator.unwrap().precedence() {
            break;
        }

        tokens.next();

        // COME BACK TO
        if let Some(Token::RightParenthesis) = tokens.peek() {
            tokens.next();
            break;
        }

        left = parse_binary_operation(tokens, left, binary_operator.unwrap())?;
    }


    return Ok(left);
}

fn parse_primary<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>) -> ParserResult<Expression> {
    let token = tokens.next().ok_or(ParserError::UnexpectedEndOfInput)?;

    match token {
        // literal
        Token::Keyword(Keyword::True) => Ok(Expression::Literal(Value::Boolean(true))),
        Token::Keyword(Keyword::False) => Ok(Expression::Literal(Value::Boolean(false))),
        Token::Number(number) => match number.parse::<i64>() {
            Ok(value) => Ok(Expression::Literal(Value::Integer(value))),
            Err(_) => match number.parse::<f64>() {
                Ok(value) => Ok(Expression::Literal(Value::Float(value))),
                Err(_) => Err(ParserError::InvalidValue)
            }
        },
        Token::String(value) => Ok(Expression::Literal(Value::String(value))),

        // identifier
        Token::Identifier(value) => Ok(Expression::Identifier(value)),

        // unary operation
        Token::LeftParenthesis => {
            let expression = parse_expression(tokens, 0)?;

            match tokens.next() {
                Some(Token::RightParenthesis) => (),
                _ => return Err(ParserError::MissingParenthesis)
            }

            Ok(expression)
        },
        _ => Err(ParserError::UnexpectedToken(token))
    }
}

fn parse_binary_operation<I: Iterator<Item = Token>>(tokens: &mut PeekingCursor<I>, left: Expression, operator: BinaryOperator) -> ParserResult<Expression> {
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
    use ast::MathematicalOperator;

    use super::*;

    #[test]
    fn parses_basic_math_equation() {
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::Plus,
            Token::Number("2".to_string()),
            Token::Plus,
            Token::Number("3".to_string())
        ];

        let ast = parse(tokens).unwrap();

        assert_eq!(ast, Expression::BinaryOperation(MathematicalOperation {
            left: Box::new(Expression::Literal(Value::Integer(1))),
            operator: MathematicalOperator::Add,
            right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                left: Box::new(Expression::Literal(Value::Integer(2))),
                operator: MathematicalOperator::Add,
                right: Box::new(Expression::Literal(Value::Integer(3)))
            }))
        }));
    }

    #[test]
    fn parses_basic_parenthetical_math_equation() {
        let tokens = vec![
            Token::Number("1".to_string()),
            Token::Plus,
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::Asterisk,
            Token::Number("3".to_string()),
            Token::Number("2".to_string())
        ];

        let ast = parse(tokens).unwrap();

        assert_eq!(ast, Expression::BinaryOperation(MathematicalOperation {
            left: Box::new(Expression::Literal(Value::Integer(1))),
            operator: MathematicalOperator::Add,
            right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                left: Box::new(Expression::Literal(Value::Integer(0))),
                operator: MathematicalOperator::Multiply,
                right: Box::new(Expression::BinaryOperation(MathematicalOperation {
                    left: Box::new(Expression::Literal(Value::Integer(3))),
                    operator: MathematicalOperator::Multiply,
                    right: Box::new(Expression::Literal(Value::Integer(2)))
                }))
            }))
        }));
    }
}