use crate::sys;

#[cfg(feature = "link")]
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
/// This is equivalent to calling `caml_main`
#[cfg(feature = "link")]
pub fn init() {
    RUNTIME.call_once(|| {
        let args = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap())
            .collect::<Vec<std::ffi::CString>>();
        println!("{:?}", args);

        // convert the strings to raw pointers
        let mut c_args = args
            .iter()
            .map(|arg| arg.as_ptr() as *const std::os::raw::c_char)
            .collect::<Vec<*const std::os::raw::c_char>>();
        c_args.push(std::ptr::null());
        unsafe { crate::sys::callback::caml_main(c_args.as_ptr()) }
    })
}

/// Shutdown and cleanup OCaml runtime
pub fn shutdown() {
    unsafe { crate::sys::callback::caml_shutdown() }
}
