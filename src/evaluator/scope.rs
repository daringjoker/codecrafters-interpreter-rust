use crate::evaluator::evaluate::{runtime_error, Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub enum ScopeLevel {
    Global,
    Module,
    Function,
    Block,
    Closure,
}

#[derive(Clone, Debug)]
pub struct Scope {
    level: ScopeLevel,
    pub parent_scope: Option<Rc<RefCell<Scope>>>,
    variables: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            level: ScopeLevel::Global,
            parent_scope: None,
            variables: HashMap::new(),
        }
    }

    pub fn define_variable(&mut self, name: String, value: Option<Value>) {
        self.variables.insert(name, value.unwrap_or(Value::Nil));
    }

    pub fn update_variable(&mut self, name: &str, value: Value) {
        if let Some(var) = self.variables.get_mut(name) {
            *var = value;
        } else if let Some(parent) = &self.parent_scope {
            parent.borrow_mut().update_variable(name, value);
        } else {
            runtime_error(&format!("Variable '{}' not defined", name));
        }
    }

    pub fn get_variable_value(&self, name: &str) -> Option<Value> {
        let value = self.variables.get(name).cloned().or_else(|| {
            if let Some(parent) = &self.parent_scope {
                parent.borrow().get_variable_value(name)
            } else {
                None
            }
        });
        value
    }
}
