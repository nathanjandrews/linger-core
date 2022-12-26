use regex::{Match, Regex};
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token<'a> {
    ID(&'a str),
    NUM(i64),
    ASSIGN,
    EQ,
    PLUS,
    MINUS,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
    COMMA,
}

pub const WHITESPACE_REGEX: &str = r"[[:space:]]+";
pub const ASSIGN_REGEX: &str = r"=";
pub const EQ_REGEX: &str = r"==";
pub const ID_REGEX: &str = r"([a-zA-Z][a-zA-Z0-9_]*)\b";
pub const NUM_REGEX: &str = r"(-?\d+)\b";
pub const PLUS_REGEX: &str = r"\+";
pub const MINUS_REGEX: &str = r"\-";
pub const LPAREN_REGEX: &str = r"\(";
pub const RPAREN_REGEX: &str = r"\)";
pub const LBRACKET_REGEX: &str = r"\{";
pub const RBRACKET_REGEX: &str = r"\}";
pub const SEMICOLON_REGEX: &str = ";";
pub const COMMA_REGEX: &str = ",";

#[derive(Debug, Clone)]
pub struct TokenizerError;

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "token error")
    }
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, TokenizerError> {
    tokenize_helper(s)
}

fn tokenize_helper(s: &str) -> Result<Vec<Token>, TokenizerError> {
    if s.len() <= 0 {
        Ok(vec![])
    } else {
        match get_token(s) {
            Ok((token_option, new_index)) => match token_option {
                Some(token) => match tokenize_helper(&s[new_index..]) {
                    Ok(mut vec) => {
                        let mut v = vec![token];
                        v.append(&mut vec);
                        Ok(v)
                    }
                    Err(e) => Err(e),
                },
                None => tokenize_helper(&s[new_index..]),
            },
            Err(e) => Err(e),
        }
    }
}

fn get_token(s: &str) -> Result<(Option<Token>, usize), TokenizerError> {
    if let Some(mat) = find(WHITESPACE_REGEX, s) {
        Ok((None, mat.end()))
    } else if let Some(mat) = find(EQ_REGEX, s) {
        Ok((Some(Token::EQ), mat.end()))
    } else if let Some(mat) = find(ASSIGN_REGEX, s) {
        Ok((Some(Token::ASSIGN), mat.end()))
    } else if let Some(mat) = find(ID_REGEX, s) {
        Ok((Some(Token::ID(mat.as_str())), mat.end()))
    } else if let Some(mat) = find(NUM_REGEX, s) {
        Ok((
            Some(Token::NUM(mat.as_str().parse::<i64>().unwrap())),
            mat.end(),
        ))
    } else if let Some(mat) = find(PLUS_REGEX, s) {
        Ok((Some(Token::PLUS), mat.end()))
    } else if let Some(mat) = find(MINUS_REGEX, s) {
        Ok((Some(Token::MINUS), mat.end()))
    } else if let Some(mat) = find(LPAREN_REGEX, s) {
        Ok((Some(Token::LPAREN), mat.end()))
    } else if let Some(mat) = find(RPAREN_REGEX, s) {
        Ok((Some(Token::RPAREN), mat.end()))
    } else if let Some(mat) = find(LBRACKET_REGEX, s) {
        Ok((Some(Token::LBRACKET), mat.end()))
    } else if let Some(mat) = find(RBRACKET_REGEX, s) {
        Ok((Some(Token::RBRACKET), mat.end()))
    } else if let Some(mat) = find(SEMICOLON_REGEX, s) {
        Ok((Some(Token::SEMICOLON), mat.end()))
    } else if let Some(mat) = find(COMMA_REGEX, s) {
        Ok((Some(Token::COMMA), mat.end()))
    } else {
        Err(TokenizerError {})
    }
}

fn find<'a>(re: &'a str, s: &'a str) -> Option<Match<'a>> {
    return Regex::new(format!("^({re})").as_str()).unwrap().find(s);
}
