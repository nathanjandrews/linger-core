use std::fmt;

use crate::tokenizer::Token;

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
            LingerError::ParseError(_) => write!(f, "parse error"),
            LingerError::TokenizerError(_) => write!(f, "tokenizer error"),
        }
    }
}

pub fn unexpected_token<'a>(tokens: &'a [Token<'a>]) -> LingerError<'a> {
    return LingerError::ParseError(ParseError::UnexpectedToken(tokens.to_vec()));
}
