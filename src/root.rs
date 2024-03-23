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

impl Root {
    /// Create a new root
    pub unsafe fn new(v: sys::Value) -> Root {
        Root(ocaml_boxroot_sys::boxroot_create(v).expect("boxroot_create failed"))
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
