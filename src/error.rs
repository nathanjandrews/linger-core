use std::fmt::{self, Display};

use crate::{
    interpreter::Value,
    tokenizer::{Operator, Token, TokenValue},
};

/// A Tokenizer Error
#[derive(Debug, Clone)]
pub enum TokenizerError {
    /// This error occurs when the tokenizer reaches a set of characters that
    /// does not match to a known token.
    UnknownToken(String),
    /// This error occurs when the tokenizer tokenizes a string but never
    /// reaches a terminating double quote.
    UnterminatedStringLiteral,
    /// This error occurs when the tokenizer reaches an invalid escape sequence.
    InvalidEscapeSequence(char),
}

/// A Parse Error
#[derive(Debug, Clone)]
pub enum ParseError {
    /// This error occurs when there is no `main` procedure.
    NoMain,
    /// This error occurs when there are multiple top-level procedures with the same name.
    MultipleSameNamedProcs(String),
    /// This error occurs when there is an unexpected token consumed when parsing.
    UnexpectedToken(Token),
    /// This error occurs when the parser unexpectedly reached the end of the file
    UnexpectedEOF,
    /// This error occurs when the consume token differs from the token that was expected.
    Expected(TokenValue, Token),
    /// This error occurs when a keyword is used a variable name.
    KeywordAsVar(String),
    /// This error occurs when a keyword is used as the name of a top-level procedure.
    KeywordAsProc(String),
    /// This error occurs when a keyword is used as the name of a procedure parameter.
    KeywordAsParam(String),
    /// This error occurs when the parser expects to parse a statement but was unsuccessful.
    ExpectedStatement,
    /// This error occurs when the parser expects to parse a block statement but was unsuccessful.
    ExpectedBlock,
    /// This error occurs when the parser expects to parse an assignment statement but was
    /// unsuccessful.
    ExpectedAssignment,
    /// This error occurs when the parser expects to parse an assignment statement or an
    /// initialization statement but was unsuccessful.
    ExpectedAssignmentOrInitialization,
}

/// A Runtime Error
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// This error occurs when the interpreter encounters an variable unbound in the environment.
    UnknownVariable(String),
    /// This error occurs when a single argument to a procedure is incorrect.
    BadArg(Value),
    /// This error occurs when multiple arguments to a procedure are incorrect.
    BadArgs(Vec<Value>),
    /// This error occurs when the number of arguments passed to a procedure is different from the
    /// number of parameters defined for that procedure.
    ArgMismatch(String, usize, usize),
    /// This error occurs when a value is expected to be a boolean but is not.
    ExpectedBool(Value),
    /// This error occurs when a value is expected to be a integer but is not.
    ExpectedInteger(Value),
    /// This error occurs when a value is expected to be a list but is not
    ExpectedList(Value),
    /// This error occurs when a binary operator is used as a unary operator.
    BinaryAsUnary(Operator),
    /// This error occurs when a unary operator is used as a binary operator.
    UnaryAsBinary(Operator),
    /// This error occurs when a `break` statement occurs outside of a loop.
    BreakNotInLoop,
    /// This error occurs when a `continue` statement occurs outside of a loop.
    ContinueNotInLoop,
    /// This error occurs when an expression is not a variable expression
    InvalidAssignmentTarget,
    /// This error occurs when attempting to reassign a constant value
    ReassignConstant(String),
    /// This error occurs when attempting to reassign a top-level procedure
    ReassignTopLevelProc(String),
    /// This error occurs when attempting to index a non-indexable value
    NotIndexable(Value),
    /// This error occurs when trying to index a value and the index is out
    /// of bounds
    IndexOutOfBounds(i64),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoMain => write!(f, "main procedure not found"),
            ParseError::UnexpectedToken(token) => write!(
                f,
                "unexpected token \"{}\" @ ({}, {})",
                token.0, token.1, token.2
            ),
            ParseError::Expected(target, token) => write!(
                f,
                "expected token \"{}\" @ ({}, {}), instead got \"{}\"",
                target, token.1, token.2, token.0
            ),
            ParseError::KeywordAsVar(keyword) => {
                write!(f, "keyword \"{}\" used as variable", keyword)
            }
            ParseError::KeywordAsProc(keyword) => {
                write!(f, "keyword \"{}\" used as procedure name", keyword)
            }
            ParseError::KeywordAsParam(keyword) => {
                write!(f, "keyword \"{}\" used as parameter name", keyword)
            }
            ParseError::ExpectedStatement => write!(f, "expected a statement"),
            ParseError::ExpectedBlock => write!(f, "expected a block"),
            ParseError::MultipleSameNamedProcs(proc_name) => {
                write!(f, "multiple procedures with name \"{proc_name}\"")
            }
            ParseError::UnexpectedEOF => write!(f, "unexpected end of file"),
            ParseError::ExpectedAssignment => write!(f, "expected an assignment statement"),
            ParseError::ExpectedAssignmentOrInitialization => {
                write!(f, "expected an assignment or initialization statement")
            }
        }
    }
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::UnknownToken(s) => write!(f, "unknown token: {s}"),
            TokenizerError::UnterminatedStringLiteral => {
                write!(f, "unterminated string literal")
            }
            TokenizerError::InvalidEscapeSequence(char) => {
                write!(f, "invalid escape sequence \"\\{char}\"")
            }
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UnknownVariable(id) => write!(f, "unknown variable \"{}\"", id),
            RuntimeError::BadArg(v) => write!(f, "bad argument \"{}\"", v),
            RuntimeError::ArgMismatch(proc_name, expected, actual) => write!(
                f,
                "procedure \"{}\" expected {} args, instead got {}",
                proc_name, expected, actual
            ),
            RuntimeError::ExpectedBool(v) => {
                write!(f, "expected boolean value, instead got {}", v)
            }
            RuntimeError::BadArgs(args) => {
                let arg_strings_vec: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
                let arg_string = arg_strings_vec.join(", ");
                write!(f, "bad args: [{}]", arg_string)
            }
            RuntimeError::BinaryAsUnary(op) => {
                write!(f, "binary operator \"{}\" used as unary operator", op)
            }
            RuntimeError::UnaryAsBinary(op) => {
                write!(f, "unary operator \"{}\" used as binary operator", op)
            }
            RuntimeError::BreakNotInLoop => write!(f, "break statement found outside of a loop"),
            RuntimeError::ContinueNotInLoop => {
                write!(f, "continue statement found outside of a loop")
            }
            RuntimeError::InvalidAssignmentTarget => write!(f, "invalid assignment target"),
            RuntimeError::ReassignConstant(var) => {
                write!(f, "cannot assign to \"{var}\" because it is a constant")
            }
            RuntimeError::ReassignTopLevelProc(proc_name) => {
                write!(f, "cannot assign to top-level procedure \"{proc_name}\"")
            }
            RuntimeError::NotIndexable(value) => write!(f, "\"{value}\" is not indexable"),
            RuntimeError::ExpectedInteger(value) => write!(
                f,
                "expected an integer but got \"{value}\", which is not an integer"
            ),
            RuntimeError::IndexOutOfBounds(index) => write!(f, "index {index} is out of bounds"),
            RuntimeError::ExpectedList(value) => write!(
                f,
                "expected a list, instead got {value}, which is not a list"
            ),
        }
    }
}
