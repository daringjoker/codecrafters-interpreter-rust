use crate::{
    evaluator::{value::Value, virtual_machine::VirtualMachine},
    parser::ast::*,
};
use std::process::exit;

pub trait Eval {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value;
}

pub fn comma_to_vec(expr: Expression, vm: &mut VirtualMachine) -> Vec<Value> {
    // dbg!(&expr);
    match expr {
        Expression::NOP => vec![], // Empty argument list
        Expression::BinaryOperation(comma_operation) => {
            if matches!(comma_operation.operator, BinaryOperator::Comma) {
                let mut left_values = comma_to_vec(*comma_operation.left, vm);
                let mut right_values = comma_to_vec(*comma_operation.right, vm);
                left_values.append(&mut right_values);
                left_values
            } else {
                vec![Expression::BinaryOperation(comma_operation).evaluate(vm)]
            }
        }
        _ => vec![expr.evaluate(vm)],
    }
}

pub fn runtime_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    exit(70);
}
