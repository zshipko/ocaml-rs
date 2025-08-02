use crate::sys;

/// Wraps rooted values
pub struct Root(pub ocaml_boxroot_sys::BoxRoot);

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        ocaml_boxroot_sys::boxroot_get_ref(self.0) == ocaml_boxroot_sys::boxroot_get_ref(other.0)
    }
}

impl PartialOrd for Root {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        ocaml_boxroot_sys::boxroot_get_ref(self.0)
            .partial_cmp(&ocaml_boxroot_sys::boxroot_get_ref(other.0))
    }
}

impl core::fmt::Debug for Root {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        ocaml_boxroot_sys::boxroot_get_ref(self.0).fmt(f)
    }
}

impl Eq for Root {}

// Adaptation from C++: https://gitlab.com/ocaml-rust/ocaml-boxroot/-/blob/c119eb0c88f3f628e683ee90f1b694b51c60a0cb/boxroot/cpp/boxroot.cpp
fn boxroot_raise_error() -> ! {
    use ocaml_boxroot_sys::Status;
    let status = unsafe { ocaml_boxroot_sys::boxroot_status() };
    let error_message = match status {
        Status::ToreDown => "boxroot_teardown has previously been called",
        Status::Invalid => "With systhreads, boxroot_setup must be called after caml_thread_initialize but before any thread is created",
        Status::Running | Status::NotSetup => {
            #[cfg(not(feature = "no-std"))]
            {
                use std::io::{Error, ErrorKind};
                match Error::last_os_error().kind() {
                    ErrorKind::PermissionDenied => {
                        "You tried calling boxroot_create or boxroot_modify without holding the domain lock"
                    }
                    ErrorKind::OutOfMemory => {
                        "Allocation failure of the backing store"
                    }
                    _ => "Unknown Error::last_os_error().kind()",
                }
            }
            #[cfg(feature = "no-std")]
            {
                "Unknown error (details unavailable in no-std environment)"
            }
        }
        _ => "Unknown ocaml_boxroot_sys::Status",
    };

    panic!("Boxroot error: {error_message}");
}

impl Root {
    /// Create a new root
    pub unsafe fn new(v: sys::Value) -> Root {
        match ocaml_boxroot_sys::boxroot_create(v) {
            Some(root) => Root(root),
            None => boxroot_raise_error(),
        }
    }

    /// Get value from root
    pub unsafe fn get(&self) -> sys::Value {
        ocaml_boxroot_sys::boxroot_get(self.0)
    }

    /// Modify root
    pub unsafe fn modify(&mut self, v: sys::Value) {
        if !ocaml_boxroot_sys::boxroot_modify(&mut self.0, v) {
            panic!("boxroot_modify failed")
        }
    }
}

impl Clone for Root {
    fn clone(&self) -> Root {
        unsafe { Root::new(self.get()) }
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        unsafe { ocaml_boxroot_sys::boxroot_delete(self.0) }
    }
}
