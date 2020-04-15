use crate::sys;

#[cfg(feature = "link")]
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

/// Initialize the OCaml runtime, this will all command-line arguments to be available using
/// `Sys.argv`.
///
/// This is equivalent to calling `caml_main`
#[cfg(feature = "link")]
pub fn init() {
    RUNTIME.call_once(|| {
        let args = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap())
            .collect::<Vec<std::ffi::CString>>();

        // convert the strings to raw pointers
        let mut c_args = args
            .iter()
            .map(|arg| arg.as_ptr() as *const std::os::raw::c_char)
            .collect::<Vec<*const std::os::raw::c_char>>();
        c_args.push(std::ptr::null());
        unsafe { crate::sys::caml_main(c_args.as_ptr()) }
    })
}

/// Shutdown and cleanup OCaml runtime
pub fn shutdown() {
    unsafe { crate::sys::caml_shutdown() }
}
