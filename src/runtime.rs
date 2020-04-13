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

/// `Locked` obtains the global lock when created and releases it when dropped
pub struct Locked<'a>(&'a ());

impl<'a> Default for Locked<'a> {
    fn default() -> Locked<'a> {
        Locked::new()
    }
}

impl<'a> Locked<'a> {
    /// Acquire the global lock
    pub fn new() -> Locked<'a> {
        acquire_lock();
        Locked(&())
    }
}

impl<'a> Drop for Locked<'a> {
    fn drop(&mut self) {
        release_lock()
    }
}

/// `Unlocked` releases the global lock when created and obtains it when dropped
pub struct Unlocked<'a>(&'a ());

impl<'a> Default for Unlocked<'a> {
    fn default() -> Unlocked<'a> {
        Unlocked::new()
    }
}

impl<'a> Unlocked<'a> {
    /// Release the global locak
    pub fn new() -> Unlocked<'a> {
        release_lock();
        Unlocked(&())
    }
}

impl<'a> Drop for Unlocked<'a> {
    fn drop(&mut self) {
        acquire_lock()
    }
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
        unsafe { crate::sys::caml_main(c_args.as_ptr()) }
    })
}

/// Shutdown and cleanup OCaml runtime
pub fn shutdown() {
    unsafe { crate::sys::caml_shutdown() }
}
