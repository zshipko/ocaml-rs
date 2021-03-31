use crate::{sys, Runtime};

#[cfg(not(feature = "no-std"))]
static RUNTIME: std::sync::Once = std::sync::Once::new();

/// Release global lock
pub fn release_lock() {
    unsafe { sys::caml_enter_blocking_section() }
}

/// Obtain global lock
pub fn acquire_lock() {
    unsafe { sys::caml_leave_blocking_section() }
}

/// Execute a function with the OCaml global lock
pub fn locked<T, F: FnOnce() -> T>(f: F) -> T {
    acquire_lock();
    let x = f();
    release_lock();
    x
}

/// Execute a function without the OCaml global lock
pub fn unlocked<T, F: FnOnce() -> T>(f: F) -> T {
    release_lock();
    let x = f();
    acquire_lock();
    x
}

/// Initialize the OCaml runtime
#[cfg(not(feature = "no-std"))]
pub fn init() -> Runtime {
    Runtime::init()
}
