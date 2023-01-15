use crate::{
    error::ParseError::{self, *},
    tokenizer::{Keyword::*, Token as T, TokenValue::*},
};

use super::utils::{ensure_block, unexpected_token};
use super::statements::parse_statement;
use super::SugaredProcedure;

pub fn parse_procs(tokens: &[T]) -> Result<(Vec<SugaredProcedure>, &[T]), ParseError> {
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
                return Err(MultipleSameNamedProcs(proc.name.to_string()));
            }

            let mut vec = vec![proc];
            vec.append(&mut rest_procs);
            return Ok((vec, tokens));
        }
        None => Ok((vec![], tokens)),
    }
}

pub fn parse_proc(tokens: &[T]) -> Result<(Option<SugaredProcedure>, &[T]), ParseError> {
    match tokens {
        [T(KW(Proc), ..), T(KW(kw), ..), T(LPAREN, ..), ..] => Err(KeywordAsProc(kw.to_string())),
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

pub fn parse_params(tokens: &[T]) -> Result<(Vec<String>, &[T]), ParseError> {
    match tokens {
        [T(RPAREN, ..), rest @ ..] => Ok((vec![], rest)),
        [T(KW(kw), ..), ..] => Err(KeywordAsParam(kw.to_string())),
        [T(ID(param_name), ..), rest_toks @ ..] => {
            let (mut rest_params, rest_toks) = parse_rest_params(rest_toks)?;
            let mut params = vec![param_name.to_string()];
            params.append(&mut rest_params);
            Ok((params, rest_toks))
        }
        tokens => Err(unexpected_token(tokens)),
    }
}

pub fn parse_rest_params(tokens: &[T]) -> Result<(Vec<String>, &[T]), ParseError> {
    match tokens {
        [T(RPAREN, ..), tokens @ ..] => Ok((vec![], tokens)),
        [T(COMMA, ..), T(RPAREN, ..), ..] => Err(unexpected_token(tokens)),
        [T(COMMA, ..), tokens @ ..] => parse_params(tokens),
        tokens => Err(unexpected_token(tokens)),
    }
}
