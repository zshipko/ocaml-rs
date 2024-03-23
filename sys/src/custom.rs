use crate::*;

use mlvalues::{Intnat, Size, Uintnat};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct custom_fixed_length {
    pub bsize_32: Intnat,
    pub bsize_64: Intnat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct custom_operations {
    pub identifier: *const Char,
    pub finalize: ::core::option::Option<unsafe extern "C" fn(v: Value)>,
    pub compare: ::core::option::Option<unsafe extern "C" fn(v1: Value, v2: Value) -> i32>,
    pub hash: ::core::option::Option<unsafe extern "C" fn(v: Value) -> Intnat>,
    pub serialize: ::core::option::Option<
        unsafe extern "C" fn(v: Value, bsize_32: *mut Uintnat, bsize_64: *mut Uintnat),
    >,
    pub deserialize:
        ::core::option::Option<unsafe extern "C" fn(dst: *mut ::core::ffi::c_void) -> Uintnat>,
    pub compare_ext: ::core::option::Option<unsafe extern "C" fn(v1: Value, v2: Value) -> i32>,
    pub fixed_length: *const custom_fixed_length,
}
extern "C" {
    pub fn caml_alloc_custom(
        ops: *const custom_operations,
        size: Uintnat,
        mem: Size,
        max: Size,
    ) -> Value;
}
extern "C" {
    pub fn caml_alloc_custom_mem(ops: *mut custom_operations, size: Uintnat, mem: Size) -> Value;
}
extern "C" {
    pub fn caml_register_custom_operations(ops: *mut custom_operations);
}
