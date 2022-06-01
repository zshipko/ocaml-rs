use crate::Runtime;

/// Initialize the OCaml runtime, the runtime will be
/// freed when the value goes out of scope
pub fn init() -> Runtime {
    #[cfg(not(feature = "no-std"))]
    crate::initial_setup();

    Runtime::init()
}

/// Initialize the OCaml runtime
pub fn init_persistent() {
    #[cfg(not(feature = "no-std"))]
    crate::initial_setup();

    Runtime::init_persistent()
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

/// Initialize the OCaml runtime when
pub fn startup() -> Runtime {
    crate::macros::initial_setup();
    Runtime::init()
}
