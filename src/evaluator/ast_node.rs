use crate::{
    evaluator::{evaluate::Eval, value::Value, virtual_machine::VirtualMachine},
    parser::ast::*,
};
impl Eval for ASTNode {
    fn evaluate(&self, vm: &mut VirtualMachine) -> Value {
        match &self {
            ASTNode::Expression(ex) => ex.evaluate(vm),
            ASTNode::Statement(st) => st.evaluate(vm),
            _ => unimplemented!(),
        }
    }
}
