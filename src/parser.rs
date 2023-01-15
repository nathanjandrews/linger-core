use crate::desugar::{desugar_statement, Procedure, Statement};
use crate::tokenizer::AssignOp;
use crate::tokenizer::Operator;
use crate::{
    error::ParseError::{self, *},
    tokenizer::Token as T,
};

use self::procedures::parse_procs;
use self::utils::unexpected_token;

mod expressions;
mod procedures;
mod statements;
mod utils;

/// A representation of a Linger program.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    /// The top-level procedures of the program, excluding the main procedure.
    pub procedures: Vec<Procedure>,
    /// The body of the main procedure of the program.
    pub main: Statement,
}

/// A representation for a procedure in the Linger programming language.
///
/// Structs beginning with the word "Sugared" mean that they are the part of
/// the user-facing syntax of the language. These statements are later
/// ["desugared"](https://en.wikipedia.org/wiki/Syntactic_sugar) (converted) to
/// a subset of the language which is then executed.
#[derive(Debug, PartialEq, Clone)]
pub struct SugaredProcedure {
    pub name: String,
    pub params: Vec<String>,
    pub body: SugaredStatement,
}

/// A representation of a statement in the Linger programming language.
///
/// Structs beginning with the word "Sugared" mean that they are the part of
/// the user-facing syntax of the language. These statements are later
/// ["desugared"](https://en.wikipedia.org/wiki/Syntactic_sugar) (converted) to
/// a subset of the language which is then executed.
#[derive(Clone, Debug, PartialEq)]
pub enum SugaredStatement {
    Expr(SugaredExpr),
    Let(String, SugaredExpr),
    Const(String, SugaredExpr),
    Assign(String, SugaredExpr),
    OperatorAssignment(AssignOp, String, SugaredExpr),
    Block(Vec<SugaredStatement>),
    If(
        SugaredExpr,
        Box<SugaredStatement>,
        Vec<(SugaredExpr, SugaredStatement)>,
        Option<Box<SugaredStatement>>,
    ),
    While(SugaredExpr, Box<SugaredStatement>),
    For(
        Box<SugaredStatement>,
        SugaredExpr,
        Box<SugaredStatement>,
        Vec<SugaredStatement>,
    ),
    Break,
    Continue,
    Return(Option<SugaredExpr>),
}

/// A representation of an expression in the Linger programming language.
///
/// Structs beginning with the word "Sugared" mean that they are the part of
/// the user-facing syntax of the language. These statements are later
/// ["desugared"](https://en.wikipedia.org/wiki/Syntactic_sugar) (converted) to
/// a subset of the language which is then executed.
#[derive(Clone, Debug, PartialEq)]
pub enum SugaredExpr {
    Num(f64),
    Bool(bool),
    Str(String),
    Var(String),
    Binary(Operator, Box<SugaredExpr>, Box<SugaredExpr>),
    Unary(Operator, Box<SugaredExpr>),
    PrimitiveCall(Builtin, Vec<SugaredExpr>),
    Call(Box<SugaredExpr>, Vec<SugaredExpr>),
    Lambda(Vec<String>, Box<SugaredStatement>),
    Index(Box<SugaredExpr>, Box<SugaredExpr>),
}

/// A built in procedure in the Linger programming language.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Builtin {
    Print,
    List,
    IsEmpty,
    IsNil,
}

/// Parses a program from a list of tokens.
pub fn parse_program(tokens: &[T]) -> Result<Program, ParseError> {
    let (procedures, rest) = parse_procs(tokens)?;

    if !rest.is_empty() {
        return Err(unexpected_token(rest)); // extra tokens
    }

    let desugared_procs = procedures.iter().map(|proc| Procedure {
        name: proc.name.to_string(),
        params: proc.params.clone(),
        body: desugar_statement(proc.body.clone()),
    });

    let (main_procs, procs): (Vec<Procedure>, Vec<Procedure>) = desugared_procs
        .into_iter()
        .partition(|proc| proc.name == "main");

    let main_proc = match main_procs.first() {
        Some(proc) => proc,
        None => return Err(NoMain),
    };

    return Ok(Program {
        procedures: procs,
        main: main_proc.body.clone(),
    });
}
