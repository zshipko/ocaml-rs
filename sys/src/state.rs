#![allow(non_camel_case_types)]
#[allow(unused)]
use crate::{Char, Value};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caml_ref_table {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caml_ephe_ref_table {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caml_custom_table {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct longjmp_buffer {
    pub _address: u8,
}

pub type backtrace_slot = *mut ::core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
#[cfg(not(feature = "ocaml5"))]
pub struct caml_domain_state {
    pub _young_ptr: *mut Value,
    pub _young_limit: *mut Value,
    pub _exception_pointer: *mut Char,
    pub _young_base: *mut ::core::ffi::c_void,
    pub _young_start: *mut Value,
    pub _young_end: *mut Value,
    pub _young_alloc_start: *mut Value,
    pub _young_alloc_end: *mut Value,
    pub _young_alloc_mid: *mut Value,
    pub _young_trigger: *mut Value,
    pub _minor_heap_wsz: usize,
    pub _in_minor_collection: isize,
    pub _extra_heap_resources_minor: f64,
    pub _ref_table: *mut caml_ref_table,
    pub _ephe_ref_table: *mut caml_ephe_ref_table,
    pub _custom_table: *mut caml_custom_table,
    pub _stack_low: *mut Value,
    pub _stack_high: *mut Value,
    pub _stack_threshold: *mut Value,
    pub _extern_sp: *mut Value,
    pub _trapsp: *mut Value,
    pub _trap_barrier: *mut Value,
    pub _external_raise: *mut longjmp_buffer,
    pub _exn_bucket: Value,
    pub _top_of_stack: *mut Char,
    pub _bottom_of_stack: *mut Char,
    pub _last_return_address: usize,
    pub _gc_regs: *mut Value,
    pub _backtrace_active: isize,
    pub _backtrace_pos: isize,
    pub _backtrace_buffer: *mut backtrace_slot,
    pub _backtrace_last_exn: Value,
    pub _compare_unordered: isize,
    pub _requested_major_slice: isize,
    pub _requested_minor_gc: isize,
    pub _local_roots: *mut crate::memory::CamlRootsBlock,
    pub _stat_minor_words: f64,
    pub _stat_promoted_words: f64,
    pub _stat_major_words: f64,
    pub _stat_minor_collections: isize,
    pub _stat_major_collections: isize,
    pub _stat_heap_wsz: isize,
    pub _stat_top_heap_wsz: isize,
    pub _stat_compactions: isize,
    pub _stat_heap_chunks: isize,
}

#[repr(C)]
#[derive(Debug)]
#[cfg(feature = "ocaml5")]
pub struct caml_domain_state {
    pub _young_limit: core::sync::atomic::AtomicUsize,
    pub _young_ptr: *mut Value,
    pub _exception_pointer: *mut Char,
    pub _young_start: *mut Value,
    pub _young_end: *mut Value,
    pub _current_stack: *mut core::ffi::c_void, // TODO: add `struct stack_info`
    pub _exn_handler: *mut core::ffi::c_void,
    pub _action_pending: core::ffi::c_int,
    pub _c_stack: *mut core::ffi::c_void, // TODO: add `struct c_stack_link`
    pub _stack_cache: *mut *mut core::ffi::c_void,
    pub _gc_regs_buckets: *mut Value,
    pub _gc_regs: *mut Value,
    pub _minor_tables: *mut core::ffi::c_void, // TODO: add `struct caml_minor_tables`
    pub _mark_stack: *mut core::ffi::c_void,   // TODO: add `struct mark_stack`
    pub _marking_done: crate::Uintnat,
    pub _sweeping_done: crate::Uintnat,
    pub _allocated_words: crate::Uintnat,
    pub _swept_words: crate::Uintnat,
    pub _major_work_computed: crate::Intnat,
    pub _major_work_todo: crate::Intnat,
    pub _major_gc_clock: f64,
    pub _local_roots: *mut crate::memory::CamlRootsBlock,
    pub _ephe_info: *mut core::ffi::c_void,
    pub _final_info: *mut core::ffi::c_void,
    pub _backtrace_pos: crate::Intnat,
    pub _backtrace_active: crate::Intnat,
    pub _backtrace_buffer: *mut backtrace_slot,
    pub _backtrace_last_exn: Value,
    pub _compare_unordered: crate::Intnat,
    pub _oo_next_id_local: crate::Uintnat,
    pub _requested_major_slice: crate::Uintnat,
    pub _requested_minor_slice: crate::Uintnat,
    pub _requested_minor_gc: crate::Uintnat,
    pub _requested_external_interrupt: core::sync::atomic::AtomicUsize,
    pub _parser_trace: core::ffi::c_int,
    pub _minor_heap_wsz: usize,
    pub _shared_heap: *mut core::ffi::c_void,
    pub _id: core::ffi::c_int,
    pub _unique_id: core::ffi::c_int,
    pub _dls_root: Value,
    pub _extra_heap_resources: f64,
    pub _extra_heap_resources_minor: f64,
    pub _dependent_size: crate::Uintnat,
    pub _dependent_allocated: crate::Uintnat,
    pub _caml_extern_state: *mut core::ffi::c_void,
    pub _caml_intern_state: *mut core::ffi::c_void,
    pub _stat_minor_words: crate::Uintnat,
    pub _stat_promoted_words: crate::Uintnat,
    pub _stat_major_words: crate::Uintnat,
    pub _stat_minor_collections: crate::Intnat,
    pub _stat_forced_major_collections: crate::Intnat,
    pub _stat_blocks_marked: crate::Uintnat,
    pub _inside_stw_handler: core::ffi::c_int,
    pub _trap_sp_off: crate::Intnat,
    pub _trap_barrier_off: crate::Intnat,
    pub _trap_barrier_block: i64,
    pub _external_raise: *mut core::ffi::c_void,
    pub _extra_params_area: [u8; 0],
}

extern "C" {
    #[doc(hidden)]
    pub fn caml_sys_get_domain_state() -> *mut caml_domain_state;
}

#[doc(hidden)]
pub unsafe fn local_roots() -> *mut crate::memory::CamlRootsBlock {
    (*caml_sys_get_domain_state())._local_roots
}

#[doc(hidden)]
pub unsafe fn set_local_roots(x: *mut crate::memory::CamlRootsBlock) {
    (*caml_sys_get_domain_state())._local_roots = x
}
