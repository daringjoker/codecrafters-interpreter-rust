use std::{cell::RefCell, rc::Rc};

use crate::evaluator::{scope::Scope, value::Value};

pub struct VirtualMachine {
    root_scope: Rc<RefCell<Scope>>,
    pub current_scope: Rc<RefCell<Scope>>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let global_scope = Rc::new(RefCell::new(Scope::new()));
        let mut machine = VirtualMachine {
            root_scope: global_scope.clone(),
            current_scope: global_scope.clone(),
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
