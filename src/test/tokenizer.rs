use crate::tokenizer::{tokenize, Token};

fn tokenize_success(s: &str, expected: Vec<Token>) -> bool {
    match tokenize(s) {
        Ok(actual) => actual.eq(&expected),
        Err(_) => false,
    }
}

fn tokenize_mismatch(s: &str, expected: Vec<Token>) -> bool {
    match tokenize(s) {
        Ok(actual) => actual.ne(&expected),
        Err(_) => false,
    }
}

fn tokenize_error(s: &str) -> bool {
    match tokenize(s) {
        Ok(_) => false,
        Err(_) => true,
    }
}

#[test]
fn numbers() {
    assert!(tokenize_success("1234", vec![Token::NUM { n: 1234 }]));
    assert!(tokenize_success("-1234", vec![Token::NUM { n: -1234 }]));
}

#[test]
fn ids() {
    assert!(tokenize_success("abc", vec![Token::ID { value: "abc" }]));
    assert!(tokenize_success("AbC", vec![Token::ID { value: "AbC" }]));
    assert!(tokenize_success("aBc_", vec![Token::ID { value: "aBc_" }]));
    assert!(tokenize_success(
        "abc_12",
        vec![Token::ID { value: "abc_12" }]
    ));
    assert!(tokenize_success(
        "abc_12_DE",
        vec![Token::ID { value: "abc_12_DE" }]
    ));

    assert!(tokenize_mismatch(
        "12345",
        vec![Token::ID { value: "12345" }]
    ));

    assert!(tokenize_error("1a"));
    assert!(tokenize_error("_abc"));
}