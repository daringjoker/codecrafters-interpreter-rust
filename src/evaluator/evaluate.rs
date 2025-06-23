use super::scope::Scope;
use crate::parser::ast::*;
use std::cell::RefCell;
use std::fmt::Display;
use std::process::exit;
use std::rc::Rc;

pub trait Eval {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value;
}

pub struct VirtualMachine {
    root_scope: Rc<RefCell<Scope>>,
    current_scope: Rc<RefCell<Scope>>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let global_scope = Rc::new(RefCell::new(Scope::new()));
        VirtualMachine {
            root_scope: global_scope.clone(),
            current_scope: global_scope.clone(),
        }
    }

    pub fn enter_child_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope::new()));
        new_scope.borrow_mut().parent_scope = Some(self.current_scope.clone());
        self.current_scope = new_scope.clone();
    }
    pub fn exit_child_scope(&mut self) {
        let cur_scope = self.current_scope.clone();
        self.current_scope = cur_scope
            .as_ref()
            .borrow()
            .parent_scope
            .clone()
            .unwrap_or(self.root_scope.clone());
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    Nothing,
    Nil,
}

impl Value {
    pub fn is_nothing(&self) -> bool {
        *self == Value::Nothing
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Number(n) => n != 0.0,
            Value::Int(n) => n != 0,
            Value::String(_) => true,
            Value::Boolean(b) => b,
            Value::Nil => false,
            Value::Nothing => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Int(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Boolean(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
            Value::Nothing => write!(f, "Nothing"),
        }
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
                    while_loop.body.evaluate(vm);
                }
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
                    for_loop.body.evaluate(vm);
                    if let Some(increment) = &for_loop.increment.as_ref() {
                        increment.evaluate(vm);
                    }
                }
                return Value::Nothing;
            }
            _ => unimplemented!(),
        }
    }
}

impl Eval for Block {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        vm.enter_child_scope();
        let mut retval = Value::Nothing;
        for stmt in &self.statements {
            retval = stmt.evaluate(vm);
        }
        vm.exit_child_scope();
        return retval;
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
            _ => unimplemented!(),
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

pub fn runtime_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    exit(70);
}
