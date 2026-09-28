use crate::{
    evaluator::{
        evaluate::{comma_to_vec, runtime_error, Eval},
        value::Value,
        virtual_machine::VirtualMachine,
    },
    parser::ast::*,
};

impl Eval for Literal {
    fn evaluate(&self, _vm: &mut VirtualMachine) -> Value {
        match &self {
            Literal::NumberFloat(n) => Value::Number(*n),
            Literal::NumberInteger(i) => Value::Int(*i),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::NIL => Value::Nil,
        }
    }
}
