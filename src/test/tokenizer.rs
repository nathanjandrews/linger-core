use crate::tokenizer::{tokenize, Token};

fn tokenize_success(s: &str, expected: Vec<Token>) -> bool {
    match tokenize(s) {
        Ok(actual) => actual.eq(&expected),
        Err(_) => false,
    }
}

#[test]
fn single_number() {
    assert!(tokenize_success("1234", vec![Token::NUM { n: 1234 }]))
}
