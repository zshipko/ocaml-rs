use crate::{sys, Runtime};

/// Release global lock
pub fn release_global_lock() {
    unsafe { sys::caml_enter_blocking_section() }
}

/// Obtain global lock
pub fn acquire_global_lock() {
    unsafe { sys::caml_leave_blocking_section() }
}

/// Execute a function with the OCaml global lock
pub fn with_global_lock<T, F: FnOnce() -> T>(f: F) -> T {
    acquire_global_lock();
    let x = f();
    release_global_lock();
    x
}

/// Execute a function without the OCaml global lock
pub fn without_global_lock<T, F: FnOnce() -> T>(f: F) -> T {
    release_global_lock();
    let x = f();
    acquire_global_lock();
    x
}

/// Initialize the OCaml runtime, the runtime will be
/// freed when the value goes out of scope
#[cfg(not(feature = "no-std"))]
pub fn init() -> Runtime {
    Runtime::init()
}

/// Initialize the OCaml runtime
#[cfg(not(feature = "no-std"))]
pub fn init_persistent() {
    Runtime::init_persistent()
}
