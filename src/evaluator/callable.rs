use crate::evaluator::{value::Value, virtual_machine::VirtualMachine};

#[derive(Debug, Clone, PartialEq)]
pub enum CallableType {
    NativeFunction,
    UserDefinedFunction,
}

pub trait Callable {
    fn call(&self, vm: &mut VirtualMachine, args: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
    fn name(&self) -> String;
    fn callable_type(&self) -> CallableType;
    fn arguments(&self) -> Vec<String>;
}
