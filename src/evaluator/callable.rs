use crate::evaluator::evaluate::{Value, VirtualMachine};

#[derive(Debug, Clone)]
pub enum CallableType {
    NativeFunction,
    UserDefinedFunction,
}

pub trait Callable {
    fn call(&self, vm: &mut VirtualMachine, args: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
    fn name(&self) -> String;
    fn callable_type(&self) -> CallableType;
}
