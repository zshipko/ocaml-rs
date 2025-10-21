/// OCaml runtime handle
pub struct Runtime {
    _panic_guard: PanicGuard,
}

static RUNTIME_INIT: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

impl Runtime {
    /// Initialize the OCaml runtime.
    pub fn init() -> Self {
        Self::init_persistent();
        Self {
            _panic_guard: PanicGuard::new(),
        }
    }

    /// Initializes the OCaml runtime.
    ///
    /// After the first invocation, this method does nothing.
    pub fn init_persistent() {
        #[cfg(not(feature = "no-std"))]
        {
            let no_caml_startup = std::env::var("NO_CAML_STARTUP");
            if no_caml_startup.is_ok() {
                return;
            }
        }

        #[cfg(not(feature = "no-caml-startup"))]
        {
            if RUNTIME_INIT
                .compare_exchange(
                    false,
                    true,
                    core::sync::atomic::Ordering::Relaxed,
                    core::sync::atomic::Ordering::Relaxed,
                )
                .is_err()
            {
                return;
            }

            let arg0 = c"ocaml".as_ptr() as *const ocaml_sys::Char;
            let c_args = [arg0, core::ptr::null()];
            unsafe {
                ocaml_sys::caml_startup(c_args.as_ptr());
                assert!(ocaml_boxroot_sys::boxroot_setup());
            }
        }
    }

    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn recover_handle() -> &'static Self {
        static RUNTIME: Runtime = Runtime {
            _panic_guard: PanicGuard,
        };
        &RUNTIME
    }

    /// Wrapper for `caml_leave_blocking_section`
    pub fn leave_blocking_section(&self) {
        unsafe { crate::sys::caml_leave_blocking_section() }
    }

    /// Wrapper for `caml_leave_blocking_section`
    pub fn enter_blocking_section(&self) {
        unsafe { crate::sys::caml_enter_blocking_section() }
    }
}

/// Initialize the OCaml runtime, the runtime will be
/// freed when the value goes out of scope
pub fn init() -> Runtime {
    Runtime::init()
}

/// Initialize the OCaml runtime
pub fn init_persistent() {
    Runtime::init_persistent();
}

/// Wrapper for `caml_leave_blocking_section`
pub fn leave_blocking_section() {
    unsafe { crate::sys::caml_leave_blocking_section() }
}

/// Wrapper for `caml_leave_blocking_section`
pub fn enter_blocking_section() {
    unsafe { crate::sys::caml_enter_blocking_section() }
}

/// Run minor GC collection
pub fn gc_minor() {
    unsafe {
        ocaml_sys::caml_gc_minor(ocaml_sys::UNIT);
    }
}

/// Run major GC collection
pub unsafe fn gc_major() {
    ocaml_sys::caml_gc_major(ocaml_sys::UNIT);
}

/// Run full major GC collection
pub unsafe fn gc_full_major() {
    ocaml_sys::caml_gc_full_major(ocaml_sys::UNIT);
}

/// Run compaction
pub unsafe fn gc_compact() {
    ocaml_sys::caml_gc_compaction(ocaml_sys::UNIT);
}

#[cfg(not(any(feature = "no-panic-hook", feature = "no-std")))]
thread_local! {
    #[allow(clippy::missing_const_for_thread_local)]
    static GUARD_COUNT: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

#[cfg(not(any(feature = "no-panic-hook", feature = "no-std")))]
static INIT: std::sync::Once = std::sync::Once::new();

#[cfg(not(any(feature = "no-panic-hook", feature = "no-std")))]
const RUST_PANIC_HOOK: &core::ffi::CStr = c"rust_panic_hook";

struct PanicGuard;

impl PanicGuard {
    #[cfg(not(any(feature = "no-panic-hook", feature = "no-std")))]
    pub(crate) fn new() -> Self {
        INIT.call_once(|| {
            let original_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |panic_info| {
                if GUARD_COUNT.with(|count| count.get()) > 0 {
                    let err = panic_info.payload();
                    let msg = if let Some(s) = err.downcast_ref::<&str>() {
                        format!("rust panic: {s}")
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        format!("rust panic: {s}")
                    } else {
                        format!("rust panic: {err:?}")
                    };

                    unsafe {
                        let f = crate::sys::caml_named_value(RUST_PANIC_HOOK.as_ptr() as *const _);
                        if !f.is_null() {
                            let value = crate::sys::caml_alloc_string(msg.len());
                            let ptr = crate::sys::string_val(value);
                            core::ptr::copy_nonoverlapping(msg.as_ptr(), ptr, msg.len());
                            crate::sys::caml_callback(*f, value);
                        }
                        crate::Error::raise_failure(&msg);
                    }
                } else {
                    original_hook(panic_info);
                }
            }));
        });

        GUARD_COUNT.with(|count| count.set(count.get() + 1));
        PanicGuard
    }

    #[cfg(any(feature = "no-panic-hook", feature = "no-std"))]
    pub(crate) fn new() -> Self {
        PanicGuard
    }
}

#[cfg(not(any(feature = "no-panic-hook", feature = "no-std")))]
impl Drop for PanicGuard {
    fn drop(&mut self) {
        GUARD_COUNT.with(|count| count.set(count.get() - 1));
    }
}
