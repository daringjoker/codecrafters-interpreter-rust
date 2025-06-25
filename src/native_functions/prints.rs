use crate::native_functions::native_function;
#[native_function("str")]
pub fn str(a: f64) -> String {
    a.to_string()
}
