use std::vec;

use crate::desugar::{desugar_statements, Procedure, Statement};
use crate::tokenizer::Operator::{self, *};
use crate::{
    error::{LingerError, LingerError::ParseError, ParseError::*},
    tokenizer::{
        Token as T,
        TokenValue::{self, *},
    },
    KEYWORDS,
};

/// A representation of a Linger program.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program<'a> {
    /// The top-level procedures of the program, excluding the main procedure.
    pub procedures: Vec<Procedure<'a>>,
    /// The body of the main procedure of the program.
    pub main: Vec<Statement<'a>>,
}

/// A representation for a procedure in the Linger programming language.
///
/// Structs beginning with the word "Sugared" mean that they are the part of
/// the user-facing syntax of the language. These statements are later
/// ["desugared"](https://en.wikipedia.org/wiki/Syntactic_sugar) (converted) to
/// a subset of the language which is then executed.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SugaredProcedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Vec<SugaredStatement<'a>>,
}

/// A representation of a statement in the Linger programming language.
///
/// Structs beginning with the word "Sugared" mean that they are the part of
/// the user-facing syntax of the language. These statements are later
/// ["desugared"](https://en.wikipedia.org/wiki/Syntactic_sugar) (converted) to
/// a subset of the language which is then executed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SugaredStatement<'a> {
    Expr(SugaredExpr<'a>),
    Let(&'a str, SugaredExpr<'a>),
    Assign(&'a str, SugaredExpr<'a>),
    Block(Vec<SugaredStatement<'a>>),
    If(
        SugaredExpr<'a>,
        Box<SugaredStatement<'a>>,
        Vec<(SugaredExpr<'a>, SugaredStatement<'a>)>,
        Option<Box<SugaredStatement<'a>>>,
    ),
    While(SugaredExpr<'a>, Box<SugaredStatement<'a>>),
    For(
        Box<SugaredStatement<'a>>,
        SugaredExpr<'a>,
        Box<SugaredStatement<'a>>,
        Vec<SugaredStatement<'a>>,
    ),
    Break,
    Continue,
    Return(Option<SugaredExpr<'a>>),
}

/// A representation of an expression in the Linger programming language.
///
/// Structs beginning with the word "Sugared" mean that they are the part of
/// the user-facing syntax of the language. These statements are later
/// ["desugared"](https://en.wikipedia.org/wiki/Syntactic_sugar) (converted) to
/// a subset of the language which is then executed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SugaredExpr<'a> {
    Num(i64),
    Bool(bool),
    Str(String),
    Var(&'a str),
    Binary(Operator, Box<SugaredExpr<'a>>, Box<SugaredExpr<'a>>),
    Unary(Operator, Box<SugaredExpr<'a>>),
    PrimitiveCall(Builtin, Vec<SugaredExpr<'a>>),
    Call(Box<SugaredExpr<'a>>, Vec<SugaredExpr<'a>>),
    Lambda(Vec<&'a str>, Vec<SugaredStatement<'a>>),
}

/// A built in procedure in the Linger programming language.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Builtin {
    Print,
}

/// A helper function to create an [UnexpectedToken Error](UnexpectedToken).
fn unexpected_token<'a>(tokens: &'a [T<'a>]) -> LingerError<'a> {
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
fn consume_token<'a>(
    target: TokenValue<'a>,
    tokens: &'a [T<'a>],
) -> Result<&'a [T<'a>], LingerError<'a>> {
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
fn conditionally_consume_semicolon<'a>(
    tokens: &'a [T<'a>],
    should_consume: bool,
) -> Result<&'a [T<'a>], LingerError<'a>> {
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
fn match_operator<'a>(
    operators: &[Operator],
    tokens: &'a [T<'a>],
) -> Option<(Operator, &'a [T<'a>])> {
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
type BinaryExpressionParser<'a> =
    fn(&'a [T<'a>]) -> Result<(SugaredExpr<'a>, &'a [T<'a>]), LingerError<'a>>;

/// A helper function for parsing binary expressions.
fn parse_binary_expr<'a>(
    parse_expr: BinaryExpressionParser<'a>,
    operators: Vec<Operator>,
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr<'a>, &'a [T<'a>]), LingerError<'a>> {
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
fn binary_expression<'a>(
    op: Operator,
    first_arg: SugaredExpr<'a>,
    second_arg: SugaredExpr<'a>,
) -> SugaredExpr<'a> {
    SugaredExpr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

/// Ensures that `statement_option` is a Some variant which contains a
/// [Block Statement](SugaredStatement::Block). Otherwise, this function returns
/// an [ExpectedBlock] parse error.
fn ensure_block<'a>(
    statement_option: Option<SugaredStatement<'a>>,
) -> Result<SugaredStatement, LingerError> {
    match statement_option {
        Some(statement) => match statement {
            SugaredStatement::Block(_) => Ok(statement),
            _ => Err(ParseError(ExpectedBlock)),
        },
        None => Err(ParseError(ExpectedBlock)),
    }
}

/// Parses a program from a list of tokens.
pub fn parse_program<'a>(tokens: &'a [T<'a>]) -> Result<Program<'a>, LingerError> {
    let (procedures, rest) = parse_procs(tokens)?;

    if !rest.is_empty() {
        return Err(unexpected_token(rest)); // extra tokens
    }

    let desugared_procs = procedures.iter().map(|proc| Procedure {
        name: proc.name,
        params: proc.params.clone(),
        body: desugar_statements(proc.body.clone()),
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
        main: main_proc.body.to_vec(),
    });
}

fn parse_procs<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Vec<SugaredProcedure<'a>>, &'a [T<'a>]), LingerError> {
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

fn parse_proc<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Option<SugaredProcedure<'a>>, &[T<'a>]), LingerError> {
    match tokens {
        [T(ID("proc"), ..), T(ID(name), ..), T(LPAREN, ..), rest @ ..] => {
            if KEYWORDS.contains(name) {
                return Err(ParseError(KeywordAsProc(name)));
            }

            let (params, tokens) = parse_params(rest)?;

            let tokens = consume_token(LBRACKET, tokens)?;

            let (body_statements, tokens) = parse_statements(tokens)?;

            Ok((
                Some(SugaredProcedure {
                    name,
                    params,
                    body: body_statements,
                }),
                tokens,
            ))
        }
        _ => Ok((None, tokens)),
    }
}

fn parse_params<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<&'a str>, &[T<'a>]), LingerError> {
    match tokens {
        [T(RPAREN, ..), rest @ ..] => Ok((vec![], rest)),
        [T(ID(param_name), ..), rest_toks @ ..] => {
            if KEYWORDS.contains(param_name) {
                return Err(ParseError(KeywordAsParam(param_name)));
            }

            let (mut rest_params, rest_toks) = parse_params(rest_toks)?;
            let mut params = vec![*param_name];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_statements<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Vec<SugaredStatement<'a>>, &[T<'a>]), LingerError> {
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

fn parse_statement<'a>(
    tokens: &'a [T<'a>],
    parse_semicolon: bool,
) -> Result<(Option<SugaredStatement>, &[T<'a>]), LingerError> {
    match tokens {
        [T(RBRACKET, ..), tokens @ ..] => Ok((None, tokens)),
        [T(ID("let"), ..), T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            if KEYWORDS.contains(var_name) {
                return Err(ParseError(KeywordAsVar(var_name)));
            }

            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((Some(SugaredStatement::Let(&var_name, var_expr)), tokens))
        }
        [T(ID(var_name), ..), T(ASSIGN, ..), tokens @ ..] => {
            if KEYWORDS.contains(var_name) {
                return Err(ParseError(KeywordAsVar(var_name)));
            }

            let (var_expr, tokens) = parse_expr(tokens)?;

            let tokens = conditionally_consume_semicolon(tokens, parse_semicolon)?;

            Ok((Some(SugaredStatement::Assign(&var_name, var_expr)), tokens))
        }
        [T(ID("if"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (cond_expr, tokens) = parse_expr(tokens)?;
            let tokens = consume_token(RPAREN, tokens)?;
            let (then_block_option, mut tokens) = parse_statement(tokens, true)?;
            let then_block = ensure_block(then_block_option)?;

            let mut else_ifs = vec![];
            loop {
                match tokens {
                    [T(ID("else"), ..), T(ID("if"), ..), T(LPAREN, ..), rest @ ..] => {
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
                [T(ID("else"), ..), tokens @ ..] => {
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
        [T(ID("while"), ..), T(LPAREN, ..), tokens @ ..] => {
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
        [T(ID("for"), ..), T(LPAREN, ..), tokens @ ..] => {
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
        [T(ID("return"), ..), T(SEMICOLON, ..), tokens @ ..] => {
            Ok((Some(SugaredStatement::Return(None)), tokens))
        }
        [T(ID("return"), ..), tokens @ ..] => {
            let (return_expr, tokens) = parse_expr(tokens)?;
            let tokens = conditionally_consume_semicolon(tokens, true)?;
            Ok((Some(SugaredStatement::Return(Some(return_expr))), tokens))
        }
        [T(ID("break"), ..), tokens @ ..] => {
            let tokens = conditionally_consume_semicolon(tokens, true)?;
            Ok((Some(SugaredStatement::Break), tokens))
        }
        [T(ID("continue"), ..), tokens @ ..] => {
            let tokens = conditionally_consume_semicolon(tokens, true)?;
            Ok((Some(SugaredStatement::Continue), tokens))
        }
        [T(LBRACKET, ..), tokens @ ..] => {
            let (statements, tokens) = parse_statements(tokens)?;
            Ok((Some(SugaredStatement::Block(statements)), tokens))
        }
        tokens => match parse_expr(tokens) {
            Ok((expr, tokens)) => {
                let tokens = conditionally_consume_semicolon(tokens, true)?;
                Ok((Some(SugaredStatement::Expr(expr)), tokens))
            }
            Err(e) => return Err(e),
        },
    }
}

fn parse_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    parse_logical_or_expr(tokens)
}

fn parse_logical_or_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_logical_and_expr, vec![LogicOr], tokens);
}

fn parse_logical_and_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_equality_expr, vec![LogicAnd], tokens);
}

fn parse_equality_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_relational_expr, vec![Eq, Ne], tokens);
}

fn parse_relational_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_additive_expr, vec![LT, GT, LTE, GTE], tokens);
}

fn parse_additive_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_multiplicative_expr, vec![Plus, Minus], tokens);
}

fn parse_multiplicative_expr<'a>(
    tokens: &'a [T<'a>],
) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    return parse_binary_expr(parse_unary_expr, vec![Times, Mod, Div], tokens);
}

fn parse_unary_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    match match_operator(vec![Minus, LogicNot].as_slice(), tokens) {
        Some((op, tokens)) => {
            let (operand, tokens) = parse_terminal_expr(tokens)?;
            return Ok((SugaredExpr::Unary(op, Box::new(operand)), tokens));
        }
        None => return parse_terminal_expr(tokens),
    }
}

fn parse_terminal_expr<'a>(tokens: &'a [T<'a>]) -> Result<(SugaredExpr, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(ID("lam"), ..), T(LPAREN, ..), tokens @ ..] => {
            let (params, tokens) = parse_params(tokens)?;
            let tokens = consume_token(THIN_ARROW, tokens)?;
            let tokens = consume_token(LBRACKET, tokens)?;
            let (lambda_body, tokens) = parse_statements(tokens)?;
            return Ok((SugaredExpr::Lambda(params, lambda_body), tokens));
        }
        [T(ID(proc_name), ..), T(LPAREN, ..), tokens @ ..] => {
            let (args, tokens) = parse_args(tokens)?;

            let expr = match check_builtin(proc_name) {
                Some(builtin) => SugaredExpr::PrimitiveCall(builtin, args),
                None => SugaredExpr::Call(Box::new(SugaredExpr::Var(proc_name)), args),
            };

            return Ok((expr, tokens));
        }
        [T(STR(s), ..), tokens @ ..] => Ok((SugaredExpr::Str(s.to_string()), tokens)),
        [T(ID(id), ..), tokens @ ..] => match *id {
            "true" => Ok((SugaredExpr::Bool(true), tokens)),
            "false" => Ok((SugaredExpr::Bool(false), tokens)),
            _ => {
                if KEYWORDS.contains(id) {
                    Err(LingerError::ParseError(KeywordAsVar(id)))
                } else {
                    Ok((SugaredExpr::Var(id), tokens))
                }
            }
        },
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

fn parse_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<SugaredExpr>, &'a [T<'a>]), LingerError> {
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

fn parse_rest_args<'a>(
    tokens: &'a [T<'a>],
) -> Result<(Vec<SugaredExpr>, &'a [T<'a>]), LingerError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        [T(COMMA, ..), T(RPAREN, ..), ..] => Err(unexpected_token(tokens)),
        [T(COMMA, ..), tokens @ ..] => parse_args(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}
