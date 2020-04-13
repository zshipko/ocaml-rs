use crate::sys;

#[cfg(any(feature = "link-native", feature = "link-bytecode"))]
static RUNTIME: std::sync::Once = std::sync::Once::new();

/// Release global lock
pub fn release_lock() {
    unsafe { sys::memory::caml_enter_blocking_section() }
}

/// Obtain global lock
pub fn acquire_lock() {
    unsafe { sys::memory::caml_leave_blocking_section() }
}

/// Initialize the OCaml runtime, this will all command-line arguments to be available using
/// `Sys.argv`.
///
/// This is equivalent to calling `caml_startup`
#[cfg(feature = "link-bytecode")]
pub fn init() {
    RUNTIME.call_once(|| {
        let args = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap())
            .collect::<Vec<std::ffi::CString>>();

        // convert the strings to raw pointers
        let mut c_args = args
            .iter()
            .map(|arg| arg.as_ptr() as *mut std::os::raw::c_char)
            .collect::<Vec<*mut std::os::raw::c_char>>();
        unsafe { crate::sys::callback::caml_startup(c_args.as_mut_ptr()) }
    })
}

/// Initialize the OCaml runtime, this will all command-line arguments to be available using
/// `Sys.argv`.
///
/// This is equivalent to calling `caml_main`
#[cfg(feature = "link-native")]
pub fn init() {
    RUNTIME.call_once(|| {
        let args = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap())
            .collect::<Vec<std::ffi::CString>>();

        // convert the strings to raw pointers
        let mut c_args = args
            .iter()
            .map(|arg| arg.as_ptr() as *mut std::os::raw::c_char)
            .collect::<Vec<*mut std::os::raw::c_char>>();
        unsafe { crate::sys::callback::caml_main(c_args.as_mut_ptr()) }
    })
}

/// Shutdown and cleanup OCaml runtime
pub fn shutdown() {
    unsafe { crate::sys::callback::caml_shutdown() }
}
