use crate::parser::ast::*;
use std::fmt::Display;
use std::process::exit;

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
        let retval = match self.operator {
            BinaryOperator::Add => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                (Value::String(x), Value::Number(y)) => Value::String(x + y.to_string().as_str()),
                (Value::Number(y), Value::String(x)) => Value::String(x + y.to_string().as_str()),
                (Value::String(x), Value::String(y)) => Value::String(x + &y),
                (_, _) => runtime_error("Operands Must be string or number"),
            },

            BinaryOperator::Subtract => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                (_, _) => runtime_error("Operands Must be string or number"),
            },

            BinaryOperator::Multiply => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::Divide => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x / y),
                (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::LessThan => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x < y),
                (Value::Int(x), Value::Int(y)) => Value::Boolean(x < y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::LessThanOrEqual => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x <= y),
                (Value::Int(x), Value::Int(y)) => Value::Boolean(x <= y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::GreaterThan => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x > y),
                (Value::Int(x), Value::Int(y)) => Value::Boolean(x > y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::GreaterThanOrEqual => match (lhs.clone(), rhs.clone()) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x >= y),
                (Value::Int(x), Value::Int(y)) => Value::Boolean(x >= y),
                (_, _) => runtime_error("Operands Must be number"),
            },

            BinaryOperator::Equal => Value::Boolean(lhs.clone() == rhs.clone()),
            BinaryOperator::NotEqual => Value::Boolean(lhs.clone() != rhs.clone()),
            _ => unimplemented!("Binary Operator {} is not implemented!", self.operator),
        };

        eprintln!(
            "{} {} {} == {} {} {} = {}",
            &self.left, &self.operator, &self.right, &lhs, &self.operator, &rhs, &retval
        );

        retval
    }
}
impl Eval for UnaryOperation {
    fn evaluate(&self, vm: &VirtualMachine) -> Value {
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
        };
        eprintln!(
            "{} {} == {} {} = {}",
            &self.operator, &self.operand, &self.operator, lhs, retval
        );
        return retval;
    }
}

pub fn runtime_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    exit(70);
}
