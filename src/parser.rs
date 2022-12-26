use std::fmt;

use crate::tokenizer::Token as T;

#[derive(Debug)]
pub struct Program<'a> {
    procedures: Vec<Procedure<'a>>,
    main: Statements<'a>,
}

#[derive(Debug)]
pub struct Procedure<'a> {
    name: &'a str,
    params: Vec<&'a str>,
    body: Statements<'a>,
}

type Statements<'a> = Vec<Statement<'a>>;

#[derive(Copy, Clone, Debug)]
pub enum Statement<'a> {
    Expr(Expr),
    Let(&'a str, Expr),
}

#[derive(Copy, Clone, Debug)]
pub enum Expr {
    NUM(i64),
}

pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}

fn consume_token<'a>(target: T<'a>, tokens: Vec<T<'a>>) -> Result<Vec<T<'a>>, ParseError> {
    match tokens.as_slice() {
        [token, rest @ ..] if token.eq(&target) => Ok(rest.to_vec()),
        _ => Err(ParseError),
    }
}

pub fn parse_program(tokens: Vec<T>) -> Result<Program, ParseError> {
    let (procedures, rest) = parse_procs(tokens);
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

fn parse_procs(tokens: Vec<T>) -> (Vec<Procedure>, Vec<T>) {
    return (vec![], vec![]);
}

fn parse_proc<'a>(tokens: &'a [T<'a>]) -> Result<(Option<Procedure<'a>>, Vec<T<'a>>), ParseError> {
    match tokens {
        [T::ID("proc"), T::ID(name), T::LPAREN, rest @ ..] => {
            let (params, tokens) = match parse_params(rest) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let (body_statements, tokens) = match parse_statements(tokens) {
                Ok(statements) => statements,
                Err(e) => return Err(e),
            };

            Ok((
                Some(Procedure {
                    name,
                    params,
                    body: body_statements,
                }),
                tokens.to_vec(),
            ))
        }
        _ => Ok((None, tokens.to_vec())),
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

fn parse_statements<'a>(tokens: &'a [T<'a>]) -> Result<(Statements, Vec<T<'a>>), ParseError> {}
