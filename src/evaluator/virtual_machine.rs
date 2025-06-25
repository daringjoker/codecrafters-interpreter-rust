use std::{cell::RefCell, rc::Rc};

use crate::evaluator::{
    evaluate::runtime_error,
    scope::{Scope, ScopeLevel},
    value::Value,
};

pub struct VirtualMachine {
    pub root_scope: Rc<RefCell<Scope>>,
    pub current_scope: Rc<RefCell<Scope>>,
    pub call_stack: Vec<Rc<RefCell<Scope>>>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let global_scope = Rc::new(RefCell::new(Scope::new(ScopeLevel::Global, "Global scope")));
        let mut machine = VirtualMachine {
            root_scope: global_scope.clone(),
            current_scope: global_scope.clone(),
            call_stack: vec![global_scope.clone()],
        };

        // Register native functions in the global scope
        machine.register_all_native_functions();

        machine
    }
    /// Register all native functions from the compile-time registry
    fn register_all_native_functions(&mut self) {
        for registration in crate::native_functions::get_all_native_functions() {
            let callable = registration.create_callable();
            self.root_scope.borrow_mut().define_variable(
                registration.name.to_string(),
                Some(Value::Function(callable, None)),
            );
        }
    }

    pub fn enter_child_scope(&mut self) -> () {
        let reason = format!("Child scope from {}", self.current_scope.borrow().reason);
        let new_scope = Rc::new(RefCell::new(Scope::new(ScopeLevel::Block, reason.as_str())));
        new_scope.borrow_mut().parent_scope = Some(self.current_scope.clone());
        self.call_stack.push(self.current_scope.clone());
        self.current_scope = new_scope.clone();
    }

    pub fn enter_closured_scope(
        &mut self,
        closure: Option<Rc<RefCell<Scope>>>,
        reason: &str,
    ) -> () {
        let new_scope = Rc::new(RefCell::new(Scope::new(ScopeLevel::Closure, reason)));
        new_scope.borrow_mut().parent_scope =
            Some(closure.clone().unwrap_or(self.root_scope.clone()));
        self.call_stack.push(self.current_scope.clone());
        self.current_scope = new_scope.clone();
    }

    pub fn exit_child_scope(&mut self) -> () {
        let next_scope = self
            .call_stack
            .pop()
            .expect("Call stack is empty, cannot exit child scope");
        let is_killed = !self.current_scope.borrow().is_alive();
        let retval = self
            .current_scope
            .borrow_mut()
            .get_return_value()
            .unwrap_or(Value::Nothing);
        let prev_scope = self.current_scope.clone();
        self.current_scope = next_scope.clone();
        if is_killed && prev_scope.borrow().level != ScopeLevel::Closure {
            self.current_scope.borrow_mut().kill_scope();
            self.current_scope.borrow_mut().set_return_value(retval);
        }
    }

    pub fn start_return_sequence(&mut self, value: Value) -> () {
        self.current_scope
            .borrow_mut()
            .set_return_value(value.clone());
        self.current_scope.borrow_mut().kill_scope();
    }

    pub fn return_from_call(&mut self) -> Value {
        let ret_val = self
            .current_scope
            .borrow_mut()
            .get_return_value()
            .unwrap_or(Value::Nothing);
        self.exit_child_scope();
        return ret_val;
    }
}
