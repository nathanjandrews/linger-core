#[allow(unused_imports)]
use crate::{
    parser::{parse_program, Expr::*, Program, Statement, Builtin::*},
    tokenizer::{
        Token,
        TokenValue::{self, *},
    },
};

#[allow(dead_code)]
fn tokens_from_values(values: Vec<TokenValue>) -> Vec<Token> {
    values
        .into_iter()
        .map(|value| Token(value, 0, 0))
        .collect::<Vec<Token>>()
}

#[allow(dead_code)]
fn parse_success(values: Vec<TokenValue>, expected: Program) -> bool {
    let tokens = tokens_from_values(values);
    match parse_program(tokens.as_slice()) {
        Ok(program) => program.eq(&expected),
        Err(_) => false,
    }
}

#[allow(dead_code)]
fn parse_mismatch(values: Vec<TokenValue>, expected: Program) -> bool {
    let tokens = tokens_from_values(values);
    match parse_program(tokens.as_slice()) {
        Ok(program) => program.ne(&expected),
        Err(_) => false,
    }
}

#[allow(dead_code)]
fn parse_error(values: Vec<TokenValue>) -> bool {
    let tokens = tokens_from_values(values);
    match parse_program(tokens.as_slice()) {
        Ok(_) => false,
        Err(_) => true,
    }
}

#[test]
fn print_ten() {
    assert!(parse_success(
        vec![
            ID("proc"),
            ID("main"),
            LPAREN,
            RPAREN,
            LBRACKET,
            ID("print"),
            LPAREN,
            NUM(10),
            RPAREN,
            SEMICOLON,
            RBRACKET
        ],
        Program {
            procedures: vec![],
            main: vec![Statement::Expr(PrimitiveCall(Print, vec![Num(10)]))]
        }
    ))
}

#[test]
fn empty_program_gives_error() {
    assert!(parse_error(vec![]))
}
