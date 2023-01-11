use std::fmt;

use regex::{Match, Regex};

use crate::error::TokenizerError::{self, *};

/// A Linger token.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Token(pub TokenValue, pub usize, pub usize);

/// A Linger token value. This is an enum which represents the type of the
/// token along with any associated data with that type.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenValue {
    ID(String),
    STR(String),
    NUM(f64),
    ASSIGN,
    OP(Operator),
    KW(Keyword),
    ASSIGN_OP(AssignOp),
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
    QUOTE,
    COMMA,
    THIN_ARROW,
    DOUBLE_SLASH,
    DOUBLE_PLUS,
    DOUBLE_MINUS,
}

/// An operator. This enum represents all of the valid operators in the Linger
/// programming language. The variants of this enum are the associated data for
/// the [OP TokenValue](TokenValue::OP) variant.
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
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum AssignOp {
    Plus,
    Minus,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Keyword {
    If,
    Else,
    Proc,
    Let,
    Const,
    True,
    False,
    Return,
    While,
    Break,
    Continue,
    For,
}

const WHITESPACE_REGEX: &str = r"[[:space:]]+";
const ASSIGN_REGEX: &str = r"=";
const THIN_ARROW_REGEX: &str = r"->";
const EQ_REGEX: &str = r"==";
const NE_REGEX: &str = r"!=";
const LT_REGEX: &str = r"<";
const GT_REGEX: &str = r">";
const LTE_REGEX: &str = r"<=";
const GTE_REGEX: &str = r">=";
const ID_REGEX: &str = r"([a-zA-Z][a-zA-Z0-9_]*)\b";
const NUM_REGEX: &str = r"\d*\.?\d+";
const PLUS_REGEX: &str = r"\+";
const MINUS_REGEX: &str = r"\-";
const STAR_REGEX: &str = r"\*";
const SLASH_REGEX: &str = r"/";
const DOUBLE_SLASH_REGEX: &str = r"//";
const DOUBLE_PLUS_REGEX: &str = r"\+\+";
const DOUBLE_MINUS_REGEX: &str = r"\-\-";
const MOD_REGEX: &str = "%";
const LPAREN_REGEX: &str = r"\(";
const RPAREN_REGEX: &str = r"\)";
const LBRACKET_REGEX: &str = r"\{";
const RBRACKET_REGEX: &str = r"\}";
const SEMICOLON_REGEX: &str = ";";
const COMMA_REGEX: &str = ",";
const QUOTE_REGEX: &str = "\"";
const LOGIC_OR_REGEX: &str = r"\|\|";
const LOGIC_AND_REGEX: &str = "&&";
const LOGIC_NOT_REGEX: &str = "!";
const ASSIGNMENT_PLUS_REGEX: &str = r"\+=";
const ASSIGNMENT_MINUS_REGEX: &str = r"\-=";

/// Returns the [Tokens](Token) which make up the program `s`.
pub fn tokenize(s: &str) -> Result<Vec<Token>, TokenizerError> {
    let enumerated_lines = s.split("\n").enumerate();
    let mut tokens: Vec<Token> = vec![];
    for (line_num, line) in enumerated_lines {
        let mut tokenized_line = tokenize_helper(line, line_num + 1, 1)?;
        tokens.append(&mut tokenized_line)
    }
    Ok(tokens)
}

/// Returns the [Tokens](Token) which make up the program `s`. This is a helper function which is
/// wrapped by [tokenize]. This function also takes a line and column number which are passed to
/// created token structures.
fn tokenize_helper(s: &str, line_num: usize, col_num: usize) -> Result<Vec<Token>, TokenizerError> {
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
                            // the "plus 2" is to account for the opening and closing quotes for the string literal
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
                            c => return Err(InvalidEscapeSequence(c)),
                        },
                        None => return Err(UnterminatedStringLiteral),
                    },
                    _ => string_token_content.push(char),
                }
            }
            return Err(UnterminatedStringLiteral);
        }
        TokenValue::DOUBLE_SLASH => return Ok(vec![]),
        token_value => {
            let mut tokens = vec![Token(token_value, line_num, col_num)];
            let mut rest_tokens =
                tokenize_helper(&s[token_length..], line_num, col_num + token_length)?;
            tokens.append(&mut rest_tokens);
            return Ok(tokens);
        }
    }
}

/// Tries to get a token beginning at the start of `s`. On success, this function returns an option
/// of a [Token] that is None in the case of whitespace, or Some(Token) in all other cases. If the
/// beginning of `s` is not a known token, this function returns a [TokenizerError].
fn get_token_value(s: &str) -> Result<(Option<TokenValue>, usize), TokenizerError> {
    // WHITESPACE TOKEN
    if let Some(mat) = find(WHITESPACE_REGEX, s) {
        Ok((None, mat.end()))

    // KEYWORDS
    } else if let Some(mat) = find("if", s) {
        Ok((Some(TokenValue::KW(Keyword::If)), mat.end()))
    } else if let Some(mat) = find("else", s) {
        Ok((Some(TokenValue::KW(Keyword::Else)), mat.end()))
    } else if let Some(mat) = find("proc", s) {
        Ok((Some(TokenValue::KW(Keyword::Proc)), mat.end()))
    } else if let Some(mat) = find("let", s) {
        Ok((Some(TokenValue::KW(Keyword::Let)), mat.end()))
    } else if let Some(mat) = find("true", s) {
        Ok((Some(TokenValue::KW(Keyword::True)), mat.end()))
    } else if let Some(mat) = find("false", s) {
        Ok((Some(TokenValue::KW(Keyword::False)), mat.end()))
    } else if let Some(mat) = find("return", s) {
        Ok((Some(TokenValue::KW(Keyword::Return)), mat.end()))
    } else if let Some(mat) = find("while", s) {
        Ok((Some(TokenValue::KW(Keyword::While)), mat.end()))
    } else if let Some(mat) = find("break", s) {
        Ok((Some(TokenValue::KW(Keyword::Break)), mat.end()))
    } else if let Some(mat) = find("continue", s) {
        Ok((Some(TokenValue::KW(Keyword::Continue)), mat.end()))
    } else if let Some(mat) = find("for", s) {
        Ok((Some(TokenValue::KW(Keyword::For)), mat.end()))
    } else if let Some(mat) = find("const", s) {
        Ok((Some(TokenValue::KW(Keyword::Const)), mat.end()))

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
    } else if let Some(mat) = find(DOUBLE_SLASH_REGEX, s) {
        Ok((Some(TokenValue::DOUBLE_SLASH), mat.end()))
    } else if let Some(mat) = find(THIN_ARROW_REGEX, s) {
        Ok((Some(TokenValue::THIN_ARROW), mat.end()))
    } else if let Some(mat) = find(DOUBLE_PLUS_REGEX, s) {
        Ok((Some(TokenValue::DOUBLE_PLUS), mat.end()))
    } else if let Some(mat) = find(DOUBLE_MINUS_REGEX, s) {
        Ok((Some(TokenValue::DOUBLE_MINUS), mat.end()))
    } else if let Some(mat) = find(ASSIGNMENT_PLUS_REGEX, s) {
        Ok((Some(TokenValue::ASSIGN_OP(AssignOp::Plus)), mat.end()))
    } else if let Some(mat) = find(ASSIGNMENT_MINUS_REGEX, s) {
        Ok((Some(TokenValue::ASSIGN_OP(AssignOp::Minus)), mat.end()))

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
        Ok((Some(TokenValue::ID(mat.as_str().to_string())), mat.end()))
    } else if let Some(mat) = find(NUM_REGEX, s) {
        Ok((
            Some(TokenValue::NUM(mat.as_str().parse::<f64>().expect("a match with the NUM_REGEX should imply that the string slice can be parsed into am i64"))),
            mat.end(),
        ))

    // THE ERROR CASE
    } else {
        Err(UnknownToken({
            let mut split =
                s.split(|c: char| str_to_regex(WHITESPACE_REGEX).is_match(c.to_string().as_str()));
            let unknown_token = split.nth(0).expect("some non-whitespace text since whitespace would have been matched on the first branch of the if statement");
            format!("{}", unknown_token).to_string()
        }))
    }
}

/// Takes a string and returns the corresponding [Regex].
fn str_to_regex(s: &str) -> Regex {
    return Regex::new(format!("^({s})").as_str())
        .expect("strings to be valid regular expressions");
}

/// Checks if `s` starts with the regular expression represented by `re`.
fn find<'a>(re: &'a str, s: &'a str) -> Option<Match<'a>> {
    return str_to_regex(re).find(s);
}

impl fmt::Display for AssignOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignOp::Plus => write!(f, "+="),
            AssignOp::Minus => write!(f, "-="),
        }
    }
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
            Operator::PreIncrement => write!(f, "++"),
            Operator::PostIncrement => write!(f, "++"),
            Operator::PreDecrement => write!(f, "--"),
            Operator::PostDecrement => write!(f, "--"),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::Proc => write!(f, "proc"),
            Keyword::Let => write!(f, "let"),
            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),
            Keyword::Return => write!(f, "return"),
            Keyword::While => write!(f, "while"),
            Keyword::Break => write!(f, "break"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::For => write!(f, "for"),
            Keyword::Const => write!(f, "const"),
        }
    }
}

impl fmt::Display for TokenValue {
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
            TokenValue::DOUBLE_SLASH => write!(f, "//"),
            TokenValue::KW(kw) => write!(f, "{kw}"),
            TokenValue::DOUBLE_PLUS => write!(f, "++"),
            TokenValue::DOUBLE_MINUS => write!(f, "--"),
            TokenValue::ASSIGN_OP(op) => write!(f, "{op}"),
        }
    }
}
