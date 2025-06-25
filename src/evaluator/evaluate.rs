use crate::{
    evaluator::{callable::Callable, value::Value, virtual_machine::VirtualMachine},
    parser::ast::*,
};
use std::{process::exit, rc::Rc};

pub trait Eval {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value;
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
        self.body.evaluate(vm);
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

impl Eval for ASTNode {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        match &self {
            ASTNode::Expression(ex) => ex.evaluate(vm),
            ASTNode::Statement(st) => st.evaluate(vm),
            _ => unimplemented!(),
        }
    }
}
impl Eval for Statement {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        match &self {
            Statement::Expression(exp) => {
                let _expr = exp.evaluate(vm);
                return Value::Nothing;
            }
            Statement::Print(pr) => {
                let expr = pr.value.evaluate(vm);
                // eprintln!("{} =>{}", pr, expr);
                println!("{}", expr);
                return Value::Nothing;
            }
            Statement::Declaration(Declaration::Variable(var)) => {
                let value = if let Some(expr) = &var.value {
                    Some(expr.evaluate(vm))
                } else {
                    None
                };
                vm.current_scope
                    .borrow_mut()
                    .define_variable(var.name.clone(), value);
                return Value::Nothing;
            }
            Statement::Block(block) => {
                let prev = vm.enter_child_scope();
                block.evaluate(vm);
                vm.exit_child_scope(prev);
                return Value::Nothing;
            }
            Statement::If(if_stmt) => {
                let condition = if_stmt.condition.evaluate(vm);
                if condition.into() {
                    if_stmt.then_branch.evaluate(vm);
                } else if let Some(else_branch) = &*if_stmt.else_branch {
                    else_branch.evaluate(vm);
                }
                return Value::Nothing;
            }

            Statement::While(while_loop) => {
                while while_loop.condition.evaluate(vm).into() {
                    if vm.current_scope.borrow_mut().is_alive() {
                        // If the scope is not killed, continue evaluating the body
                        while_loop.body.evaluate(vm);
                    } else {
                        // If the scope is killed, break out of the loop
                        break;
                    }
                }
                return Value::Nothing;
            }

            Statement::Declaration(Declaration::Function(func)) => {
                vm.current_scope.as_ref().borrow_mut().define_variable(
                    func.name.clone(),
                    Some(Value::Function(
                        Rc::new(func.clone()),
                        Some(vm.current_scope.clone()),
                    )),
                );
                return Value::Nothing;
            }

            Statement::For(for_loop) => {
                if let Some(decl) = for_loop.initializer.as_ref() {
                    decl.evaluate(vm);
                }
                while for_loop
                    .condition
                    .as_ref()
                    .unwrap_or(&Expression::NOP)
                    .evaluate(vm)
                    .into()
                {
                    if !vm.current_scope.borrow_mut().is_alive() {
                        break; // Exit the loop if the scope is killed
                    }
                    for_loop.body.evaluate(vm);
                    if let Some(increment) = &for_loop.increment.as_ref() {
                        increment.evaluate(vm);
                    }
                }
                return Value::Nothing;
            }
            Statement::Return(ret) => {
                let value = ret.value.as_ref().map_or(Value::Nil, |v| v.evaluate(vm));
                vm.current_scope
                    .borrow_mut()
                    .set_return_value(value.clone());
                vm.current_scope.borrow_mut().kill_scope();
                return Value::Nothing;
            }
            _ => unimplemented!(),
        }
    }
}

impl Eval for Block {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        for stmt in &self.statements {
            stmt.evaluate(vm);
            if !vm.current_scope.borrow_mut().is_alive() {
                break;
            }
        }
        Value::Nothing
    }
}

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
                .unwrap_or_else(|| runtime_error(&format!("Variable {} not found", id)))
                .clone(),
            Expression::NOP => Value::Nothing,
        }
    }
}

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
                    // Enter a new scope for the function call
                    // eprintln!("Calling function {} with args: {:?}", id, args);
                    if args.len() != func.arity() {
                        runtime_error(&format!(
                            "Function {} expects {} arguments, got {}",
                            func.name(),
                            func.arity(),
                            args.len()
                        ));
                    }
                    let curr_scope = vm.enter_closured_scope(closure);
                    let retval = func.call(vm, args.clone());
                    let scoped_return_value = vm.exit_child_scope(curr_scope);
                    if func.callable_type() == super::callable::CallableType::UserDefinedFunction {
                        // eprintln!(
                        //     " {}({:?}) returned: {:?}",
                        //     id, &args, scoped_return_value
                        // );
                        return scoped_return_value;
                    } else {
                        // eprintln!(" {}({:?}) returned: {:?}", id, &args, retval);
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

fn comma_to_vec(expr: Expression, vm: &mut VirtualMachine) -> Vec<Value> {
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
