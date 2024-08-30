use crate::sys;

/// Wraps rooted values
pub struct Root(#[cfg(not(feature = "docs-rs"))] pub ocaml_boxroot_sys::BoxRoot);

impl PartialEq for Root {
    #[cfg(not(feature = "docs-rs"))]
    fn eq(&self, other: &Self) -> bool {
        ocaml_boxroot_sys::boxroot_get_ref(self.0) == ocaml_boxroot_sys::boxroot_get_ref(other.0)
    }

    #[cfg(feature = "docs-rs")]
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialOrd for Root {
    #[cfg(not(feature = "docs-rs"))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        ocaml_boxroot_sys::boxroot_get_ref(self.0)
            .partial_cmp(&ocaml_boxroot_sys::boxroot_get_ref(other.0))
    }

    #[cfg(feature = "docs-rs")]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        None
    }
}

impl core::fmt::Debug for Root {
    #[cfg(not(feature = "docs-rs"))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        ocaml_boxroot_sys::boxroot_get_ref(self.0).fmt(f)
    }

    #[cfg(feature = "docs-rs")]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl Eq for Root {}

impl Root {
    /// Create a new root
    pub unsafe fn new(v: sys::Value) -> Root {
        Root(
            #[cfg(not(feature = "docs-rs"))]
            ocaml_boxroot_sys::boxroot_create(v).expect("boxroot_create failed"),
        )
    }

    /// Get value from root
    #[cfg(not(feature = "docs-rs"))]
    pub unsafe fn get(&self) -> sys::Value {
        ocaml_boxroot_sys::boxroot_get(self.0)
    }

    /// Get value from root
    #[cfg(feature = "docs-rs")]
    pub unsafe fn get(&self) -> sys::Value {
        0
    }

    /// Modify root
    pub unsafe fn modify(&mut self, v: sys::Value) {
        #[cfg(not(feature = "docs-rs"))]
        if !ocaml_boxroot_sys::boxroot_modify(&mut self.0, v) {
            panic!("boxroot_modify failed")
        }
    }
}

impl Clone for Root {
    #[cfg(not(feature = "docs-rs"))]
    fn clone(&self) -> Root {
        unsafe { Root::new(self.get()) }
    }

    #[cfg(feature = "docs-rs")]
    fn clone(&self) -> Root {
        Root()
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        #[cfg(not(feature = "docs-rs"))]
        unsafe {
            ocaml_boxroot_sys::boxroot_delete(self.0)
        }
    }
}
