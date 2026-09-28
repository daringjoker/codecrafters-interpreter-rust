use crate::{
    evaluator::{
        evaluate::{comma_to_vec, runtime_error, Eval},
        value::Value,
        virtual_machine::VirtualMachine,
    },
    parser::ast::*,
};

impl Eval for BinaryOperation {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        let retval = match self.operator {
            BinaryOperator::Add => match (
                self.left.evaluate(vm).clone(),
                self.right.evaluate(vm).clone(),
            ) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                // (Value::String(x), Value::Number(y)) => Value::String(x + y.to_string().as_str()),
                // (Value::Number(y), Value::String(x)) => Value::String(x + y.to_string().as_str()),
                (Value::String(x), Value::String(y)) => Value::String(x + &y),
                (_, _) => runtime_error("Operands Must be string or number"),
            },

            BinaryOperator::Subtract => match (
                self.left.evaluate(vm).clone(),
                self.right.evaluate(vm).clone(),
            ) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                (_, _) => runtime_error("Operands Must be string or number"),
            },

            BinaryOperator::Multiply => match (
                self.left.evaluate(vm).clone(),
                self.right.evaluate(vm).clone(),
            ) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::Divide => match (
                self.left.evaluate(vm).clone(),
                self.right.evaluate(vm).clone(),
            ) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x / y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::LessThan => match (
                self.left.evaluate(vm).clone(),
                self.right.evaluate(vm).clone(),
            ) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x < y),
                (Value::Int(x), Value::Int(y)) => Value::Boolean(x < y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::LessThanOrEqual => {
                match (
                    self.left.evaluate(vm).clone(),
                    self.right.evaluate(vm).clone(),
                ) {
                    (Value::Number(x), Value::Number(y)) => Value::Boolean(x <= y),
                    (Value::Int(x), Value::Int(y)) => Value::Boolean(x <= y),
                    (_, _) => runtime_error("Operands Must be number"),
                }
            }

            BinaryOperator::GreaterThan => match (
                self.left.evaluate(vm).clone(),
                self.right.evaluate(vm).clone(),
            ) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x > y),
                (Value::Int(x), Value::Int(y)) => Value::Boolean(x > y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::GreaterThanOrEqual => {
                match (
                    self.left.evaluate(vm).clone(),
                    self.right.evaluate(vm).clone(),
                ) {
                    (Value::Number(x), Value::Number(y)) => Value::Boolean(x >= y),
                    (Value::Int(x), Value::Int(y)) => Value::Boolean(x >= y),
                    (_, _) => runtime_error("Operands Must be number"),
                }
            }

            BinaryOperator::And => {
                let left_bool: bool = self.left.evaluate(vm).clone().into();
                if !left_bool {
                    self.left.evaluate(vm)
                } else {
                    self.right.evaluate(vm)
                }
            }

            BinaryOperator::Or => {
                let left_bool: bool = self.left.evaluate(vm).clone().into();
                if left_bool {
                    self.left.evaluate(vm)
                } else {
                    self.right.evaluate(vm)
                }
            }
            BinaryOperator::Equal => {
                Value::Boolean(self.left.evaluate(vm).clone() == self.right.evaluate(vm).clone())
            }
            BinaryOperator::NotEqual => {
                Value::Boolean(self.left.evaluate(vm).clone() != self.right.evaluate(vm).clone())
            }
            BinaryOperator::Comma => {
                let _ = self.left.evaluate(vm);
                self.right.evaluate(vm)
            }
            BinaryOperator::Call => {
                let callable = self.left.evaluate(vm);
                if let Value::Function(func, closure) = callable {
                    let args = comma_to_vec(*self.right.clone(), vm);
                    if args.len() != func.arity() {
                        runtime_error(&format!(
                            "Function {} expects {} arguments, got {}",
                            func.name(),
                            func.arity(),
                            args.len()
                        ));
                    }
                    vm.enter_closured_scope(closure, func.name().as_str());
                    let retval = func.call(vm, args.clone());
                    let scoped_return_value = vm.return_from_call();
                    if func.callable_type() == super::callable::CallableType::UserDefinedFunction {
                        return scoped_return_value;
                    } else {
                        return retval;
                    }
                } else {
                    runtime_error(&format!("{} is not a function", callable));
                }
            }
            BinaryOperator::Assignment => {
                if let Expression::Identifier(id) = &self.left.as_ref() {
                    let value = self.right.evaluate(vm);
                    vm.current_scope
                        .borrow_mut()
                        .update_variable(id.as_str(), value.clone());
                    return value.clone();
                } else {
                    runtime_error("Left-hand side of assignment must be an identifier");
                }
            }
            _ => unimplemented!("Binary Operator {} is not implemented!", self.operator),
        };

        // eprintln!(
        //     "{} {} {} == {} {} {} = {}",
        //     &self.left, &self.operator, &self.right, &self.left.evaluate(vm), &self.operator, &rhs, &retval
        // );

        retval
    }
}
