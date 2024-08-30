/// OCaml runtime handle
pub struct Runtime {
    _private: (),
}

static RUNTIME_INIT: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

impl Runtime {
    /// Initialize the OCaml runtime.
    pub fn init() -> Self {
        Self::init_persistent();
        Self { _private: () }
    }

    /// Initializes the OCaml runtime.
    ///
    /// After the first invocation, this method does nothing.
    pub fn init_persistent() {
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

            let arg0 = "ocaml\0".as_ptr() as *const ocaml_sys::Char;
            let c_args = [arg0, core::ptr::null()];
            unsafe {
                ocaml_sys::caml_startup(c_args.as_ptr());
                #[cfg(not(feature = "docs-rs"))]
                assert!(ocaml_boxroot_sys::boxroot_setup());
            }
        }
        #[cfg(feature = "no-caml-startup")]
        panic!("Rust code that is called from an OCaml program should not try to initialize the runtime.");
    }

    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn recover_handle() -> &'static Self {
        static RUNTIME: Runtime = Runtime { _private: () };
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
    crate::initial_setup();
    rt
}

/// Initialize the OCaml runtime
pub fn init_persistent() {
    Runtime::init_persistent();
    crate::initial_setup();
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
