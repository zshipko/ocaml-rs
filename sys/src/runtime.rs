use crate::{Char, Value};

extern "C" {
    pub fn caml_main(argv: *const *const Char);
    pub fn caml_startup(argv: *const *const Char);
    pub fn caml_shutdown();
    pub fn caml_named_value(name: *const Char) -> *const Value;
    pub fn caml_enter_blocking_section();
    pub fn caml_leave_blocking_section();
    pub fn caml_thread_initialize(unit: Value) -> Value;
}

// GC control
extern "C" {
    pub fn caml_gc_minor(v: Value);
    pub fn caml_gc_major(v: Value);
    pub fn caml_gc_full_major(v: Value);
    pub fn caml_gc_compaction(v: Value);
}
