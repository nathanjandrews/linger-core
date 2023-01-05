use std::collections::HashMap;

use crate::{
    error::{
        LingerError::{self, RuntimeError},
        RuntimeError::*,
    },
    interpreter::Value,
};

type Binding = (String, Value);

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
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

    pub fn extend(mut self, bindings: Vec<Binding>) -> Self {
        for (var, value) in bindings {
            self.values.insert(var, value);
        }
        return self;
    }

    pub fn update(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn bindings(&self) -> Vec<Binding> {
        return self.values.clone().into_iter().collect();
    }
}
