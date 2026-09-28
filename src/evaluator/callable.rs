use crate::{
    evaluator::{
        evaluate::{runtime_error, Eval},
        value::Value,
        virtual_machine::VirtualMachine,
    },
    parser::ast::FunctionDeclaration,
};

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

impl Callable for FunctionDeclaration {
    fn callable_type(&self) -> super::callable::CallableType {
        super::callable::CallableType::UserDefinedFunction
    }
    fn call(&self, vm: &mut VirtualMachine, args: Vec<Value>) -> Value {
        if args.len() != self.parameters.len() {
            runtime_error(&format!(
                "Function {} expects {} arguments, got {}",
                self.name,
                self.parameters.len(),
                args.len()
            ));
        }
        for (param, arg) in self.parameters.iter().zip(args) {
            vm.current_scope
                .borrow_mut()
                .define_variable(param.clone(), Some(arg));
        }
        for stmt in &self.body.statements {
            if !vm.current_scope.borrow_mut().is_alive() {
                break;
            }
            stmt.evaluate(vm);
        }
        Value::Nothing
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn arity(&self) -> usize {
        self.parameters.len()
    }
    fn arguments(&self) -> Vec<String> {
        self.parameters.clone()
    }
}
