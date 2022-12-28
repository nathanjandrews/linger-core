use crate::{
    error::{unexpected_token, LingerError, LingerError::ParseError, ParseError::*},
    tokenizer::{
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
    Return(Expr<'a>),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Eq,
    LogicOr,
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

fn binary_expression<'a>(
    op: BinaryOperator,
    first_arg: Expr<'a>,
    second_arg: Expr<'a>,
) -> Expr<'a> {
    Expr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

pub fn parse_program<'a>(tokens: &'a [T<'a>]) -> Result<Program<'a>, LingerError> {
    let (procedures, rest) = match parse_procs(tokens) {
        Ok((procs, tokens)) => (procs, tokens),
        Err(e) => return Err(e),
    };

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
    let (proc_option, tokens) = match parse_proc(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    match proc_option {
        Some(proc) => {
            let (mut rest_procs, tokens) = match parse_procs(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
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

            let (params, tokens) = match parse_params(rest) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let tokens = match consume_token(LBRACKET, tokens) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            let (body_statements, tokens) = match parse_statements(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

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

            let (mut rest_params, rest_toks) = match parse_params(rest_toks) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let mut params = vec![*param_name];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_statements<'a>(tokens: &'a [T<'a>]) -> Result<(Statements, &[T<'a>]), LingerError> {
    let (statement_option, tokens) = match parse_statement(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    let statement = if statement_option.is_some() {
        statement_option.unwrap()
    } else {
        return Ok((vec![], tokens));
    };

    let (mut rest_statements, tokens) = match parse_statements(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };
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

            let (var_expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let tokens = match consume_token(SEMICOLON, tokens) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };
            Ok((Some(Statement::Let(&var_name, var_expr)), tokens))
        }
        [T(ID("if"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (cond_expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            // TODO: include more granular error handling for each of the RPAREN and LBRACKET tokens
            let (then_statements, tokens) = match tokens {
                [T(RPAREN, ..), T(LBRACKET, ..), tokens @ ..] => match parse_statements(tokens) {
                    Ok(pair) => pair,
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(ParseError(Custom(String::from(
                        "expected \")\", followed by \"{\"",
                    ))))
                }
            };

            // TODO: include more granular error handling for each of the ID("else") and LBRACKET tokens
            let (else_statements_option, tokens) = match tokens {
                [T(ID("else"), ..), T(LBRACKET, ..), tokens @ ..] => match parse_statements(tokens)
                {
                    Ok((statements, tokens)) => (Some(statements), tokens),
                    Err(e) => return Err(e),
                },
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
        [T(ID("return"), ..), tokens @ ..] => {
            let (return_expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let tokens = match consume_token(SEMICOLON, tokens) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };
            Ok((Some(Statement::Return(return_expr)), tokens))
        }
        tokens => match parse_expr(tokens) {
            Ok((expr, tokens)) => {
                let tokens = match consume_token(SEMICOLON, tokens) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
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
    let (relational_expr, tokens) = match parse_relational_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };
    match tokens {
        [T(LOGIC_OR, ..), tokens @ ..] => {
            let (logical_or_expr, tokens) = match parse_logical_or_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::LogicOr, relational_expr, logical_or_expr),
                tokens,
            ));
        }
        tokens => {
            return Ok((relational_expr, tokens));
        }
    }
}

fn parse_relational_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (additive_expr, tokens) = match parse_additive_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };
    match tokens {
        [T(EQ, ..), tokens @ ..] => {
            let (relational_expr, tokens) = match parse_relational_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Eq, additive_expr, relational_expr),
                tokens,
            ));
        }
        tokens => {
            return Ok((additive_expr, tokens));
        }
    }
}

fn parse_additive_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (terminal, tokens) = match parse_terminal_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    match tokens {
        [T(PLUS, ..), tokens @ ..] => {
            let (additive_expr, tokens) = match parse_additive_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Plus, terminal, additive_expr),
                tokens,
            ));
        }
        [T(MINUS, ..), tokens @ ..] => {
            let (additive_expr, tokens) = match parse_additive_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Minus, terminal, additive_expr),
                tokens,
            ));
        }
        tokens => {
            return Ok((terminal, tokens));
        }
    }
}

fn parse_terminal_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(ID(proc_name), ..), T(LPAREN, ..), tokens @ ..] => {
            let (args, tokens) = match parse_args(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

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
            let (expr, tokens) = match parse_logical_or_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let tokens = match consume_token(RPAREN, tokens) {
                Ok(tokens) => tokens,
                Err(e) => return Err(e),
            };
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
            let (expr, tokens) = match parse_logical_or_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let (mut rest_args, tokens) = match parse_rest_args(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

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
