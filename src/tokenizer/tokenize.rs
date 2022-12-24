use regex::Regex;

use super::{
    error::TokenizerError,
    token::{
        Token, ID_REGEX, LBRACKET_REGEX, LPAREN_REGEX, NUM_REGEX, RBRACKET_REGEX, RPAREN_REGEX,
        SEMICOLON_REGEX, WHITESPACE_REGEX,
    },
};

pub fn tokenize(s: String) -> Result<Vec<Token>, TokenizerError> {
    tokenize_helper(s, 0)
}

pub fn tokenize_helper(s: String, index: usize) -> Result<Vec<Token>, TokenizerError> {
    if index >= s.len() {
        Ok(vec![])
    } else {
        match get_token(s.as_str(), index) {
            Ok((token_option, new_index)) => match token_option {
                Some(token) => match tokenize_helper(s, new_index) {
                    Ok(mut vec) => {
                        let mut v = vec![token];
                        v.append(&mut vec);
                        Ok(v)
                    }
                    Err(e) => Err(e),
                },
                None => tokenize_helper(s, new_index),
            },
            Err(e) => Err(e),
        }
    }
}

fn get_token(s: &str, index: usize) -> Result<(Option<Token>, usize), TokenizerError> {
    if let Some(mat) = regex(WHITESPACE_REGEX).find_at(s, index) {
        Ok((None, mat.end()))
    } else if let Some(mat) = regex(ID_REGEX).find_at(s, index) {
        Ok((
            Some(Token::ID {
                value: mat.as_str().to_string(),
            }),
            mat.end(),
        ))
    } else if let Some(mat) = regex(NUM_REGEX).find_at(s, index) {
        Ok((
            Some(Token::NUM {
                n: mat.as_str().parse::<i64>().unwrap(),
            }),
            mat.end(),
        ))
    } else if let Some(mat) = regex(LPAREN_REGEX).find_at(s, index) {
        Ok((Some(Token::LPAREN), mat.end()))
    } else if let Some(mat) = regex(RPAREN_REGEX).find_at(s, index) {
        Ok((Some(Token::RPAREN), mat.end()))
    } else if let Some(mat) = regex(LBRACKET_REGEX).find_at(s, index) {
        Ok((Some(Token::LBRACKET), mat.end()))
    } else if let Some(mat) = regex(RBRACKET_REGEX).find_at(s, index) {
        Ok((Some(Token::RBRACKET), mat.end()))
    } else if let Some(mat) = regex(SEMICOLON_REGEX).find_at(s, index) {
        Ok((Some(Token::SEMICOLON), mat.end()))
    } else {
        Err(TokenizerError {})
    }
}

fn regex(re: &str) -> Regex {
    return Regex::new(re).unwrap();
}
