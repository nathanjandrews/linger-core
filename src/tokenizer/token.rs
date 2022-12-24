use regex::Regex;

#[derive(Debug)]
pub enum Token {
    ID { value: String },
    NUM { n: i64 },
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
}

pub const WHITESPACE_REGEX: &str = r"\s+";
pub const ID_REGEX: &str = r"([a-zA-Z][a-zA-Z0-9_]*)\b";
pub const NUM_REGEX: &str = r"(-?\d+)\b";
pub const LPAREN_REGEX: &str = "(";
pub const RPAREN_REGEX: &str = ")";
pub const LBRACKET_REGEX: &str = "{";
pub const RBRACKET_REGEX: &str = "}";
pub const SEMICOLON_REGEX: &str = ";";
