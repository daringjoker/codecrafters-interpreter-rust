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
    parent_scope: Option<Rc<RefCell<Scope>>>,
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
        self.variables.insert(name, value.unwrap_or(Value::Nothing));
    }

    pub fn get_variable_value(&self, name: &str) -> Option<Value> {
        let value = self.variables.get(name).cloned().or_else(|| {
            if let Some(parent) = &self.parent_scope {
                parent.borrow().get_variable_value(name)
            } else {
                None
            }
        });
        if let Some(Value::Nothing) = value {
            runtime_error(&format!("Variable {} used before initialization", name));
        }
        value
    }
}
