use crate::tokenizer::{
    tokenize,
    TokenValue::{self, *},
};

fn tokenize_success(s: &str, expected: Vec<TokenValue>) -> bool {
    match tokenize(s) {
        Ok(actual) => {
            let actual: Vec<TokenValue> = actual.into_iter().map(|t| t.0).collect();
            actual.eq(&expected)
        }
        Err(_) => false,
    }
}

fn tokenize_mismatch(s: &str, expected: Vec<TokenValue>) -> bool {
    match tokenize(s) {
        Ok(actual) => {
            let actual: Vec<TokenValue> = actual.into_iter().map(|t| t.0).collect();
            actual.ne(&expected)
        }
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
    assert!(tokenize_success("1234", vec![NUM(1234)]));
    assert!(tokenize_success("-1234", vec![NUM(-1234)]));
}

#[test]
fn ids() {
    assert!(tokenize_success("abc", vec![ID("abc")]));
    assert!(tokenize_success("AbC", vec![ID("AbC")]));
    assert!(tokenize_success("aBc_", vec![ID("aBc_")]));
    assert!(tokenize_success("abc_12", vec![ID("abc_12")]));
    assert!(tokenize_success("abc_12_DE", vec![ID("abc_12_DE")]));

    assert!(tokenize_mismatch("12345", vec![ID("12345")]));

    assert!(tokenize_error("1a"));
    assert!(tokenize_error("_abc"));
}
