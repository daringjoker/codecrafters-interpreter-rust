use crate::evaluator::callable::Callable;
use std::rc::Rc;

pub mod examples;

// Re-export the macro
pub use native_function_macro::native_function;

/// Structure used for compile-time registration of native functions
pub struct NativeFunctionRegistration {
    pub name: &'static str,
    pub factory: fn() -> Rc<dyn Callable>,
}

impl NativeFunctionRegistration {
    pub fn create_callable(&self) -> Rc<dyn Callable> {
        (self.factory)()
    }
}

// Collect all registered native functions at runtime
inventory::collect!(NativeFunctionRegistration);

/// Get all registered native functions
pub fn get_all_native_functions() -> Vec<&'static NativeFunctionRegistration> {
    inventory::iter::<NativeFunctionRegistration>().collect()
}

/// Find a native function by name
pub fn find_native_function(name: &str) -> Option<&'static NativeFunctionRegistration> {
    inventory::iter::<NativeFunctionRegistration>()
        .find(|reg| reg.name == name)
}