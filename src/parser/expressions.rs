use crate::tokenizer::Operator::*;
use crate::{
    error::ParseError::{self, *},
    tokenizer::{Keyword::*, Token as T, TokenValue::*},
};

use super::utils::{
    check_builtin, consume_token, match_operator, parse_binary_expr, unexpected_token,
};
use super::procedures::parse_params;
use super::statements::parse_statement;
use super::SugaredExpr;

pub fn parse_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    parse_logical_or_expr(tokens)
}

pub fn parse_logical_or_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    return parse_binary_expr(parse_logical_and_expr, vec![LogicOr], tokens);
}

pub fn parse_logical_and_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    return parse_binary_expr(parse_equality_expr, vec![LogicAnd], tokens);
}

pub fn parse_equality_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    return parse_binary_expr(parse_relational_expr, vec![Eq, Ne], tokens);
}

pub fn parse_relational_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    return parse_binary_expr(parse_additive_expr, vec![LT, GT, LTE, GTE], tokens);
}

pub fn parse_additive_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    return parse_binary_expr(parse_multiplicative_expr, vec![Plus, Minus], tokens);
}

pub fn parse_multiplicative_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    return parse_binary_expr(parse_unary_expr, vec![Times, Mod, Div], tokens);
}

pub fn parse_unary_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    match match_operator(vec![Minus, LogicNot].as_slice(), tokens) {
        Some((operator, tokens)) => {
            let (right, tokens) = parse_unary_expr(tokens)?;
            return Ok((SugaredExpr::Unary(operator, Box::new(right)), tokens));
        }
        None => {
            let (increment_op_option, tokens) = match tokens {
                [T(DOUBLE_PLUS, ..), tokens @ ..] => (Some(PreIncrement), tokens),
                [T(DOUBLE_MINUS, ..), tokens @ ..] => (Some(PreDecrement), tokens),
                tokens => (None, tokens),
            };
            let (terminal_expr, tokens) = parse_call_expr(tokens)?;
            match increment_op_option {
                Some(op) => return Ok((SugaredExpr::Unary(op, Box::new(terminal_expr)), tokens)),
                None => match tokens {
                    [T(DOUBLE_PLUS, ..), tokens @ ..] => {
                        return Ok((
                            SugaredExpr::Unary(PostIncrement, Box::new(terminal_expr)),
                            tokens,
                        ))
                    }
                    [T(DOUBLE_MINUS, ..), tokens @ ..] => {
                        return Ok((
                            SugaredExpr::Unary(PostDecrement, Box::new(terminal_expr)),
                            tokens,
                        ))
                    }
                    tokens => return Ok((terminal_expr, tokens)),
                },
            }
        }
    }
}

pub fn parse_call_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    let (mut expr, mut tokens) = parse_terminal_expr(tokens)?;
    loop {
        (expr, tokens) = match tokens {
            [T(LPAREN, ..), rest @ ..] => {
                let (args, rest) = parse_args(rest)?;
                let call_expr = match check_builtin(&expr) {
                    Some(builtin) => SugaredExpr::PrimitiveCall(builtin, args),
                    None => SugaredExpr::Call(Box::new(expr), args),
                };
                (call_expr, rest)
            }
            _ => break,
        }
    }
    return Ok((expr, tokens));
}

pub fn parse_terminal_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), ParseError> {
    match tokens {
        [T(STR(s), ..), tokens @ ..] => Ok((SugaredExpr::Str(s.to_string()), tokens)),
        [T(KW(True), ..), tokens @ ..] => Ok((SugaredExpr::Bool(true), tokens)),
        [T(KW(False), ..), tokens @ ..] => Ok((SugaredExpr::Bool(false), tokens)),
        [T(KW(kw), ..), ..] => Err(KeywordAsVar(kw.to_string())),
        [T(ID(id), ..), tokens @ ..] => Ok((SugaredExpr::Var(id.to_string()), tokens)),
        [T(LPAREN, ..), tokens @ ..] => match parse_params(tokens) {
            // if the next sequence of tokens is a params list, then parse a lambda expression
            Ok((params, tokens)) => {
                let tokens = consume_token(THIN_ARROW, tokens)?;
                let (lambda_body, tokens) = match parse_statement(tokens, false)? {
                    (Some(statement), tokens) => (statement, tokens),
                    _ => return Err(ExpectedStatement),
                };
                return Ok((SugaredExpr::Lambda(params, Box::new(lambda_body)), tokens));
            }
            // if the next sequence of tokens is a valid sequence of tokens, but not a params list,
            // then parse a parenthesized expression
            Err(UnexpectedToken(_)) => {
                let (expr, tokens) = parse_expr(tokens)?;
                let tokens = consume_token(RPAREN, tokens)?;
                return Ok((expr, tokens));
            }
            // if the next sequence of tokens is not a valid sequence of tokens, return the error
            Err(e) => return Err(e),
        },

        [T(NUM(n), ..), tokens @ ..] => Ok((SugaredExpr::Num(*n), tokens)),
        tokens => Err(unexpected_token(tokens)),
    }
}

pub fn parse_args(tokens: &[T]) -> Result<(Vec<SugaredExpr>, &[T]), ParseError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        tokens => {
            let (expr, tokens) = parse_expr(tokens)?;
            let (mut rest_args, tokens) = parse_rest_args(tokens)?;

            let mut vec = vec![expr];
            vec.append(&mut rest_args);
            return Ok((vec, tokens));
        }
    }
}

pub fn parse_rest_args(tokens: &[T]) -> Result<(Vec<SugaredExpr>, &[T]), ParseError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        [T(COMMA, ..), T(RPAREN, ..), ..] => Err(unexpected_token(tokens)),
        [T(COMMA, ..), tokens @ ..] => parse_args(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}
