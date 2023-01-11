use std::collections::HashMap;

use crate::{
    desugar::{Procedure, Statement},
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

#[derive(Debug, Clone)]
struct TopLevelProcedure {
    params: Vec<String>,
    body: Statement,
}

pub type Entry = (Value, AssignmentType, Mutability);
pub type Binding = (String, Entry);

#[derive(Debug, Clone)]
pub struct Environment {
    top_level_procedures: HashMap<String, TopLevelProcedure>,
    values: HashMap<String, Entry>,
}

impl Environment {
    pub fn new(procedures: Vec<Procedure>) -> Self {
        let mut top_level_procedures = HashMap::new();
        for Procedure { name, params, body } in procedures {
            top_level_procedures.insert(name, TopLevelProcedure { params, body });
        }
        Self {
            values: HashMap::new(),
            top_level_procedures,
        }
    }

    pub fn get(&self, key: String) -> Result<Value, RuntimeError> {
        match self.values.get(&key) {
            Some((value, ..)) => Ok(value.clone()),
            None => match self.top_level_procedures.get(&key) {
                Some(proc) => Ok(Value::Proc(
                    proc.params.clone(),
                    proc.body.clone(),
                    self.clone(),
                )),
                None => Err(UnknownVariable(key)),
            },
        }
    }

    pub fn extend(mut self, bindings: Vec<Binding>) -> Self {
        for (var, value) in bindings {
            self.values.insert(var, value);
        }
        return self;
    }

    pub fn insert_new_mutable_value(&mut self, key: String, value: Value) {
        self.values.insert(
            key,
            (value, AssignmentType::Initialized, Mutability::Mutable),
        );
    }

    pub fn insert_new_constant_value(&mut self, key: String, value: Value) {
        self.values.insert(
            key,
            (value, AssignmentType::Initialized, Mutability::Constant),
        );
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
            None => match self.top_level_procedures.get(&key) {
                Some(_) => return Err(ReassignTopLevelProc(key)),
                None => return Err(UnknownVariable(key)),
            },
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
