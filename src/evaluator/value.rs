use crate::evaluator::scope::Scope;
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    Nothing,
    Nil,
    Function(
        Rc<dyn crate::evaluator::callable::Callable>,
        Option<Rc<RefCell<Scope>>>,
    ),
}

impl Value {
    pub fn is_nothing(&self) -> bool {
        matches!(self, Value::Nothing)
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Number(n) => n != 0.0,
            Value::Int(n) => n != 0,
            Value::String(_) => true,
            Value::Boolean(b) => b,
            Value::Nil => false,
            Value::Nothing => false,
            Value::Function(_, _) => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Int(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Boolean(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
            Value::Nothing => write!(f, "Nothing"),
            Value::Function(func, _) => {
                write!(f, "<fn {}>", func.name(),)
            }
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Int(n) => write!(f, "Int({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Nothing => write!(f, "Nothing"),
            Value::Nil => write!(f, "Nil"),
            Value::Function(func, _) => {
                write!(
                    f,
                    "<{:?} {}({})>",
                    func.callable_type(),
                    func.name(),
                    func.arguments().join(",")
                )
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nothing, Value::Nothing) => true,
            (Value::Nil, Value::Nil) => true,
            (Value::Function(a, _), Value::Function(b, _)) => a.name() == b.name(),
            _ => false,
        }
    }
}
