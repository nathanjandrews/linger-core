use std::fmt;

use regex::{Match, Regex};

use crate::error::{
    LingerError::{self, *},
    TokenizerError::*,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Token<'a>(pub TokenValue<'a>, pub usize, pub usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenValue<'a> {
    ID(&'a str),
    STR(String),
    NUM(i64),
    ASSIGN,
    OP(Operator),
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
    QUOTE,
    COMMA,
    THIN_ARROW,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Operator {
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
    Div,
    LogicOr,
    LogicAnd,
    LogicNot,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Times => write!(f, "*"),
            Operator::Eq => write!(f, "=="),
            Operator::Ne => write!(f, "!="),
            Operator::LT => write!(f, "<"),
            Operator::GT => write!(f, ">"),
            Operator::LTE => write!(f, "<="),
            Operator::GTE => write!(f, ">="),
            Operator::Mod => write!(f, "%"),
            Operator::LogicOr => write!(f, "||"),
            Operator::LogicAnd => write!(f, "&&"),
            Operator::Div => write!(f, "/"),
            Operator::LogicNot => write!(f, "!"),
        }
    }
}

impl fmt::Display for TokenValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValue::ID(id) => write!(f, "{id}"),
            TokenValue::NUM(n) => write!(f, "{n}"),
            TokenValue::ASSIGN => write!(f, "="),
            TokenValue::LPAREN => write!(f, "("),
            TokenValue::RPAREN => write!(f, ")"),
            TokenValue::LBRACKET => write!(f, "{{"),
            TokenValue::RBRACKET => write!(f, "}}"),
            TokenValue::SEMICOLON => write!(f, ";"),
            TokenValue::COMMA => write!(f, ","),
            TokenValue::OP(op) => write!(f, "{op}"),
            TokenValue::QUOTE => write!(f, "\""),
            TokenValue::STR(s) => write!(f, "\"{s}\""),
            TokenValue::THIN_ARROW => write!(f, "->"),
        }
    }
}

pub const WHITESPACE_REGEX: &str = r"[[:space:]]+";
pub const ASSIGN_REGEX: &str = r"=";
pub const THIN_ARROW_REGEX: &str = r"->";
pub const EQ_REGEX: &str = r"==";
pub const NE_REGEX: &str = r"!=";
pub const LT_REGEX: &str = r"<";
pub const GT_REGEX: &str = r">";
pub const LTE_REGEX: &str = r"<=";
pub const GTE_REGEX: &str = r">=";
pub const ID_REGEX: &str = r"([a-zA-Z][a-zA-Z0-9_]*)\b";
pub const NUM_REGEX: &str = r"(\d+)\b";
pub const PLUS_REGEX: &str = r"\+";
pub const MINUS_REGEX: &str = r"\-";
pub const STAR_REGEX: &str = r"\*";
pub const SLASH_REGEX: &str = r"/";
pub const MOD_REGEX: &str = "%";
pub const LPAREN_REGEX: &str = r"\(";
pub const RPAREN_REGEX: &str = r"\)";
pub const LBRACKET_REGEX: &str = r"\{";
pub const RBRACKET_REGEX: &str = r"\}";
pub const SEMICOLON_REGEX: &str = ";";
pub const COMMA_REGEX: &str = ",";
pub const QUOTE_REGEX: &str = "\"";
pub const LOGIC_OR_REGEX: &str = r"\|\|";
pub const LOGIC_AND_REGEX: &str = "&&";
pub const LOGIC_NOT_REGEX: &str = "!";

pub fn tokenize(s: &str) -> Result<Vec<Token>, LingerError> {
    let enumerated_lines = s.split("\n").enumerate();
    let mut tokens: Vec<Token> = vec![];
    for (line_num, line) in enumerated_lines {
        let mut tokenized_line = tokenize_helper(line, line_num + 1, 1)?;
        tokens.append(&mut tokenized_line)
    }
    Ok(tokens)
}

fn tokenize_helper(s: &str, line_num: usize, col_num: usize) -> Result<Vec<Token>, LingerError> {
    if s.len() == 0 {
        return Ok(vec![]);
    }

    let (token_value_option, token_length) = get_token_value(s)?;
    let token_value = match token_value_option {
        Some(token) => token,
        None => return tokenize_helper(&s[token_length..], line_num, col_num + token_length),
    };

    match token_value {
        TokenValue::QUOTE => {
            let s = &s[token_length..];
            let mut string_token_content = String::new();
            let mut enumerated_character_iter = s.chars().enumerate();
            while let Some((index, char)) = enumerated_character_iter.next() {
                match char {
                    '"' => {
                        let mut tokens = vec![Token(
                            TokenValue::STR(string_token_content.to_string()),
                            line_num,
                            col_num,
                        )];
                        let mut rest_tokens = tokenize_helper(
                            &s[index + 1..],
                            line_num,
                            // the "plus 2" if to account for the opening and closing quotes for the string literal
                            col_num + string_token_content.len() + 2,
                        )?;
                        tokens.append(&mut rest_tokens);
                        return Ok(tokens);
                    }
                    '\\' => match enumerated_character_iter.nth(0) {
                        Some((_, escaped_char)) => match escaped_char {
                            'n' => string_token_content.push('\n'),
                            'r' => string_token_content.push('\r'),
                            't' => string_token_content.push('\t'),
                            '\\' => string_token_content.push('\\'),
                            '0' => string_token_content.push('\0'),
                            '"' => string_token_content.push('"'),
                            '\'' => string_token_content.push('\''),
                            c => return Err(TokenizerError(InvalidEscapeSequence(c))),
                        },
                        None => return Err(TokenizerError(UnterminatedStringLiteral)),
                    },
                    _ => string_token_content.push(char),
                }
            }
            return Err(TokenizerError(UnterminatedStringLiteral));
        }
        token_value => {
            let mut tokens = vec![Token(token_value, line_num, col_num)];
            let mut rest_tokens =
                tokenize_helper(&s[token_length..], line_num, col_num + token_length)?;
            tokens.append(&mut rest_tokens);
            return Ok(tokens);
        }
    }
}

fn get_token_value(s: &str) -> Result<(Option<TokenValue>, usize), LingerError> {
    // WHITESPACE TOKEN
    if let Some(mat) = find(WHITESPACE_REGEX, s) {
        Ok((None, mat.end()))

    // TWO-CHARACTER TOKENS
    } else if let Some(mat) = find(NE_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Ne)), mat.end()))
    } else if let Some(mat) = find(EQ_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Eq)), mat.end()))
    } else if let Some(mat) = find(LTE_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::LTE)), mat.end()))
    } else if let Some(mat) = find(GTE_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::GTE)), mat.end()))
    } else if let Some(mat) = find(LOGIC_AND_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::LogicAnd)), mat.end()))
    } else if let Some(mat) = find(LOGIC_OR_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::LogicOr)), mat.end()))
    } else if let Some(mat) = find(THIN_ARROW_REGEX, s) {
        Ok((Some(TokenValue::THIN_ARROW), mat.end()))

    // ONE-CHARACTER TOKENS
    } else if let Some(mat) = find(ASSIGN_REGEX, s) {
        Ok((Some(TokenValue::ASSIGN), mat.end()))
    } else if let Some(mat) = find(LT_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::LT)), mat.end()))
    } else if let Some(mat) = find(GT_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::GT)), mat.end()))
    } else if let Some(mat) = find(STAR_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Times)), mat.end()))
    } else if let Some(mat) = find(MOD_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Mod)), mat.end()))
    } else if let Some(mat) = find(SLASH_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Div)), mat.end()))
    } else if let Some(mat) = find(PLUS_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Plus)), mat.end()))
    } else if let Some(mat) = find(MINUS_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::Minus)), mat.end()))
    } else if let Some(mat) = find(LPAREN_REGEX, s) {
        Ok((Some(TokenValue::LPAREN), mat.end()))
    } else if let Some(mat) = find(RPAREN_REGEX, s) {
        Ok((Some(TokenValue::RPAREN), mat.end()))
    } else if let Some(mat) = find(LBRACKET_REGEX, s) {
        Ok((Some(TokenValue::LBRACKET), mat.end()))
    } else if let Some(mat) = find(RBRACKET_REGEX, s) {
        Ok((Some(TokenValue::RBRACKET), mat.end()))
    } else if let Some(mat) = find(SEMICOLON_REGEX, s) {
        Ok((Some(TokenValue::SEMICOLON), mat.end()))
    } else if let Some(mat) = find(COMMA_REGEX, s) {
        Ok((Some(TokenValue::COMMA), mat.end()))
    } else if let Some(mat) = find(QUOTE_REGEX, s) {
        Ok((Some(TokenValue::QUOTE), mat.end()))
    } else if let Some(mat) = find(LOGIC_NOT_REGEX, s) {
        Ok((Some(TokenValue::OP(Operator::LogicNot)), mat.end()))

    // VARIABLE-LENGTH TOKENS
    } else if let Some(mat) = find(ID_REGEX, s) {
        Ok((Some(TokenValue::ID(mat.as_str())), mat.end()))
    } else if let Some(mat) = find(NUM_REGEX, s) {
        Ok((
            Some(TokenValue::NUM(mat.as_str().parse::<i64>().expect("a match with the NUM_REGEX should imply that the string slice can be parsed into am i64"))),
            mat.end(),
        ))
    } else {
        Err(TokenizerError(UnknownToken({
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
