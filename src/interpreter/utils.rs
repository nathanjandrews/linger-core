use crate::{
    desugar::Expr,
    error::RuntimeError::{self, *},
};

pub fn ensure_single_arg(args: Vec<Expr>) -> Result<Expr, RuntimeError> {
    if args.len() > 1 {
        return Err(ArgMismatch("is_empty".to_string(), args.len(), 1));
    }

    match args.first() {
        Some(arg) => Ok(arg.clone()),
        None => return Err(ArgMismatch("is_empty".to_string(), 0, 1)),
    }
}
