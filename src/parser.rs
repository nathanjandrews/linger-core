use crate::{
    error::{unexpected_token, LingerError, LingerError::ParseError, ParseError::*},
    tokenizer::{
        BinaryOperator::{self, *},
        Token as T,
        TokenValue::{self, *},
    },
    KEYWORDS,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Program<'a> {
    pub procedures: Vec<Procedure<'a>>,
    pub main: Statements<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Procedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Statements<'a>,
}

pub type Statements<'a> = Vec<Statement<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Let(&'a str, Expr<'a>),
    If(Expr<'a>, Statements<'a>, Option<Statements<'a>>),
    Return(Option<Expr<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<'a> {
    Num(i64),
    Bool(bool),
    Var(&'a str),
    Binary(BinaryOperator, Box<Expr<'a>>, Box<Expr<'a>>),
    PrimitiveCall(Builtin, Vec<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),
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
        [token, ..] => Err(ParseError(Expected(target, *token))),
        _ => unreachable!(),
    }
}

fn match_binary_operator<'a>(
    operators: Vec<BinaryOperator>,
    tokens: &'a [T<'a>],
) -> Option<(BinaryOperator, &'a [T<'a>])> {
    match tokens {
        [T(value, ..), rest @ ..] => match value {
            BIN_OP(b) => {
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

fn binary_expression<'a>(
    op: BinaryOperator,
    first_arg: Expr<'a>,
    second_arg: Expr<'a>,
) -> Expr<'a> {
    Expr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

pub fn parse_program<'a>(tokens: &'a [T<'a>]) -> Result<Program<'a>, LingerError> {
    let (procedures, rest) = parse_procs(tokens)?;

    if !rest.is_empty() {
        return Err(unexpected_token(rest)); // extra tokens
    }

    let (main_procs, procs): (Vec<Procedure>, Vec<Procedure>) =
        procedures.into_iter().partition(|proc| proc.name == "main");

    if main_procs.len() == 0 {
        return Err(ParseError(NoMain)); // more than one main procedure
    }

    let main_proc = main_procs.first().unwrap();

    return Ok(Program {
        procedures: procs,
        main: main_proc.body.to_vec(),
    });
}

fn parse_procs<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Procedure<'a>>, &'a [T<'a>]), LingerError> {
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

fn parse_proc<'a>(tokens: &'a [T<'a>]) -> Result<(Option<Procedure<'a>>, &[T<'a>]), LingerError> {
    match tokens {
        [T(ID("proc"), ..), T(ID(name), ..), T(LPAREN, ..), rest @ ..] => {
            if KEYWORDS.contains(name) {
                return Err(ParseError(KeywordAsProc(name)));
            }

            let (params, tokens) = parse_params(rest)?;

            let tokens = consume_token(LBRACKET, tokens)?;

            let (body_statements, tokens) = parse_statements(tokens)?;

            Ok((
                Some(Procedure {
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

fn parse_statements<'a>(tokens: &'a [T<'a>]) -> Result<(Statements, &[T<'a>]), LingerError> {
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

fn parse_statement<'a>(tokens: &'a [T<'a>]) -> Result<(Option<Statement>, &[T<'a>]), LingerError> {
    match tokens {
        [T(RBRACKET, ..), tokens @ ..] => Ok((None, tokens)),
        [T(ID("let"), ..), T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            if KEYWORDS.contains(var_name) {
                return Err(ParseError(KeywordAsVar(var_name)));
            }

            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(Statement::Let(&var_name, var_expr)), tokens))
        }
        [T(ID("if"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (cond_expr, tokens) = parse_expr(tokens)?;
            let (then_statements, tokens) = match tokens {
                [T(RPAREN, ..), T(LBRACKET, ..), tokens @ ..] => parse_statements(tokens)?,
                [T(RPAREN, ..), token, ..] => return Err(ParseError(Expected(LBRACKET, *token))),
                [token, ..] => return Err(ParseError(Expected(RPAREN, *token))),
                _ => return Err(ParseError(ExpectedSomething)),
            };

            let (else_statements_option, tokens) = match tokens {
                [T(ID("else"), ..), T(LBRACKET, ..), tokens @ ..] => match parse_statements(tokens)
                {
                    Ok((statements, tokens)) => (Some(statements), tokens),
                    Err(e) => return Err(e),
                },
                [T(ID("else"), ..), token, ..] => {
                    return Err(ParseError(Expected(LBRACKET, *token)))
                }
                tokens => (None, tokens),
            };

            Ok((
                Some(Statement::If(
                    cond_expr,
                    then_statements,
                    else_statements_option,
                )),
                tokens,
            ))
        }
        [T(ID("return"), ..), T(SEMICOLON, ..), tokens @ ..] => {
            Ok((Some(Statement::Return(None)), tokens))
        }
        [T(ID("return"), ..), tokens @ ..] => {
            let (return_expr, tokens) = parse_expr(tokens)?;

            let tokens = consume_token(SEMICOLON, tokens)?;
            Ok((Some(Statement::Return(Some(return_expr))), tokens))
        }
        tokens => match parse_expr(tokens) {
            Ok((expr, tokens)) => {
                let tokens = consume_token(SEMICOLON, tokens)?;
                Ok((Some(Statement::Expr(expr)), tokens))
            }
            Err(e) => return Err(e),
        },
    }
}

fn parse_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    parse_logical_or_expr(tokens)
}

fn parse_logical_or_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (mut expr, mut tokens) = parse_logical_and_expr(tokens)?;
    loop {
        match match_binary_operator(vec![LogicOr], tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_logical_and_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

fn parse_logical_and_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (mut expr, mut tokens) = parse_equality_expr(tokens)?;
    loop {
        match match_binary_operator(vec![LogicAnd], tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_equality_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

fn parse_equality_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (mut expr, mut tokens) = parse_relational_expr(tokens)?;
    loop {
        match match_binary_operator(vec![Eq, Ne], tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_relational_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

fn parse_relational_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (mut expr, mut tokens) = parse_additive_expr(tokens)?;
    loop {
        match match_binary_operator(vec![LT, GT, LTE, GTE], tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_additive_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

fn parse_additive_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (mut expr, mut tokens) = parse_multiplicative_expr(tokens)?;
    loop {
        match match_binary_operator(vec![Plus, Minus], tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_multiplicative_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

fn parse_multiplicative_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (mut expr, mut tokens) = parse_terminal_expr(tokens)?;
    loop {
        match match_binary_operator(vec![Times, Mod, Div], tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_terminal_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

fn parse_terminal_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(ID(proc_name), ..), T(LPAREN, ..), tokens @ ..] => {
            let (args, tokens) = parse_args(tokens)?;

            let expr = match check_builtin(proc_name) {
                Some(builtin) => Expr::PrimitiveCall(builtin, args),
                None => Expr::Call(proc_name, args),
            };

            return Ok((expr, tokens));
        }
        [T(ID(id), ..), tokens @ ..] => match *id {
            "true" => Ok((Expr::Bool(true), tokens)),
            "false" => Ok((Expr::Bool(false), tokens)),
            _ => {
                if KEYWORDS.contains(id) {
                    Err(LingerError::ParseError(KeywordAsVar(id)))
                } else {
                    Ok((Expr::Var(id), tokens))
                }
            }
        },
        [T(LPAREN, ..), tokens @ ..] => {
            let (expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            Ok((expr, tokens))
        }
        [T(NUM(n), ..), tokens @ ..] => Ok((Expr::Num(*n), tokens)),
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Expr>, &'a [T<'a>]), LingerError> {
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

fn parse_rest_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Expr>, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        [T(COMMA, ..), T(RPAREN, ..), ..] => Err(unexpected_token(tokens)),
        [T(COMMA, ..), tokens @ ..] => parse_args(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}
