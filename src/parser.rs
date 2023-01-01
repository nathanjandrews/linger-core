use std::vec;

use crate::desugar::{desugar_statements, Procedure, Statements};
use crate::tokenizer::Operator::{self, *};
use crate::{
    error::{unexpected_token, LingerError, LingerError::ParseError, ParseError::*},
    tokenizer::{
        Token as T,
        TokenValue::{self, *},
    },
    KEYWORDS,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program<'a> {
    pub procedures: Vec<Procedure<'a>>,
    pub main: Statements<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SugaredProcedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: SugaredStatements<'a>,
}

pub type SugaredStatements<'a> = Vec<SugaredStatement<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SugaredStatement<'a> {
    Expr(SugaredExpr<'a>),
    Let(&'a str, SugaredExpr<'a>),
    Assign(&'a str, SugaredExpr<'a>),
    If(
        SugaredExpr<'a>,
        SugaredStatements<'a>,
        Vec<(SugaredExpr<'a>, SugaredStatements<'a>)>,
        Option<SugaredStatements<'a>>,
    ),
    While(SugaredExpr<'a>, SugaredStatements<'a>),
    Break,
    Return(Option<SugaredExpr<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SugaredExpr<'a> {
    Num(i64),
    Bool(bool),
    Str(String),
    Var(&'a str),
    Binary(Operator, Box<SugaredExpr<'a>>, Box<SugaredExpr<'a>>),
    Unary(Operator, Box<SugaredExpr<'a>>),
    PrimitiveCall(Builtin, Vec<SugaredExpr<'a>>),
    Call(Box<SugaredExpr<'a>>, Vec<SugaredExpr<'a>>),
    Lambda(Vec<&'a str>, SugaredStatements<'a>),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Builtin {
    Print,
}

pub fn check_builtin(s: &str) -> Option<Builtin> {
    match s {
        "print" => Some(Builtin::Print),
        _ => None,
    }
}

fn consume_token<'a>(
    target: TokenValue<'a>,
    tokens: &'a [T<'a>],
) -> Result<&'a [T<'a>], LingerError<'a>> {
    match tokens {
        [token, rest @ ..] if token.0.eq(&target) => Ok(rest),
        [token, ..] => Err(ParseError(Expected(target, token.clone()))),
        _ => unreachable!(),
    }
}

fn match_operator<'a>(
    operators: &[Operator],
    tokens: &'a [T<'a>],
) -> Option<(Operator, &'a [T<'a>])> {
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

type BinaryExpressionParser<'a> =
    fn(&'a [T<'a>]) -> Result<(SugaredExpr<'a>, &'a [T<'a>]), LingerError<'a>>;

fn parse_binary_expr<'a>(
    parse_expr: BinaryExpressionParser<'a>,
    operators: Vec<Operator>,
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr<'a>, &'a [T<'a>]), LingerError<'a>> {
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

fn binary_expression<'a>(
    op: Operator,
    first_arg: SugaredExpr<'a>,
    second_arg: SugaredExpr<'a>,
) -> SugaredExpr<'a> {
    SugaredExpr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

pub fn parse_program<'a>(tokens: &'a [T<'a>]) -> Result<Program<'a>, LingerError> {
    let (procedures, rest) = parse_procs(tokens)?;

    if !rest.is_empty() {
        return Err(unexpected_token(rest)); // extra tokens
    }

    let desugared_procs = procedures.iter().map(|proc| Procedure {
        name: proc.name,
        params: proc.params.clone(),
        body: desugar_statements(proc.body.clone()),
    });

    let (main_procs, procs): (Vec<Procedure>, Vec<Procedure>) = desugared_procs
        .into_iter()
        .partition(|proc| proc.name == "main");

    if main_procs.len() == 0 {
        return Err(ParseError(NoMain)); // no main procedure
    } else if main_procs.len() > 1 {
        return Err(ParseError(MultipleMain)); // more than one main procedure
    }

    let main_proc = main_procs.first().unwrap();

    return Ok(Program {
        procedures: procs,
        main: main_proc.body.to_vec(),
    });
}

fn parse_procs<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Vec<SugaredProcedure<'a>>, &'a [T<'a>]), LingerError> {
    let (proc_option, tokens) = parse_proc(tokens)?;

    match proc_option {
        Some(proc) => {
            let (mut rest_procs, tokens) = parse_procs(tokens)?;
            let mut vec = vec![proc];
            vec.append(&mut rest_procs);
            return Ok((vec, tokens));
        }
        None => Ok((vec![], tokens)),
    }
}

fn parse_proc<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Option<SugaredProcedure<'a>>, &[T<'a>]), LingerError> {
    match tokens {
        [T(ID("proc"), ..), T(ID(name), ..), T(LPAREN, ..), rest @ ..] => {
            if KEYWORDS.contains(name) {
                return Err(ParseError(KeywordAsProc(name)));
            }

            let (params, tokens) = parse_params(rest)?;

            let tokens = consume_token(LBRACKET, tokens)?;

            let (body_statements, tokens) = parse_statements(tokens)?;

            Ok((
                Some(SugaredProcedure {
                    name,
                    params,
                    body: body_statements,
                }),
                tokens,
            ))
        }
        _ => Ok((None, tokens)),
    }
}

fn parse_params<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<&'a str>, &[T<'a>]), LingerError> {
    match tokens {
        [T(RPAREN, ..), rest @ ..] => Ok((vec![], rest)),
        [T(ID(param_name), ..), rest_toks @ ..] => {
            if KEYWORDS.contains(param_name) {
                return Err(ParseError(KeywordAsParam(param_name)));
            }

            let (mut rest_params, rest_toks) = parse_params(rest_toks)?;
            let mut params = vec![*param_name];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_statements<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredStatements, &[T<'a>]), LingerError> {
    let (statement_option, tokens) = parse_statement(tokens)?;

    let statement = if statement_option.is_some() {
        statement_option.unwrap()
    } else {
        return Ok((vec![], tokens));
    };

    let (mut rest_statements, tokens) = parse_statements(tokens)?;
    let mut vec = vec![statement];
    vec.append(&mut rest_statements);
    Ok((vec, tokens))
}

fn parse_statement<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Option<SugaredStatement>, &[T<'a>]), LingerError> {
    match tokens {
        [T(RBRACKET, ..), tokens @ ..] => Ok((None, tokens)),
        [T(ID("let"), ..), T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            if KEYWORDS.contains(var_name) {
                return Err(ParseError(KeywordAsVar(var_name)));
            }

            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Let(&var_name, var_expr)), tokens))
        }
        [T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            if KEYWORDS.contains(var_name) {
                return Err(ParseError(KeywordAsVar(var_name)));
            }

            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Assign(&var_name, var_expr)), tokens))
        }
        [T(ID("if"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (cond_expr, tokens) = parse_expr(tokens)?;
            let (then_statements, mut tokens) = match tokens {
                [T(RPAREN, ..), T(LBRACKET, ..), tokens @ ..] => parse_statements(tokens)?,
                [T(RPAREN, ..), token, ..] => {
                    return Err(ParseError(Expected(LBRACKET, token.clone())))
                }
                [token, ..] => return Err(ParseError(Expected(RPAREN, token.clone()))),
                _ => return Err(ParseError(ExpectedSomething)),
            };

            let mut else_ifs = vec![];

            loop {
                match tokens {
                    [T(ID("else"), ..), T(ID("if"), ..), T(LPAREN, ..), rest @ ..] => {
                        let (else_if_cond, rest) = parse_expr(rest)?;
                        let (else_if_statements, rest) = match rest {
                            [T(RPAREN, ..), T(LBRACKET, ..), rest @ ..] => parse_statements(rest)?,
                            [T(RPAREN, ..), token, ..] => {
                                return Err(ParseError(Expected(LBRACKET, token.clone())))
                            }
                            [token, ..] => return Err(ParseError(Expected(RPAREN, token.clone()))),
                            _ => return Err(ParseError(ExpectedSomething)),
                        };
                        else_ifs.push((else_if_cond, else_if_statements));
                        tokens = rest;
                    }
                    _ => break,
                }
            }

            let (else_statements_option, tokens) = match tokens {
                [T(ID("else"), ..), T(LBRACKET, ..), tokens @ ..] => match parse_statements(tokens)
                {
                    Ok((statements, tokens)) => (Some(statements), tokens),
                    Err(e) => return Err(e),
                },
                [T(ID("else"), ..), token, ..] => {
                    return Err(ParseError(Expected(LBRACKET, token.clone())))
                }
                tokens => (None, tokens),
            };

            Ok((
                Some(SugaredStatement::If(
                    cond_expr,
                    then_statements,
                    else_ifs,
                    else_statements_option,
                )),
                tokens,
            ))
        }
        [T(ID("while"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (while_cond_expr, tokens) = parse_expr(tokens)?;
            let (while_body, tokens) = match tokens {
                [T(RPAREN, ..), T(LBRACKET, ..), tokens @ ..] => parse_statements(tokens)?,
                [T(RPAREN, ..), token, ..] => {
                    return Err(ParseError(Expected(LBRACKET, token.clone())))
                }
                [token, ..] => return Err(ParseError(Expected(RPAREN, token.clone()))),
                _ => return Err(ParseError(ExpectedSomething)),
            };

            Ok((
                Some(SugaredStatement::While(while_cond_expr, while_body)),
                tokens,
            ))
        }
        [T(ID("return"), ..), T(SEMICOLON, ..), tokens @ ..] => {
            Ok((Some(SugaredStatement::Return(None)), tokens))
        }
        [T(ID("return"), ..), tokens @ ..] => {
            let (return_expr, tokens) = parse_expr(tokens)?;

            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Return(Some(return_expr))), tokens))
        }
        [T(ID("break"), ..), tokens @ ..] => {
            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(SugaredStatement::Break), tokens))
        }
        tokens => match parse_expr(tokens) {
            Ok((expr, tokens)) => {
                let tokens = consume_token(SEMICOLON, tokens)?;
                Ok((Some(SugaredStatement::Expr(expr)), tokens))
            }
            Err(e) => return Err(e),
        },
    }
}

fn parse_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    parse_logical_or_expr(tokens)
}

fn parse_logical_or_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_logical_and_expr, vec![LogicOr], tokens);
}

fn parse_logical_and_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_equality_expr, vec![LogicAnd], tokens);
}

fn parse_equality_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_relational_expr, vec![Eq, Ne], tokens);
}

fn parse_relational_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_additive_expr, vec![LT, GT, LTE, GTE], tokens);
}

fn parse_additive_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_multiplicative_expr, vec![Plus, Minus], tokens);
}

fn parse_multiplicative_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_unary_expr, vec![Times, Mod, Div], tokens);
}

fn parse_unary_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    match match_operator(vec![Minus, LogicNot].as_slice(), tokens) {
        Some((op, tokens)) => {
            let (operand, tokens) = parse_terminal_expr(tokens)?;
            return Ok((SugaredExpr::Unary(op, Box::new(operand)), tokens));
        }
        None => return parse_terminal_expr(tokens),
    }
}

fn parse_terminal_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(ID("lam"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (params, tokens) = parse_params(tokens)?;
            let tokens = consume_token(THIN_ARROW, tokens)?;
            let tokens = consume_token(LBRACKET, tokens)?;
            let (lambda_body, tokens) = parse_statements(tokens)?;
            return Ok((SugaredExpr::Lambda(params, lambda_body), tokens));
        }
        [T(ID(proc_name), ..), T(LPAREN, ..), tokens @ ..] => {
            let (args, tokens) = parse_args(tokens)?;

            let expr = match check_builtin(proc_name) {
                Some(builtin) => SugaredExpr::PrimitiveCall(builtin, args),
                None => SugaredExpr::Call(Box::new(SugaredExpr::Var(proc_name)), args),
            };

            return Ok((expr, tokens));
        }
        [T(STR(s), ..), tokens @ ..] => Ok((SugaredExpr::Str(s.to_string()), tokens)),
        [T(ID(id), ..), tokens @ ..] => match *id {
            "true" => Ok((SugaredExpr::Bool(true), tokens)),
            "false" => Ok((SugaredExpr::Bool(false), tokens)),
            _ => {
                if KEYWORDS.contains(id) {
                    Err(LingerError::ParseError(KeywordAsVar(id)))
                } else {
                    Ok((SugaredExpr::Var(id), tokens))
                }
            }
        },
        [T(LPAREN, ..), tokens @ ..] => {
            let (expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            match expr {
                SugaredExpr::Lambda(params, body) => {
                    // expect an immediately invoked function, parse the arguments and return the call
                    let tokens = consume_token(LPAREN, tokens)?;
                    let (args, tokens) = parse_args(tokens)?;
                    Ok((
                        SugaredExpr::Call(Box::new(SugaredExpr::Lambda(params, body)), args),
                        tokens,
                    ))
                }
                expr => Ok((expr, tokens)),
            }
        }
        [T(NUM(n), ..), tokens @ ..] => Ok((SugaredExpr::Num(*n), tokens)),
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<SugaredExpr>, &'a [T<'a>]), LingerError> {
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

fn parse_rest_args<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Vec<SugaredExpr>, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        [T(COMMA, ..), T(RPAREN, ..), ..] => Err(unexpected_token(tokens)),
        [T(COMMA, ..), tokens @ ..] => parse_args(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}
