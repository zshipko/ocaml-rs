/// OCaml runtime handle
pub struct Runtime {
    _panic_guard: PanicGuard,
}

thread_local! {
    static RUNTIME_INIT: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
}

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
        #[cfg(not(feature = "no-caml-startup"))]
        {
            if RUNTIME_INIT.with(|x| {
                x.compare_exchange(
                    false,
                    true,
                    core::sync::atomic::Ordering::Relaxed,
                    core::sync::atomic::Ordering::Relaxed,
                )
                .is_err()
            }) {
                return;
            }

            let arg0 = c"ocaml".as_ptr() as *const ocaml_sys::Char;
            let c_args = [arg0, core::ptr::null()];
            unsafe {
                ocaml_sys::caml_startup(c_args.as_ptr());
                assert!(ocaml_boxroot_sys::boxroot_setup());
            }
        }
        #[cfg(feature = "no-caml-startup")]
        panic!("Rust code that is called from an OCaml program should not try to initialize the runtime.");
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
    let rt = Runtime::init();
    rt
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

thread_local! {
    static GUARD_COUNT: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

static INIT: std::sync::Once = std::sync::Once::new();

struct PanicGuard;

impl PanicGuard {
    #[cfg(not(feature = "no-panic-hook"))]
    pub(crate) fn new() -> Self {
        INIT.call_once(|| {
            let original_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |panic_info| {
                if GUARD_COUNT.with(|count| count.get()) > 0 {
                    let err = panic_info.payload();
                    let msg = if let Some(s) = err.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        format!("{:?}", err)
                    };
                    crate::Error::raise_failure(&msg);
                } else {
                    original_hook(panic_info);
                }
            }));
        });

        GUARD_COUNT.with(|count| count.set(count.get() + 1));
        PanicGuard
    }

    #[cfg(feature = "no-panic-hook")]
    pub(crate) fn new() -> Self {
        PanicGuard
    }
}

#[cfg(not(feature = "no-panic-hook"))]
impl Drop for PanicGuard {
    fn drop(&mut self) {
        GUARD_COUNT.with(|count| count.set(count.get() - 1));
    }
}
