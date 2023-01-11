use std::collections::HashMap;

use crate::{
    error::RuntimeError::{self, *},
    interpreter::Value,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssignmentType {
    Initialized,
    Reassigned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mutability {
    Constant,
    Mutable,
}

pub type Entry = (Value, AssignmentType, Mutability);
pub type Binding = (String, Entry);

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Entry>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Result<Entry, RuntimeError> {
        match self.values.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(UnknownVariable(key)),
        }
    }

    pub fn extend(mut self, bindings: Vec<Binding>) -> Self {
        for (var, value) in bindings {
            self.values.insert(var, value);
        }
        return self;
    }

    pub fn extend_new_bindings(mut self, bindings: Vec<Binding>) -> Self {
        for (var, value) in bindings {
            if !self.contains_key(&var) {
                self.values.insert(var, value);
            }
        }
        return self;
    }

    pub fn insert_new(&mut self, key: String, value: Value, mutability: Mutability) {
        self.values
            .insert(key, (value, AssignmentType::Initialized, mutability));
    }

    pub fn reassign(&mut self, key: String, value: Value) -> Result<(), RuntimeError> {
        match self.values.get(&key) {
            Some((_, _, Mutability::Mutable)) => {
                self.values.insert(
                    key,
                    (value, AssignmentType::Reassigned, Mutability::Mutable),
                );
                return Ok(());
            }
            Some((_, _, Mutability::Constant)) => return Err(ReassignConstant(key)),
            None => return Err(UnknownVariable(key)),
        }
    }

    pub fn bindings(&self) -> Vec<Binding> {
        return self.values.clone().into_iter().collect();
    }

    pub fn contains_key(&self, key: &String) -> bool {
        return self.values.contains_key(key);
    }

    pub fn update_reassigned_entries(&mut self, other_env: &Self) -> Result<(), RuntimeError> {
        for (id, (value, assignment_type, _)) in other_env.bindings() {
            if self.contains_key(&id) && assignment_type == AssignmentType::Reassigned {
                self.reassign(id, value)?;
            }
        }
        Ok(())
    }
}
