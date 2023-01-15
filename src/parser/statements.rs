use crate::{
    error::ParseError::{self, *},
    tokenizer::{Keyword::*, Token as T, TokenValue::*},
};

use super::{
    expressions::parse_expr,
    utils::{
        conditionally_consume_semicolon, consume_token, ensure_block, is_assignment,
        is_assignment_or_initialization,
    },
    SugaredStatement,
};

pub fn parse_statements(tokens: &[T]) -> Result<(Vec<SugaredStatement>, &[T]), ParseError> {
    let (statement_option, tokens) = parse_statement(tokens, true)?;

    let statement = match statement_option {
        Some(statement) => statement,
        None => return Ok((vec![], tokens)),
    };

    let (mut rest_statements, tokens) = parse_statements(tokens)?;
    let mut vec = vec![statement];
    vec.append(&mut rest_statements);
    Ok((vec, tokens))
}

pub fn parse_statement(
    tokens: &[T],
    parse_semicolon: bool,
) -> Result<(Option<SugaredStatement>, &[T]), ParseError> {
    match tokens {
        [T(R_CURLY_BRACKET, ..), tokens @ ..] => Ok((None, tokens)),
        [T(KW(Let), ..), T(KW(kw), ..), ..] => Err(KeywordAsVar(kw.to_string())),
        [T(KW(Const), ..), T(KW(kw), ..), ..] => Err(KeywordAsVar(kw.to_string())),
        [T(KW(Let), ..), T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((
                Some(SugaredStatement::Let(var_name.to_string(), var_expr)),
                tokens,
            ))
        }
        [T(KW(Const), ..), T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((
                Some(SugaredStatement::Const(var_name.to_string(), var_expr)),
                tokens,
            ))
        }
        [T(KW(kw), ..), T(ASSIGN, ..), ..] => Err(KeywordAsVar(kw.to_string())),
        [T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((
                Some(SugaredStatement::Assign(var_name.to_string(), var_expr)),
                tokens,
            ))
        }
        [T(ID(var_name), ..), T(ASSIGN_OP(assign_op), ..), tokens @ ..] => {
            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((
                Some(SugaredStatement::OperatorAssignment(
                    *assign_op,
                    var_name.to_string(),
                    var_expr,
                )),
                tokens,
            ))
        }
        [T(KW(If), ..), T(LPAREN, ..), tokens @ ..] => {
            let (cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            let (then_block_option, mut tokens) = parse_statement(tokens, true)?;
            let then_block = ensure_block(then_block_option)?;

            let mut else_ifs = vec![];
            loop {
                match tokens {
                    [T(KW(Else), ..), T(KW(If), ..), T(LPAREN, ..), rest @ ..] => {
                        let (else_if_cond, rest) = parse_expr(rest)?;
                        let rest = consume_token(RPAREN, rest)?;
                        let (else_if_block_option, rest) = parse_statement(rest, true)?;
                        let else_if_block = ensure_block(else_if_block_option)?;
                        else_ifs.push((else_if_cond, else_if_block));
                        tokens = rest;
                    }
                    _ => break,
                }
            }

            let (else_block_option, tokens) = match tokens {
                [T(KW(Else), ..), tokens @ ..] => {
                    let (else_block, tokens) = parse_statement(tokens, true)?;
                    let else_block = ensure_block(else_block)?;
                    (Some(Box::new(else_block)), tokens)
                }
                tokens => (None, tokens),
            };

            Ok((
                Some(SugaredStatement::If(
                    cond_expr,
                    Box::new(then_block),
                    else_ifs,
                    else_block_option,
                )),
                tokens,
            ))
        }
        [T(KW(While), ..), T(LPAREN, ..), tokens @ ..] => {
            let (while_cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            let (while_block_option, tokens) = parse_statement(tokens, true)?;
            let while_block = ensure_block(while_block_option)?;

            Ok((
                Some(SugaredStatement::While(
                    while_cond_expr,
                    Box::new(while_block),
                )),
                tokens,
            ))
        }
        [T(KW(For), ..), T(LPAREN, ..), tokens @ ..] => {
            let (var_statement_option, tokens) = parse_statement(tokens, true)?;
            let var_statement = match var_statement_option {
                Some(statement) => {
                    if is_assignment_or_initialization(&statement) {
                        statement
                    } else {
                        return Err(ExpectedAssignmentOrInitialization);
                    }
                }
                None => return Err(ExpectedStatement),
            };

            let (stop_cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(SEMICOLON, tokens)?;

            let (reassign_statement_option, tokens) = parse_statement(tokens, false)?;
            let reassign_statement = match reassign_statement_option {
                Some(statement) => {
                    if is_assignment(&statement) {
                        statement
                    } else {
                        return Err(ExpectedAssignment);
                    }
                }
                None => return Err(ExpectedStatement),
            };
            let tokens = consume_token(RPAREN, tokens)?;

            let (for_block_option, tokens) = parse_statement(tokens, true)?;
            let for_block_statements = match for_block_option {
                Some(statement) => match statement {
                    SugaredStatement::Block(statements) => statements,
                    _ => return Err(ExpectedBlock),
                },
                None => return Err(ExpectedBlock),
            };

            return Ok((
                Some(SugaredStatement::For(
                    Box::new(var_statement),
                    stop_cond_expr,
                    Box::new(reassign_statement),
                    for_block_statements,
                )),
                tokens,
            ));
        }
        [T(KW(Return), ..), T(SEMICOLON, ..), tokens @ ..] => {
            Ok((Some(SugaredStatement::Return(None)), tokens))
        }
        [T(KW(Return), ..), tokens @ ..] => {
            let (return_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Return(Some(return_expr))), tokens))
        }
        [T(KW(Break), ..), tokens @ ..] => {
            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Break), tokens))
        }
        [T(KW(Continue), ..), tokens @ ..] => {
            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Continue), tokens))
        }
        [T(L_CURLY_BRACKET, ..), tokens @ ..] => {
            let (statements, tokens) = parse_statements(tokens)?;
            Ok((Some(SugaredStatement::Block(statements)), tokens))
        }
        tokens => match parse_expr(tokens)? {
            (expr, tokens) => {
                let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;
                Ok((Some(SugaredStatement::Expr(expr)), tokens))
            }
        },
    }
}
