use std::fmt;

use crate::{desugar::Statement, environment::Environment, error::RuntimeError, parser::Program};

use self::statements::interp_statement;

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Bool(bool),
    Str(String),
    Proc(Vec<String>, Statement, Environment),
    List(Vec<Value>),
    // ! consider if Nil should be an explicit value or just return an Option<Value> instead where None represents Nil
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Str(s) => write!(f, "{}", s),
            Value::Proc(..) => write!(f, "<lambda>"),
            Value::List(list) => {
                let values_as_strings: Vec<String> = list.iter().map(|v| v.to_string()).collect();
                let list_string = values_as_strings.join(", ");
                write!(f, "[{list_string}]")
            }
        }
    }
}

mod expressions;
mod statements;
mod utils;

pub fn interp_program<'a>(p: Program) -> Result<Value, RuntimeError> {
    return match interp_statement(&mut Environment::new(p.procedures), p.main, false)? {
        (value, _) => Ok(value),
    };
}
