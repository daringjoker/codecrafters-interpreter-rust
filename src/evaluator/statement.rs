use std::rc::Rc;

use crate::{
    evaluator::{evaluate::Eval, value::Value, virtual_machine::VirtualMachine},
    parser::ast::*,
};

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
            Statement::Block(block) => block.evaluate(vm),
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
                        while_loop.body.evaluate(vm);
                    } else {
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
                vm.start_return_sequence(value.clone());
                return value;
            }
        }
    }
}
