use crate::{
    evaluator::{
        evaluate::{runtime_error, Eval},
        value::Value,
        virtual_machine::VirtualMachine,
    },
    parser::ast::*,
};

impl Eval for UnaryOperation {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        let lhs = self.operand.evaluate(vm);
        let retval = match (self.operator.clone(), lhs.clone()) {
            (UnaryOperator::Negate, Value::Number(n)) => Value::Number(-n),
            (UnaryOperator::Negate, _) => runtime_error("Operand Must be a number"),
            (UnaryOperator::Plus, Value::Number(n)) => Value::Number(n),
            (UnaryOperator::Plus, _) => runtime_error("Operand Must be a number"),
            (UnaryOperator::Not, Value::Boolean(n)) => Value::Boolean(!n),
            (UnaryOperator::Not, Value::Number(n)) => Value::Boolean(n == 0.0),
            (UnaryOperator::Not, Value::Int(n)) => Value::Boolean(n == 0),
            (UnaryOperator::Not, Value::String(n)) => Value::Boolean(n == ""),
            (UnaryOperator::Not, Value::Nil) => Value::Boolean(true),
            (UnaryOperator::Group, l) => l,
            (_, _) => runtime_error("undefined behaviour"),
        };
        // eprintln!(
        //     "{} {} == {} {} = {}",
        //     &self.operator, &self.operand, &self.operator, lhs, retval
        // );
        return retval;
    }
}
