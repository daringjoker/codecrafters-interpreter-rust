use crate::evaluator::{evaluate::runtime_error, value::Value};
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopeLevel {
    Global,
    Module,
    Function,
    Block,
    Closure,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    pub reason: String,
    pub level: ScopeLevel,
    alive: bool,
    return_value: Option<Value>,
    pub parent_scope: Option<Rc<RefCell<Scope>>>,
    pub variables: HashMap<String, Value>,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Scope(level: {:?}, reason: '{}' variables: {:?})",
            self.level,
            self.reason,
            self.variables.keys().collect::<Vec<&String>>(),
        )
    }
}
impl Scope {
    pub fn new(level: ScopeLevel, reason: &str) -> Scope {
        Scope {
            reason: reason.to_string(),
            level,
            parent_scope: None,
            variables: HashMap::new(),
            return_value: None,
            alive: true,
        }
    }

    pub fn get_return_value(&self) -> Option<Value> {
        self.return_value.clone()
    }
    pub fn set_return_value(&mut self, value: Value) {
        self.return_value = Some(value);
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn kill_scope(&mut self) {
        self.alive = false;
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
