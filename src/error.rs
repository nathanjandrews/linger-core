use std::fmt;

use crate::tokenizer::{Token, Tokens};

#[derive(Debug, Clone)]
pub struct TokenizerError;

#[derive(Debug, Clone)]
pub enum ParseError<'a> {
    NoMain,
    MultipleMain,
    MissingSemicolon,
    UnexpectedToken(Vec<Token<'a>>),
}

pub enum LingerError<'a> {
    ParseError(ParseError<'a>),
    TokenizerError(TokenizerError),
}

impl fmt::Display for LingerError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LingerError::ParseError(err) => {
                match err {
                    ParseError::NoMain => write!(f, "main procedure not found"),
                    ParseError::MultipleMain => write!(f, "multiple main procedures found"),
                    ParseError::MissingSemicolon => write!(f, "missing semicolon"),
                    ParseError::UnexpectedToken(tokens) => write!(f, "unexpected tokens: {}", Tokens(tokens.to_vec())),
                }
            }
            LingerError::TokenizerError(_) => write!(f, "tokenizer error"),
        }
    }
}

pub fn unexpected_token<'a>(tokens: &'a [Token<'a>]) -> LingerError<'a> {
    return LingerError::ParseError(ParseError::UnexpectedToken(tokens.to_vec()));
}
