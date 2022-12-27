use crate::{
    error::{unexpected_token, LingerError, LingerError::ParseError, ParseError::*},
    tokenizer::Token as T,
};

#[derive(Debug)]
pub struct Program<'a> {
    pub procedures: Vec<Procedure<'a>>,
    pub main: Statements<'a>,
}

#[derive(Debug)]
pub struct Procedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Statements<'a>,
}

type Statements<'a> = Vec<Statement<'a>>;

#[derive(Clone, Debug)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Let(&'a str, Expr<'a>),
    Return(Expr<'a>),
}

#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Num(i64),
    Bool(bool),
    Var(&'a str),
    Binary(BinaryOperator, Box<Expr<'a>>, Box<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Eq,
}

fn consume_token<'a>(target: T<'a>, tokens: &'a [T<'a>]) -> Result<&'a [T<'a>], LingerError<'a>> {
    println!("consuming token... {target}");
    match tokens {
        [token, rest @ ..] if token.eq(&target) => Ok(rest),
        tokens => Err(unexpected_token(tokens)),
    }
}

fn binary_expression<'a>(
    op: BinaryOperator,
    first_arg: Expr<'a>,
    second_arg: Expr<'a>,
) -> Expr<'a> {
    Expr::Binary(op, Box::new(first_arg), Box::new(second_arg))
}

pub fn parse_program<'a>(tokens: &'a [T<'a>]) -> Result<Program<'a>, LingerError> {
    let (procedures, rest) = match parse_procs(tokens) {
        Ok((procs, tokens)) => (procs, tokens),
        Err(e) => return Err(e),
    };
    if !rest.is_empty() {
        return Err(unexpected_token(rest)); // extra tokens
    }

    let (main_procs, procs): (Vec<Procedure>, Vec<Procedure>) =
        procedures.into_iter().partition(|proc| proc.name == "main");

    if main_procs.len() == 0 {
        return Err(ParseError(NoMain)); // more than one main function
    }

    let main_proc = main_procs.first().unwrap();

    return Ok(Program {
        procedures: procs,
        main: main_proc.body.to_vec(),
    });
}

fn parse_procs<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Procedure<'a>>, &'a [T<'a>]), LingerError> {
    let (proc_option, tokens) = match parse_proc(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    match (proc_option, tokens.len()) {
        (Some(proc), 0) => Ok((vec![proc], tokens)),
        (Some(proc), _) => {
            match parse_procs(tokens) {
                Ok((mut rest_procs, tokens)) => {
                    let mut vec = vec![proc];
                    vec.append(&mut rest_procs);
                    return Ok((vec, tokens));
                }
                Err(e) => return Err(e),
            };
        }
        (None, 0) => Err(ParseError(NoMain)),
        (None, _) => Err(unexpected_token(tokens)),
    }
}

fn parse_proc<'a>(tokens: &'a [T<'a>]) -> Result<(Option<Procedure<'a>>, &[T<'a>]), LingerError> {
    match tokens {
        [T::ID("proc"), T::ID(name), T::LPAREN, rest @ ..] => {
            let (params, tokens) = match parse_params(rest) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let tokens = match consume_token(T::LBRACKET, tokens) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            let (body_statements, tokens) = match parse_statements(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            Ok((
                Some(Procedure {
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
        [T::RPAREN, rest @ ..] => Ok((vec![], rest)),
        [T::ID(param_name), rest_toks @ ..] => {
            let (mut rest_params, rest_toks) = match parse_params(rest_toks) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let mut params = vec![*param_name];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_statements<'a>(tokens: &'a [T<'a>]) -> Result<(Statements, &[T<'a>]), LingerError> {
    let (statement_option, tokens) = match parse_statement(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    let statement = if statement_option.is_some() {
        statement_option.unwrap()
    } else {
        return Ok((vec![], tokens));
    };

    match tokens {
        [T::SEMICOLON, tokens @ ..] => {
            let (mut rest_statements, tokens) = match parse_statements(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let mut vec = vec![statement];
            vec.append(&mut rest_statements);
            Ok((vec, tokens))
        }
        _ => return Err(ParseError(MissingSemicolon)),
    }
}

fn parse_statement<'a>(tokens: &'a [T<'a>]) -> Result<(Option<Statement>, &[T<'a>]), LingerError> {
    match tokens {
        [T::RBRACKET, tokens @ ..] => Ok((None, tokens)),
        [T::ID("let"), T::ID(var_name), T::ASSIGN, tokens @ ..] => {
            let (var_expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            Ok((Some(Statement::Let(&var_name, var_expr)), tokens))
        }
        [T::ID("return"), tokens @ ..] => {
            let (return_expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            Ok((Some(Statement::Return(return_expr)), tokens))
        }
        tokens => match parse_expr(tokens) {
            Ok((expr, tokens)) => Ok((Some(Statement::Expr(expr)), tokens)),
            Err(e) => return Err(e),
        },
    }
}

fn parse_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (additive_expr, tokens) = match parse_additive_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };
    match tokens {
        [T::EQ, tokens @ ..] => {
            let (relational_expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Eq, additive_expr, relational_expr),
                tokens,
            ));
        }
        tokens => {
            return Ok((additive_expr, tokens));
        }
    }
}

fn parse_additive_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    let (terminal, tokens) = match parse_terminal_expr(tokens) {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };

    match tokens {
        [T::PLUS, tokens @ ..] => {
            let (additive_expr, tokens) = match parse_additive_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Plus, terminal, additive_expr),
                tokens,
            ));
        }
        [T::MINUS, tokens @ ..] => {
            let (additive_expr, tokens) = match parse_additive_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((
                binary_expression(BinaryOperator::Minus, terminal, additive_expr),
                tokens,
            ));
        }
        tokens => {
            return Ok((terminal, tokens));
        }
    }
}

fn parse_terminal_expr<'a>(tokens: &'a [T<'a>]) -> Result<(Expr, &'a [T<'a>]), LingerError> {
    match tokens {
        [T::ID(function_name), T::LPAREN, tokens @ ..] => {
            let (args, tokens) = match parse_args(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            return Ok((Expr::Call(*function_name, args), tokens));
        }
        [T::ID(id), tokens @ ..] => match *id {
            "true" => Ok((Expr::Bool(true), tokens)),
            "false" => Ok((Expr::Bool(false), tokens)),
            _ => Ok((Expr::Var(id), tokens)),
        },
        [T::LPAREN, tokens @ ..] => {
            let (expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let tokens = match consume_token(T::RPAREN, tokens) {
                Ok(tokens) => tokens,
                Err(e) => return Err(e),
            };
            Ok((expr, tokens))
        }
        [T::NUM(n), tokens @ ..] => Ok((Expr::Num(*n), tokens)),
        tokens => Err(unexpected_token(tokens)),
    }
}

fn parse_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Expr>, &'a [T<'a>]), LingerError> {
    match tokens {
        [T::RPAREN, tokens @ ..] => Ok((vec![], tokens)),
        tokens => {
            let (expr, tokens) = match parse_expr(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };
            let (mut rest_args, tokens) = match parse_rest_args(tokens) {
                Ok(pair) => pair,
                Err(e) => return Err(e),
            };

            let mut vec = vec![expr];
            vec.append(&mut rest_args);
            return Ok((vec, tokens));
        }
    }
}

fn parse_rest_args<'a>(tokens: &'a [T<'a>]) -> Result<(Vec<Expr>, &'a [T<'a>]), LingerError> {
    match tokens {
        [T::RPAREN, tokens @ ..] => Ok((vec![], tokens)),
        [T::COMMA, T::RPAREN, ..] => Err(unexpected_token(tokens)),
        [T::COMMA, tokens @ ..] => parse_args(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}
