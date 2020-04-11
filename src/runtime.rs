use crate::sys;

/// Release global lock
pub fn release_lock() {
    unsafe { sys::memory::caml_enter_blocking_section() }
}

/// Obtain global lock
pub fn acquire_lock() {
    unsafe { sys::memory::caml_leave_blocking_section() }
}
