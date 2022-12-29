use std::fmt::{self, write};

use crate::{
    interpreter::Value,
    tokenizer::{Operator, Token, TokenValue},
};

#[derive(Debug, Clone)]
pub enum TokenizerError {
    UnknownToken(String),
    UnterminatedStringLiteral,
}

#[derive(Debug, Clone)]
pub enum ParseError<'a> {
    NoMain,
    MultipleMain,
    UnexpectedToken(Token<'a>),
    Expected(TokenValue<'a>, Token<'a>),
    ExpectedSomething,
    KeywordAsVar(&'a str),
    KeywordAsProc(&'a str),
    KeywordAsParam(&'a str),
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum RuntimeError {
    UnknownVariable(String),
    UnknownProc(String),
    BadArg(Value),
    BadArgs(Vec<Value>),
    ArgMismatch(String, usize, usize),
    BadCondition(Value),
    BinaryAsUnary(Operator),
    UnaryAsBinary(Operator),
}

#[derive(Debug, Clone)]
pub enum LingerError<'a> {
    ParseError(ParseError<'a>),
    TokenizerError(TokenizerError),
    RuntimeError(RuntimeError),
}

impl fmt::Display for LingerError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LingerError::ParseError(err) => match err {
                ParseError::NoMain => write!(f, "main procedure not found"),
                ParseError::MultipleMain => write!(f, "multiple main procedures found"),
                ParseError::UnexpectedToken(token) => write!(
                    f,
                    "unexpected token {} @ ({}, {})",
                    token.0, token.1, token.2
                ),
                ParseError::Expected(target, token) => write!(
                    f,
                    "expected token {} @ ({}, {}), instead got {}",
                    target, token.1, token.2, token.0
                ),
                ParseError::Custom(s) => write!(f, "{}", s),
                ParseError::KeywordAsVar(keyword) => {
                    write!(f, "keyword \"{}\" used as variable", keyword)
                }
                ParseError::KeywordAsProc(keyword) => {
                    write!(f, "keyword \"{}\" used as procedure name", keyword)
                }
                ParseError::KeywordAsParam(keyword) => {
                    write!(f, "keyword \"{}\" used as parameter name", keyword)
                }
                ParseError::ExpectedSomething => write!(f, "expected token"),
            },
            LingerError::TokenizerError(err) => match err {
                TokenizerError::UnknownToken(s) => write!(f, "unknown token: {s}"),
                TokenizerError::UnterminatedStringLiteral => {
                    write!(f, "unterminated string literal")
                }
            },
            LingerError::RuntimeError(err) => match err {
                RuntimeError::UnknownVariable(id) => write!(f, "unknown variable \"{}\"", id),
                RuntimeError::BadArg(v) => write!(f, "bad argument \"{}\"", v),
                RuntimeError::UnknownProc(proc_name) => {
                    write!(f, "unknown procedure \"{}\"", proc_name)
                }
                RuntimeError::ArgMismatch(proc_name, actual, expected) => write!(
                    f,
                    "procedure {} expected {} args, instead got {}",
                    proc_name, expected, actual
                ),
                RuntimeError::BadCondition(v) => {
                    write!(f, "expected boolean value, instead got {}", v)
                }
                RuntimeError::BadArgs(args) => {
                    let arg_strings_vec: Vec<String> =
                        args.iter().map(|arg| arg.to_string()).collect();
                    let arg_string = arg_strings_vec.join(", ");
                    write!(f, "bad args: [{}]", arg_string)
                }
                RuntimeError::BinaryAsUnary(op) => {
                    write!(f, "binary operator \"{}\" used as unary operator", op)
                }
                RuntimeError::UnaryAsBinary(op) => {
                    write!(f, "unary operator \"{}\" used as binary operator", op)
                }
            },
        }
    }
}

pub fn unexpected_token<'a>(tokens: &'a [Token<'a>]) -> LingerError<'a> {
    return LingerError::ParseError(ParseError::UnexpectedToken(
        tokens.first().unwrap().to_owned(),
    ));
}
