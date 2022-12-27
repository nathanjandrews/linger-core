use std::fmt;

#[derive(Debug, Clone)]
pub struct TokenizerError;

#[derive(Debug, Clone)]
pub struct ParseError;

pub enum LingerError {
    ParseError(ParseError),
    TokenizerError(TokenizerError),
}

impl fmt::Display for LingerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LingerError::ParseError(_) => write!(f, "parse error"),
            LingerError::TokenizerError(_) => write!(f, "tokenizer error"),
        }
    }
}
