use crate::{
    desugar::Expr,
    error::RuntimeError::{self, *},
};

use super::Value;

pub fn ensure_single_arg(args: Vec<Expr>) -> Result<Expr, RuntimeError> {
    if args.len() > 1 {
        return Err(ArgMismatch("is_empty".to_string(), args.len(), 1));
    }

    match args.first() {
        Some(arg) => Ok(arg.clone()),
        None => return Err(ArgMismatch("is_empty".to_string(), 0, 1)),
    }
}

pub fn ensure_list(value: Value) -> Result<Vec<Value>, RuntimeError> {
    match value {
        Value::List(list) => Ok(list),
        bad_value => Err(ExpectedList(bad_value.to_string())),
    }
}
