use crate::parser::ast::{ASTNode, Expression, Literal};

pub trait Eval {
    fn evaluate(&self, vm: &VirtualMachine) -> ();
}

pub struct VirtualMachine {}

struct Scope {}

impl Eval for ASTNode {
    fn evaluate(&self, vm: &VirtualMachine) {
        match &self {
            ASTNode::Expression(ex) => ex.evaluate(vm),
            ASTNode::Statement(_st) => todo!("implement the statement using vm"),
            _ => unimplemented!(),
        }
    }
}

impl Eval for Expression {
    fn evaluate(&self, vm: &VirtualMachine) {
        match &self {
            Expression::Literal(lit) => lit.evaluate(vm),
            _ => unimplemented!(),
        }
    }
}

impl Eval for Literal {
    fn evaluate(&self, _vm: &VirtualMachine) {
        match &self {
            Literal::NumberFloat(n) => println!("{}", n),
            Literal::NumberInteger(i) => println!("{}", i),
            Literal::String(s) => println!("{}", s),
            Literal::Boolean(b) => println!("{}", b),
            Literal::NIL => println!("nil"),
        }
    }
}
