use std::fmt;

use crate::tokenizer::Token as T;

#[derive(Debug)]
pub struct Program<'a> {
    pub procedures: Vec<Procedure<'a>>,
    pub main: Statements<'a>,
}

#[derive(Debug)]
pub struct Procedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Statements<'a>,
}

type Statements<'a> = Vec<Statement<'a>>;

#[derive(Clone, Debug)]
pub enum Statement<'a> {
    EXPR(Expr<'a>),
    LET(&'a str, Expr<'a>),
}

#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Num(i64),
    Bool(bool),
    Var(&'a str),
    Binary(BinaryOperator, Box<Expr<'a>>, Box<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Eq,
}

pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}

fn consume_token<'a>(target: T<'a>, tokens: &'a [T<'a>]) -> Result<&'a [T<'a>], ParseError> {
    match tokens {
        [token, rest @ ..] if token.eq(&target) => Ok(rest),
        _ => Err(ParseError),
    }
}

fn binary_expression<'a>(
    op: BinaryOperator,
    first_arg: Expr<'a>,
    second_arg: Expr<'a>,
) -> Expr<'a> {
    Expr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

pub fn parse_program<'a>(tokens: &'a [T<'a>]) -> Result<Program<'a>, ParseError> {
    let (procedures, rest) = match parse_procs(tokens) {
        Ok((procs, tokens)) => (procs, tokens.to_vec()),
        Err(e) => return Err(e),
    };
    if !rest.is_empty() {
        return Err(ParseError); // extra tokens
    }

    let (main_procs, procs): (Vec<Procedure>, Vec<Procedure>) =
        procedures.into_iter().partition(|proc| proc.name == "main");

    if main_procs.len() != 1 {
        return Err(ParseError); // more than one main function
    }

    let main_proc = main_procs.first().unwrap();

    return Ok(Program {
        procedures: procs,
        main: main_proc.body.to_vec(),
    });
}

fn parse_procs<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Procedure<'a>>, &'a [T<'a>]), ParseError> {
    let (proc_option, tokens) = match parse_proc(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    match (proc_option, tokens.len()) {
        (Some(proc), 0) => Ok((vec![proc], tokens)),
        (Some(proc), _) => {
            match parse_procs(tokens) {
                Ok((mut rest_procs, tokens)) => {
                    let mut vec = vec![proc];
                    vec.append(&mut rest_procs);
                    return Ok((vec, tokens));
                }
                Err(e) => return Err(e),
            };
        }
        (None, _) => Err(ParseError),
    }
}

fn parse_proc<'a>(tokens: &'a [T<'a>]) -> Result<(Option<Procedure<'a>>, &[T<'a>]), ParseError> {
    match tokens {
        [T::ID("proc"), T::ID(name), T::LPAREN, rest @ ..] => {
            let (params, tokens) = match parse_params(rest) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let tokens = match consume_token(T::LBRACKET, tokens) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            let (body_statements, tokens) = parse_statements(tokens);

            let tokens = match consume_token(T::RBRACKET, tokens) {
                Ok(t) => t,
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

fn parse_params<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<&'a str>, &[T<'a>]), ParseError> {
    match tokens {
        [T::RPAREN, rest @ ..] => Ok((vec![], rest)),
        [T::ID(param_name), rest_toks @ ..] => {
            let (mut rest_params, rest_toks) = match parse_params(rest_toks) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let mut params = vec![*param_name];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        _ => Err(ParseError),
    }
}

fn parse_statements<'a>(tokens: &'a [T<'a>]) -> (Statements, &[T<'a>]) {
    match parse_statement(tokens) {
        Ok((statement, tokens)) => {
            let (mut rest_statements, tokens) = parse_statements(tokens);
            let mut vec = vec![statement];
            vec.append(&mut rest_statements);
            (vec, tokens)
        }
        Err(_) => (vec![], tokens),
    }
}

fn parse_statement<'a>(tokens: &'a [T<'a>]) -> Result<(Statement, &[T<'a>]), ParseError> {
    match tokens {
        [T::ID("let"), T::ID(var_name), T::ASSIGN, rest @ ..] => {
            let (var_expr, tokens) = match parse_expr(rest) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((Statement::LET(&var_name, var_expr), tokens));
        }
        tokens => match parse_expr(tokens) {
            Ok((expr, tokens)) => Ok((Statement::EXPR(expr), tokens)),
            Err(e) => Err(e),
        },
    }
}

fn parse_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), ParseError> {
    let (additive_expr, tokens) = match parse_additive_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };
    match tokens {
        [T::EQ, tokens @ ..] => {
            let (relational_expr, tokens) = match parse_expr(tokens) {
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

fn parse_additive_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), ParseError> {
    let (terminal, tokens) = match parse_terminal_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    match tokens {
        [T::PLUS, tokens @ ..] => {
            let (additive_expr, tokens) = match parse_additive_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Plus, terminal, additive_expr),
                tokens,
            ));
        }
        [T::MINUS, tokens @ ..] => {
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

fn parse_terminal_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), ParseError> {
    match tokens {
        [T::ID(function_name), T::LPAREN, tokens @ ..] => {
            let (args, tokens) = match parse_args(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((Expr::Call(*function_name, args), tokens));
        }
        [T::ID(id), tokens @ ..] => match *id {
            "true" => Ok((Expr::Bool(true), tokens)),
            "false" => Ok((Expr::Bool(false), tokens)),
            _ => Ok((Expr::Var(id), tokens)),
        },
        [T::LPAREN, tokens @ ..] => {
            let (expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let tokens = match consume_token(T::RPAREN, tokens) {
                Ok(tokens) => tokens,
                Err(e) => return Err(e),
            };
            Ok((expr, tokens))
        }
        [T::NUM(n), tokens @ ..] => Ok((Expr::Num(*n), tokens)),
        _ => Err(ParseError),
    }
}

fn parse_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Expr>, &'a [T<'a>]), ParseError> {
    match tokens {
        [T::RPAREN, tokens @ ..] => Ok((vec![], tokens)),
        tokens => {
            let (expr, tokens) = match parse_expr(tokens) {
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

fn parse_rest_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Expr>, &'a [T<'a>]), ParseError> {
    match tokens {
        [T::RPAREN, tokens @ ..] => Ok((vec![], tokens)),
        [T::COMMA, T::RPAREN, ..] => Err(ParseError),
        [T::COMMA, tokens @ ..] => parse_args(tokens),
        _ => Err(ParseError),
    }
}
