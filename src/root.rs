use crate::sys;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
/// Wraps rooted values
pub struct Root(pub ocaml_boxroot_sys::BoxRoot);

impl Root {
    /// Create a new root
    pub unsafe fn new(v: sys::Value) -> Root {
        Root(ocaml_boxroot_sys::boxroot_create(v).expect("Unable to allocate boxroot"))
    }

    /// Get value from root
    pub unsafe fn get(&self) -> sys::Value {
        ocaml_boxroot_sys::boxroot_get(self.0)
    }

    /// Modify root
    pub unsafe fn modify(&mut self, v: sys::Value) {
        ocaml_boxroot_sys::boxroot_modify(&mut self.0, v);
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
