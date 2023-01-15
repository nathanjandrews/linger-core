use crate::tokenizer::Operator::{self, *};
use crate::{
    error::ParseError::{self, *},
    tokenizer::{
        Token as T,
        TokenValue::{self, *},
    },
};

use super::{Builtin, SugaredExpr, SugaredStatement};

/// A helper function to handle unexpected token patterns. This function returns an
/// [UnexpectedToken Error](UnexpectedToken), or an [Unexpected End-of-File](UnexpectedEOF) if
/// `tokens` is empty.
pub fn unexpected_token(tokens: &[T]) -> ParseError {
    match tokens {
        [unexpected_token, ..] => UnexpectedToken(unexpected_token.to_owned()),
        [] => UnexpectedEOF,
    }
}

/// A helper function to check if `s` matches one of the [Builtin] procedures.
pub fn check_builtin(expr: &SugaredExpr) -> Option<Builtin> {
    match expr {
        SugaredExpr::Var(name) => match name.as_str() {
            "print" => Some(Builtin::Print),
            "list" => Some(Builtin::List),
            _ => None,
        },
        _ => None,
    }
}

/// Tries to consume a token with a [TokenValue] of `target` from the front of `tokens`. On success,
/// this function returns `tokens` with the first element removed. On failure, this function returns
/// an [Expected] error.
pub fn consume_token(target: TokenValue, tokens: &[T]) -> Result<&[T], ParseError> {
    match tokens {
        [token, rest @ ..] if token.0.eq(&target) => Ok(rest),
        [token, ..] => Err(Expected(target, token.clone())),
        [] => Err(UnexpectedEOF),
    }
}

/// This function conditionally tries to consume a [SEMICOLON] token if `should_consume` is true.
/// If `should_consume` is true, then this function returns the result of [consume_token] with a
/// `target` of [SEMICOLON]. If `should_consume` is false, then this function returns the `tokens`
/// list unmodified.
pub fn conditionally_consume_semicolon(
    tokens: &[T],
    should_consume: bool,
) -> Result<&[T], ParseError> {
    if should_consume {
        return consume_token(SEMICOLON, tokens);
    } else {
        return Ok(tokens);
    }
}

/// This function tries to consume an [OP] token with an associated [Operator] found in `operators`.
/// If such a token is successfully consumed, this function returns the token's operator and the
/// list of tokens that comes after as a pair. If `tokens` does not start with such an operator,
/// then this function returns `None`.
pub fn match_operator<'a>(operators: &[Operator], tokens: &'a [T]) -> Option<(Operator, &'a [T])> {
    match tokens {
        [T(value, ..), rest @ ..] => match value {
            OP(b) => {
                if operators.contains(b) {
                    return Some((*b, rest));
                } else {
                    return None;
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// Type alias for the return value of a binary expression parsing function.
type BinaryExpressionParser = fn(&[T]) -> Result<(SugaredExpr, &[T]), ParseError>;

/// A helper function for parsing binary expressions.
pub fn parse_binary_expr(
    parse_expr: BinaryExpressionParser,
    operators: Vec<Operator>,
    tokens: &[T],
) -> Result<(SugaredExpr, &[T]), ParseError> {
    let (mut expr, mut tokens) = parse_expr(tokens)?;
    loop {
        match match_operator(operators.as_slice(), tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

/// A helper function for creating a [Binary Expression](SugaredExpr::Binary)
pub fn binary_expression(
    op: Operator,
    first_arg: SugaredExpr,
    second_arg: SugaredExpr,
) -> SugaredExpr {
    SugaredExpr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

/// Ensures that `statement_option` is a Some variant which contains a
/// [Block Statement](SugaredStatement::Block). Otherwise, this function returns
/// an [ExpectedBlock] parse error.
pub fn ensure_block(
    statement_option: Option<SugaredStatement>,
) -> Result<SugaredStatement, ParseError> {
    match statement_option {
        Some(statement) => match statement {
            SugaredStatement::Block(_) => Ok(statement),
            _ => Err(ExpectedBlock),
        },
        None => Err(ExpectedBlock),
    }
}

pub fn is_assignment(statement: &SugaredStatement) -> bool {
    match statement {
        SugaredStatement::Assign(_, _) => true,
        SugaredStatement::OperatorAssignment(_, _, _) => true,
        SugaredStatement::Expr(expr) => match expr {
            SugaredExpr::Unary(op, _) => match op {
                PreIncrement | PostIncrement | PreDecrement | PostDecrement => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

pub fn is_assignment_or_initialization(statement: &SugaredStatement) -> bool {
    match statement {
        SugaredStatement::Let(_, _) => true,
        statement => is_assignment(statement),
    }
}
