use crate::parser::ast::*;
use std::fmt::Display;

pub trait Eval {
    fn evaluate(&self, vm: &VirtualMachine) -> Value;
}

pub struct VirtualMachine {}

struct Scope {}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Int(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Boolean(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Eval for ASTNode {
    fn evaluate(&self, vm: &VirtualMachine) -> Value {
        match &self {
            ASTNode::Expression(ex) => ex.evaluate(vm),
            ASTNode::Statement(_st) => todo!("implement the statement using vm"),
            _ => unimplemented!(),
        }
    }
}

impl Eval for Expression {
    fn evaluate(&self, vm: &VirtualMachine) -> Value {
        match &self {
            Expression::Literal(lit) => lit.evaluate(vm),
            Expression::BinaryOperation(op) => op.evaluate(vm),
            Expression::UnaryOperation(op) => op.evaluate(vm),
            _ => unimplemented!(),
        }
    }
}

impl Eval for Literal {
    fn evaluate(&self, _vm: &VirtualMachine) -> Value {
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
    fn evaluate(&self, vm: &VirtualMachine) -> Value {
        let lhs = self.left.evaluate(vm);
        let rhs = self.right.evaluate(vm);
        match self.operator {
            BinaryOperator::Add => match (lhs, rhs) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                (Value::String(x), Value::Number(y)) => Value::String(x + y.to_string().as_str()),
                (Value::Number(y), Value::String(x)) => Value::String(x + y.to_string().as_str()),
                (Value::String(x), Value::String(y)) => Value::String(x + &y),
                (_, _) => unimplemented!("No valid symantic meaning for this"),
            },
            BinaryOperator::Subtract => match (lhs, rhs) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                (_, _) => unimplemented!("No valid symantic meaning for this"),
            },
            BinaryOperator::Multiply => match (lhs, rhs) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                (_, _) => unimplemented!("No valid symantic meaning for this"),
            },
            BinaryOperator::Divide => match (lhs, rhs) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x / y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                (_, _) => unimplemented!("No valid symantic meaning for this"),
            },
            BinaryOperator::Equal => Value::Boolean(lhs == rhs),
            BinaryOperator::NotEqual => Value::Boolean(lhs != rhs),
            _ => unimplemented!("Binary Operator {} is not implemented!", self.operator),
        }
    }
}
impl Eval for UnaryOperation {
    fn evaluate(&self, vm: &VirtualMachine) -> Value {
        let lhs = self.operand.evaluate(vm);
        let retval = match (self.operator.clone(), lhs.clone()) {
            (UnaryOperator::Negate, Value::Number(n)) => Value::Number(-n),
            (UnaryOperator::Plus, Value::Number(n)) => Value::Number(n),
            (UnaryOperator::Not, Value::Boolean(n)) => Value::Boolean(!n),
            (UnaryOperator::Not, Value::Number(n)) => Value::Boolean(n == 0.0),
            (UnaryOperator::Not, Value::Int(n)) => Value::Boolean(n == 0),
            (UnaryOperator::Not, Value::String(n)) => Value::Boolean(n == ""),
            (UnaryOperator::Not, Value::Nil) => Value::Boolean(true),
            (UnaryOperator::Group, l) => l,
            _ => unimplemented!(
                "Unary Operator {} is not Valid for {}!",
                self.operator,
                &lhs
            ),
        };
        eprintln!(
            "{} {} == {} {} = {}",
            &self.operator, &self.operand, &self.operator, lhs, retval
        );
        return retval;
    }
}
