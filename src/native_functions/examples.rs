use crate::native_functions::native_function;
use std::time::{SystemTime, UNIX_EPOCH};

/// Clock function that returns the current time in seconds since Unix epoch
#[native_function("clock")]
pub fn clock() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

