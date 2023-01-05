use std::collections::HashMap;

use crate::{
    error::{
        LingerError::{self, RuntimeError},
        RuntimeError::*,
    },
    interpreter::Value,
};

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    values: HashMap<String, Value<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Result<Value, LingerError> {
        match self.values.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError(UnknownVariable(key))),
        }
    }

    pub fn extend(mut self, key: String, value: Value<'a>) -> Self {
        self.values.insert(key, value);
        return self;
    }

    pub fn update(&mut self, key: String, value: Value<'a>) {
        self.values.insert(key, value);
    }
}
