use std::fmt;

use regex::{Match, Regex};

use crate::error::{LingerError as LE, TokenizerError};

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

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut format_msg = |s: &str| write!(f, "\"{s}\"");
        match self {
            Token::ID(id) => format_msg(id),
            Token::NUM(n) => format_msg(n.to_string().as_str()),
            Token::ASSIGN => format_msg("="),
            Token::EQ => format_msg("=="),
            Token::PLUS => format_msg("+"),
            Token::MINUS => format_msg("-"),
            Token::LPAREN => format_msg("("),
            Token::RPAREN => format_msg(")"),
            Token::LBRACKET => format_msg("{"),
            Token::RBRACKET => format_msg("}"),
            Token::SEMICOLON => format_msg(";"),
            Token::COMMA => format_msg(","),
        }
    }
}

pub struct Tokens<'a>(pub Vec<Token<'a>>);

impl fmt::Display for Tokens<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{s}]")
    }
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

pub fn tokenize(s: &str) -> Result<Vec<Token>, LE> {
    tokenize_helper(s)
}

fn tokenize_helper(s: &str) -> Result<Vec<Token>, LE> {
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

fn get_token(s: &str) -> Result<(Option<Token>, usize), LE> {
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
        Err(LE::TokenizerError(TokenizerError({
            let mut split =
                s.split(|c: char| str_to_regex(WHITESPACE_REGEX).is_match(c.to_string().as_str()));
            let unknown_token = split.nth(0).unwrap();
            format!("\"{}\"", unknown_token).to_string()
        })))
    }
}

fn str_to_regex(s: &str) -> Regex {
    return Regex::new(format!("^({s})").as_str()).unwrap();
}

fn find<'a>(re: &'a str, s: &'a str) -> Option<Match<'a>> {
    return str_to_regex(re).find(s);
}
