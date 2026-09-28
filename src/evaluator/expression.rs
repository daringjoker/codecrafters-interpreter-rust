use crate::{
    evaluator::{
        evaluate::{comma_to_vec, runtime_error, Eval},
        value::Value,
        virtual_machine::VirtualMachine,
    },
    parser::ast::*,
};
impl Eval for Expression {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        match &self {
            Expression::Literal(lit) => lit.evaluate(vm),
            Expression::BinaryOperation(op) => op.evaluate(vm),
            Expression::UnaryOperation(op) => op.evaluate(vm),
            Expression::Identifier(id) => vm
                .current_scope
                .as_ref()
                .borrow()
                .get_variable_value(id)
                .unwrap_or_else(|| {
                    dbg!(vm.current_scope.borrow().variables.keys());
                    runtime_error(&format!("Variable {} not found", id))
                })
                .clone(),
            Expression::NOP => Value::Nothing,
        }
    }
}
