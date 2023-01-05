use std::vec;

use crate::desugar::{desugar_statement, Procedure, Statement};
use crate::tokenizer::Operator::{self, *};
use crate::{
    error::{LingerError, LingerError::ParseError, ParseError::*},
    tokenizer::{
        Keyword::*,
        Token as T,
        TokenValue::{self, *},
    },
};

/// A representation of a Linger program.
#[derive(Debug, Eq, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
struct SugaredProcedure {
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SugaredStatement {
    Expr(SugaredExpr),
    Let(String, SugaredExpr),
    Assign(String, SugaredExpr),
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SugaredExpr {
    Num(i64),
    Bool(bool),
    Str(String),
    Var(String),
    Binary(Operator, Box<SugaredExpr>, Box<SugaredExpr>),
    Unary(Operator, Box<SugaredExpr>),
    PrimitiveCall(Builtin, Vec<SugaredExpr>),
    Call(Box<SugaredExpr>, Vec<SugaredExpr>),
    Lambda(Vec<String>, Box<SugaredStatement>),
}

/// A built in procedure in the Linger programming language.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Builtin {
    Print,
}

/// Parses a program from a list of tokens.
pub fn parse_program(tokens: &[T]) -> Result<Program, LingerError> {
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
        None => return Err(ParseError(NoMain)),
    };

    return Ok(Program {
        procedures: procs,
        main: main_proc.body.clone(),
    });
}

fn parse_procs(tokens: &[T]) -> Result<(Vec<SugaredProcedure>, &[T]), LingerError> {
    let (proc_option, tokens) = parse_proc(tokens)?;

    match proc_option {
        Some(proc) => {
            let (mut rest_procs, tokens) = parse_procs(tokens)?;

            if rest_procs
                .clone()
                .iter()
                .find(|p| p.name == proc.name)
                .is_some()
            {
                return Err(ParseError(MultipleSameNamedProcs(proc.name.to_string())));
            }

            let mut vec = vec![proc];
            vec.append(&mut rest_procs);
            return Ok((vec, tokens));
        }
        None => Ok((vec![], tokens)),
    }
}

fn parse_proc(tokens: &[T]) -> Result<(Option<SugaredProcedure>, &[T]), LingerError> {
    match tokens {
        [T(KW(Proc), ..), T(KW(kw), ..), T(LPAREN, ..), ..] => {
            Err(ParseError(KeywordAsProc(kw.to_string())))
        }
        [T(KW(Proc), ..), T(ID(name), ..), T(LPAREN, ..), rest @ ..] => {
            let (params, tokens) = parse_params(rest)?;

            let (body_block_option, tokens) = parse_statement(tokens, true)?;
            let body_block = ensure_block(body_block_option)?;

            Ok((
                Some(SugaredProcedure {
                    name: name.to_string(),
                    params,
                    body: body_block,
                }),
                tokens,
            ))
        }
        _ => Ok((None, tokens)),
    }
}

fn parse_params(tokens: &[T]) -> Result<(Vec<String>, &[T]), LingerError> {
    match tokens {
        [T(RPAREN, ..), rest @ ..] => Ok((vec![], rest)),
        [T(KW(kw), ..), ..] => Err(ParseError(KeywordAsParam(kw.to_string()))),
        [T(ID(param_name), ..), rest_toks @ ..] => {
            let (mut rest_params, rest_toks) = parse_params(rest_toks)?;
            let mut params = vec![param_name.to_string()];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_statements(tokens: &[T]) -> Result<(Vec<SugaredStatement>, &[T]), LingerError> {
    let (statement_option, tokens) = parse_statement(tokens, true)?;

    let statement = match statement_option {
        Some(statement) => statement,
        None => return Ok((vec![], tokens)),
    };

    let (mut rest_statements, tokens) = parse_statements(tokens)?;
    let mut vec = vec![statement];
    vec.append(&mut rest_statements);
    Ok((vec, tokens))
}

fn parse_statement(
    tokens: &[T],
    parse_semicolon: bool,
) -> Result<(Option<SugaredStatement>, &[T]), LingerError> {
    match tokens {
        [T(RBRACKET, ..), tokens @ ..] => Ok((None, tokens)),
        [T(KW(Let), ..), T(KW(kw), ..), ..] => Err(ParseError(KeywordAsVar(kw.to_string()))),
        [T(KW(Let), ..), T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((
                Some(SugaredStatement::Let(var_name.to_string(), var_expr)),
                tokens,
            ))
        }
        [T(KW(kw), ..), T(ASSIGN, ..), ..] => Err(ParseError(KeywordAsVar(kw.to_string()))),
        [T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((
                Some(SugaredStatement::Assign(var_name.to_string(), var_expr)),
                tokens,
            ))
        }
        [T(KW(If), ..), T(LPAREN, ..), tokens @ ..] => {
            let (cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            let (then_block_option, mut tokens) = parse_statement(tokens, true)?;
            let then_block = ensure_block(then_block_option)?;

            let mut else_ifs = vec![];
            loop {
                match tokens {
                    [T(KW(Else), ..), T(KW(If), ..), T(LPAREN, ..), rest @ ..] => {
                        let (else_if_cond, rest) = parse_expr(rest)?;
                        let rest = consume_token(RPAREN, rest)?;
                        let (else_if_block_option, rest) = parse_statement(rest, true)?;
                        let else_if_block = ensure_block(else_if_block_option)?;
                        else_ifs.push((else_if_cond, else_if_block));
                        tokens = rest;
                    }
                    _ => break,
                }
            }

            let (else_block_option, tokens) = match tokens {
                [T(KW(Else), ..), tokens @ ..] => {
                    let (else_block, tokens) = parse_statement(tokens, true)?;
                    let else_block = ensure_block(else_block)?;
                    (Some(Box::new(else_block)), tokens)
                }
                tokens => (None, tokens),
            };

            Ok((
                Some(SugaredStatement::If(
                    cond_expr,
                    Box::new(then_block),
                    else_ifs,
                    else_block_option,
                )),
                tokens,
            ))
        }
        [T(KW(While), ..), T(LPAREN, ..), tokens @ ..] => {
            let (while_cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            let (while_block_option, tokens) = parse_statement(tokens, true)?;
            let while_block = ensure_block(while_block_option)?;

            Ok((
                Some(SugaredStatement::While(
                    while_cond_expr,
                    Box::new(while_block),
                )),
                tokens,
            ))
        }
        [T(KW(For), ..), T(LPAREN, ..), tokens @ ..] => {
            let (var_statement_option, tokens) = parse_statement(tokens, true)?;
            let var_statement = match var_statement_option {
                Some(statement) => match statement {
                    s @ (SugaredStatement::Let(_, _) | SugaredStatement::Assign(_, _)) => s,
                    _ => {
                        return Err(ParseError(Custom(
                            "expected variable assignment or initialization".to_string(),
                        )))
                    }
                },
                None => return Err(ParseError(ExpectedStatement)),
            };
            let (stop_cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            let (reassign_statement_option, tokens) = parse_statement(tokens, false)?;
            let reassign_statement = match reassign_statement_option {
                Some(statement) => match statement {
                    s @ SugaredStatement::Assign(_, _) => s,
                    _ => {
                        return Err(ParseError(Custom(
                            "expected variable assignment".to_string(),
                        )))
                    }
                },
                None => return Err(ParseError(ExpectedStatement)),
            };
            let tokens = consume_token(RPAREN, tokens)?;

            let (for_block_option, tokens) = parse_statement(tokens, true)?;
            let for_block_statements = match for_block_option {
                Some(statement) => match statement {
                    SugaredStatement::Block(statements) => statements,
                    _ => return Err(ParseError(ExpectedStatement)),
                },
                None => return Err(ParseError(ExpectedStatement)),
            };

            return Ok((
                Some(SugaredStatement::For(
                    Box::new(var_statement),
                    stop_cond_expr,
                    Box::new(reassign_statement),
                    for_block_statements,
                )),
                tokens,
            ));
        }
        [T(KW(Return), ..), T(SEMICOLON, ..), tokens @ ..] => {
            Ok((Some(SugaredStatement::Return(None)), tokens))
        }
        [T(KW(Return), ..), tokens @ ..] => {
            let (return_expr, tokens) = parse_expr(tokens)?;
            let tokens = conditionally_consume_semicolon(tokens, true)?;
            Ok((Some(SugaredStatement::Return(Some(return_expr))), tokens))
        }
        [T(KW(Break), ..), tokens @ ..] => {
            let tokens = conditionally_consume_semicolon(tokens, true)?;
            Ok((Some(SugaredStatement::Break), tokens))
        }
        [T(KW(Continue), ..), tokens @ ..] => {
            let tokens = conditionally_consume_semicolon(tokens, true)?;
            Ok((Some(SugaredStatement::Continue), tokens))
        }
        [T(LBRACKET, ..), tokens @ ..] => {
            let (statements, tokens) = parse_statements(tokens)?;
            Ok((Some(SugaredStatement::Block(statements)), tokens))
        }
        tokens => match parse_expr(tokens)? {
            (expr, tokens) => {
                let tokens = conditionally_consume_semicolon(tokens, true)?;
                Ok((Some(SugaredStatement::Expr(expr)), tokens))
            }
        },
    }
}

fn parse_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    parse_logical_or_expr(tokens)
}

fn parse_logical_or_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    return parse_binary_expr(parse_logical_and_expr, vec![LogicOr], tokens);
}

fn parse_logical_and_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    return parse_binary_expr(parse_equality_expr, vec![LogicAnd], tokens);
}

fn parse_equality_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    return parse_binary_expr(parse_relational_expr, vec![Eq, Ne], tokens);
}

fn parse_relational_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    return parse_binary_expr(parse_additive_expr, vec![LT, GT, LTE, GTE], tokens);
}

fn parse_additive_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    return parse_binary_expr(parse_multiplicative_expr, vec![Plus, Minus], tokens);
}

fn parse_multiplicative_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    return parse_binary_expr(parse_unary_expr, vec![Times, Mod, Div], tokens);
}

fn parse_unary_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    let (op_option, tokens) = match tokens {
        [T(OP(Minus), ..), tokens @ ..] => (Some(Minus), tokens),
        [T(OP(LogicNot), ..), tokens @ ..] => (Some(LogicNot), tokens),
        [T(DOUBLE_PLUS, ..), tokens @ ..] => (Some(PreIncrement), tokens),
        tokens => (None, tokens),
    };

    let (terminal_expr, tokens) = parse_terminal_expr(tokens)?;

    match op_option {
        Some(op) => return Ok((SugaredExpr::Unary(op, Box::new(terminal_expr)), tokens)),
        None => match tokens {
            [T(DOUBLE_PLUS, ..), tokens @ ..] => {
                return Ok((
                    SugaredExpr::Unary(PostIncrement, Box::new(terminal_expr)),
                    tokens,
                ))
            }
            tokens => return Ok((terminal_expr, tokens)),
        },
    }
}

fn parse_terminal_expr(tokens: &[T]) -> Result<(SugaredExpr, &[T]), LingerError> {
    match tokens {
        [T(KW(Lam), ..), T(LPAREN, ..), tokens @ ..] => {
            let (params, tokens) = parse_params(tokens)?;
            let tokens = consume_token(THIN_ARROW, tokens)?;

            let (lambda_body_option, tokens) = parse_statement(tokens, true)?;
            let lambda_body = ensure_block(lambda_body_option)?;

            return Ok((SugaredExpr::Lambda(params, Box::new(lambda_body)), tokens));
        }
        [T(ID(proc_name), ..), T(LPAREN, ..), tokens @ ..] => {
            let (args, tokens) = parse_args(tokens)?;

            let expr = match check_builtin(proc_name) {
                Some(builtin) => SugaredExpr::PrimitiveCall(builtin, args),
                None => SugaredExpr::Call(Box::new(SugaredExpr::Var(proc_name.to_string())), args),
            };

            return Ok((expr, tokens));
        }
        [T(STR(s), ..), tokens @ ..] => Ok((SugaredExpr::Str(s.to_string()), tokens)),
        [T(KW(True), ..), tokens @ ..] => Ok((SugaredExpr::Bool(true), tokens)),
        [T(KW(False), ..), tokens @ ..] => Ok((SugaredExpr::Bool(false), tokens)),
        [T(KW(kw), ..), ..] => Err(ParseError(KeywordAsVar(kw.to_string()))),
        [T(ID(id), ..), tokens @ ..] => Ok((SugaredExpr::Var(id.to_string()), tokens)),
        [T(LPAREN, ..), tokens @ ..] => {
            let (expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            match expr {
                SugaredExpr::Lambda(params, body) => {
                    // expect an immediately invoked function, parse the arguments and return the call
                    let tokens = consume_token(LPAREN, tokens)?;
                    let (args, tokens) = parse_args(tokens)?;
                    Ok((
                        SugaredExpr::Call(Box::new(SugaredExpr::Lambda(params, body)), args),
                        tokens,
                    ))
                }
                expr => Ok((expr, tokens)),
            }
        }
        [T(NUM(n), ..), tokens @ ..] => Ok((SugaredExpr::Num(*n), tokens)),
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_args(tokens: &[T]) -> Result<(Vec<SugaredExpr>, &[T]), LingerError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        tokens => {
            let (expr, tokens) = parse_expr(tokens)?;
            let (mut rest_args, tokens) = parse_rest_args(tokens)?;

            let mut vec = vec![expr];
            vec.append(&mut rest_args);
            return Ok((vec, tokens));
        }
    }
}

fn parse_rest_args(tokens: &[T]) -> Result<(Vec<SugaredExpr>, &[T]), LingerError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        [T(COMMA, ..), T(RPAREN, ..), ..] => Err(unexpected_token(tokens)),
        [T(COMMA, ..), tokens @ ..] => parse_args(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}

//////////////////////////////////////////////////////////////////////////
// Helper Functions
//////////////////////////////////////////////////////////////////////////

/// A helper function to create an [UnexpectedToken Error](UnexpectedToken).
fn unexpected_token(tokens: &[T]) -> LingerError {
    return LingerError::ParseError(UnexpectedToken(tokens.first().unwrap().to_owned()));
}

/// A helper function to check if `s` matches one of the [Builtin] procedures.
pub fn check_builtin(s: &str) -> Option<Builtin> {
    match s {
        "print" => Some(Builtin::Print),
        _ => None,
    }
}

/// Tries to consume a token with a [TokenValue] of `target` from the front of `tokens`. On success,
/// this function returns `tokens` with the first element removed. On failure, this function returns
/// an [Expected] error.
fn consume_token(target: TokenValue, tokens: &[T]) -> Result<&[T], LingerError> {
    match tokens {
        [token, rest @ ..] if token.0.eq(&target) => Ok(rest),
        [token, ..] => Err(ParseError(Expected(target, token.clone()))),
        _ => unreachable!(),
    }
}

/// This function conditionally tries to consume a [SEMICOLON] token if `should_consume` is true.
/// If `should_consume` is true, then this function returns the result of [consume_token] with a
/// `target` of [SEMICOLON]. If `should_consume` is false, then this function returns the `tokens`
/// list unmodified.
fn conditionally_consume_semicolon(
    tokens: &[T],
    should_consume: bool,
) -> Result<&[T], LingerError> {
    if should_consume {
        return consume_token(SEMICOLON, tokens);
    } else {
        return Ok(tokens);
    }
}

/// This function tries to consume an [OP] token with an associated [Operator] found in `operators`.
/// If such a token is successfully consumed, this function returns the token's operator and the
/// list of tokens that comes after as a pair. If `tokens` does not start with such an operator,
/// then this function returns `None`.
fn match_operator<'a>(operators: &[Operator], tokens: &'a [T]) -> Option<(Operator, &'a [T])> {
    match tokens {
        [T(value, ..), rest @ ..] => match value {
            OP(b) => {
                if operators.contains(b) {
                    return Some((*b, rest));
                } else {
                    return None;
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// Type alias for the return value of a binary expression parsing function.
type BinaryExpressionParser = fn(&[T]) -> Result<(SugaredExpr, &[T]), LingerError>;

/// A helper function for parsing binary expressions.
fn parse_binary_expr(
    parse_expr: BinaryExpressionParser,
    operators: Vec<Operator>,
    tokens: &[T],
) -> Result<(SugaredExpr, &[T]), LingerError> {
    let (mut expr, mut tokens) = parse_expr(tokens)?;
    loop {
        match match_operator(operators.as_slice(), tokens) {
            Some((op, rest)) => {
                let (right, rest) = parse_expr(rest)?;
                expr = binary_expression(op, expr, right);
                tokens = rest;
            }
            None => return Ok((expr, tokens)),
        }
    }
}

/// A helper function for creating a [Binary Expression](SugaredExpr::Binary)
fn binary_expression(op: Operator, first_arg: SugaredExpr, second_arg: SugaredExpr) -> SugaredExpr {
    SugaredExpr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

/// Ensures that `statement_option` is a Some variant which contains a
/// [Block Statement](SugaredStatement::Block). Otherwise, this function returns
/// an [ExpectedBlock] parse error.
fn ensure_block(
    statement_option: Option<SugaredStatement>,
) -> Result<SugaredStatement, LingerError> {
    match statement_option {
        Some(statement) => match statement {
            SugaredStatement::Block(_) => Ok(statement),
            _ => Err(ParseError(ExpectedBlock)),
        },
        None => Err(ParseError(ExpectedBlock)),
    }
}
