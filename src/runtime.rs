use crate::sys;

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
/// NOTE: Your application must link the OCaml libraries to be able to use this function
pub fn startup() {
    let args = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap())
        .collect::<Vec<std::ffi::CString>>();

    // convert the strings to raw pointers
    let mut c_args = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut std::os::raw::c_char)
        .collect::<Vec<*mut std::os::raw::c_char>>();
    unsafe { crate::sys::callback::caml_startup(c_args.as_mut_ptr()) }
}

/// Initialize the OCaml runtime, this will all command-line arguments to be available using
/// `Sys.argv`.
///
/// NOTE: Your application must link the OCaml libraries to be able to use this function
pub fn caml_main() {
    let args = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap())
        .collect::<Vec<std::ffi::CString>>();

    // convert the strings to raw pointers
    let mut c_args = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut std::os::raw::c_char)
        .collect::<Vec<*mut std::os::raw::c_char>>();
    unsafe { crate::sys::callback::caml_main(c_args.as_mut_ptr()) }
}

/// Shutdown and cleanup OCaml runtime
pub fn shutdown() {
    unsafe { crate::sys::callback::caml_shutdown() }
}
