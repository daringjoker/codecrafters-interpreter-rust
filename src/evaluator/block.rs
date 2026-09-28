use crate::{
    evaluator::{
        evaluate::{comma_to_vec, runtime_error, Eval},
        value::Value,
        virtual_machine::VirtualMachine,
    },
    parser::ast::*,
};

impl Eval for Block {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        vm.enter_child_scope();
        for stmt in &self.statements {
            if !vm.current_scope.borrow_mut().is_alive() {
                break;
            }
            stmt.evaluate(vm);
        }
        vm.exit_child_scope();
        Value::Nothing
    }
}
