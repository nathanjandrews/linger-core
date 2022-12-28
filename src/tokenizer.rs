use std::fmt;

use regex::{Match, Regex};

use crate::error::{LingerError as LE, TokenizerError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Token<'a>(pub TokenValue<'a>, pub usize, pub usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TokenValue<'a> {
    ID(&'a str),
    NUM(i64),
    ASSIGN,
    BIN_OP(BinaryOperator),
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
    COMMA,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Eq,
    Ne,
    LT,
    GT,
    LTE,
    GTE,
    Mod,
    LogicOr,
    LogicAnd,
}

impl fmt::Display for TokenValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut format_msg = |s: &str| write!(f, "{s}");
        match self {
            TokenValue::ID(id) => format_msg(id),
            TokenValue::NUM(n) => format_msg(n.to_string().as_str()),
            TokenValue::ASSIGN => format_msg("="),
            TokenValue::LPAREN => format_msg("("),
            TokenValue::RPAREN => format_msg(")"),
            TokenValue::LBRACKET => format_msg("{"),
            TokenValue::RBRACKET => format_msg("}"),
            TokenValue::SEMICOLON => format_msg(";"),
            TokenValue::COMMA => format_msg(","),
            TokenValue::BIN_OP(op) => match op {
                BinaryOperator::Plus => format_msg("+"),
                BinaryOperator::Minus => format_msg("-"),
                BinaryOperator::Times => format_msg("*"),
                BinaryOperator::Eq => format_msg("=="),
                BinaryOperator::Ne => format_msg("!="),
                BinaryOperator::LT => format_msg("<"),
                BinaryOperator::GT => format_msg(">"),
                BinaryOperator::LTE => format_msg("<="),
                BinaryOperator::GTE => format_msg(">="),
                BinaryOperator::Mod => format_msg("%"),
                BinaryOperator::LogicOr => format_msg("||"),
                BinaryOperator::LogicAnd => format_msg("&&"),
            },
        }
    }
}

pub struct Tokens<'a>(pub Vec<Token<'a>>);

impl fmt::Display for Tokens<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|t| t.0.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{s}]")
    }
}

pub const WHITESPACE_REGEX: &str = r"[[:space:]]+";
pub const ASSIGN_REGEX: &str = r"=";
pub const EQ_REGEX: &str = r"==";
pub const NE_REGEX: &str = r"!=";
pub const LT_REGEX: &str = r"<";
pub const GT_REGEX: &str = r">";
pub const LTE_REGEX: &str = r"<=";
pub const GTE_REGEX: &str = r">=";
pub const ID_REGEX: &str = r"([a-zA-Z][a-zA-Z0-9_]*)\b";
pub const NUM_REGEX: &str = r"(-?\d+)\b";
pub const PLUS_REGEX: &str = r"\+";
pub const MINUS_REGEX: &str = r"\-";
pub const STAR_REGEX: &str = r"\*";
pub const MOD_REGEX: &str = "%";
pub const LPAREN_REGEX: &str = r"\(";
pub const RPAREN_REGEX: &str = r"\)";
pub const LBRACKET_REGEX: &str = r"\{";
pub const RBRACKET_REGEX: &str = r"\}";
pub const SEMICOLON_REGEX: &str = ";";
pub const COMMA_REGEX: &str = ",";
pub const LOGIC_OR_REGEX: &str = r"\|\|";
pub const LOGIC_AND_REGEX: &str = "&&";

pub fn tokenize(s: &str) -> Result<Vec<Token>, LE> {
    let enumerated_lines = s.split("\n").enumerate();
    let mut tokens: Vec<Token> = vec![];
    for (line_num, line) in enumerated_lines {
        let mut line_tokens = match tokenize_helper(line, line_num + 1, 1) {
            Ok(tokens) => tokens,
            Err(e) => return Err(e),
        };
        tokens.append(&mut line_tokens)
    }
    Ok(tokens)
}

fn tokenize_helper(s: &str, line_num: usize, col_num: usize) -> Result<Vec<Token>, LE> {
    if s.len() <= 0 {
        Ok(vec![])
    } else {
        match get_token(s, line_num, col_num) {
            Ok((token_option, new_index)) => match token_option {
                Some(token) => match tokenize_helper(&s[new_index..], line_num, new_index + 1) {
                    Ok(mut vec) => {
                        let mut v = vec![token];
                        v.append(&mut vec);
                        Ok(v)
                    }
                    Err(e) => Err(e),
                },
                None => tokenize_helper(&s[new_index..], line_num, new_index + 1),
            },
            Err(e) => Err(e),
        }
    }
}

fn get_token(s: &str, row: usize, col: usize) -> Result<(Option<Token>, usize), LE> {
    if let Some(mat) = find(WHITESPACE_REGEX, s) {
        Ok((None, mat.end()))
    } else if let Some(mat) = find(NE_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::Ne), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(EQ_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::Eq), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(LTE_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::LTE), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(GTE_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::GTE), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(LOGIC_AND_REGEX, s) {
        Ok((
            Some(Token(
                TokenValue::BIN_OP(BinaryOperator::LogicAnd),
                row,
                col,
            )),
            mat.end(),
        ))
    } else if let Some(mat) = find(LOGIC_OR_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::LogicOr), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(ASSIGN_REGEX, s) {
        Ok((Some(Token(TokenValue::ASSIGN, row, col)), mat.end()))
    } else if let Some(mat) = find(LT_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::LT), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(GT_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::GT), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(STAR_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::Times), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(MOD_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::Mod), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(ID_REGEX, s) {
        Ok((
            Some(Token(TokenValue::ID(mat.as_str()), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(NUM_REGEX, s) {
        Ok((
            Some(Token(
                TokenValue::NUM(mat.as_str().parse::<i64>().unwrap()),
                row,
                col,
            )),
            mat.end(),
        ))
    } else if let Some(mat) = find(PLUS_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::Plus), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(MINUS_REGEX, s) {
        Ok((
            Some(Token(TokenValue::BIN_OP(BinaryOperator::Minus), row, col)),
            mat.end(),
        ))
    } else if let Some(mat) = find(LPAREN_REGEX, s) {
        Ok((Some(Token(TokenValue::LPAREN, row, col)), mat.end()))
    } else if let Some(mat) = find(RPAREN_REGEX, s) {
        Ok((Some(Token(TokenValue::RPAREN, row, col)), mat.end()))
    } else if let Some(mat) = find(LBRACKET_REGEX, s) {
        Ok((Some(Token(TokenValue::LBRACKET, row, col)), mat.end()))
    } else if let Some(mat) = find(RBRACKET_REGEX, s) {
        Ok((Some(Token(TokenValue::RBRACKET, row, col)), mat.end()))
    } else if let Some(mat) = find(SEMICOLON_REGEX, s) {
        Ok((Some(Token(TokenValue::SEMICOLON, row, col)), mat.end()))
    } else if let Some(mat) = find(COMMA_REGEX, s) {
        Ok((Some(Token(TokenValue::COMMA, row, col)), mat.end()))
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
