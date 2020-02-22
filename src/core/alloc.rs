//! External definitions for allocating values in the OCaml runtime

use crate::core::mlvalues::{Size, Tag, Value};

extern "C" {
    pub fn caml_alloc(size: Size, tag: Tag) -> Value;
    pub fn caml_alloc_small(size: Size, tag: Tag) -> Value;
    pub fn caml_alloc_tuple(size: Size) -> Value;
    pub fn caml_alloc_string(size: Size) -> Value; // size in bytes
    pub fn caml_copy_string(string: *const u8) -> Value;
    pub fn caml_copy_string_array(arr: *const *const u8) -> Value;
    pub fn caml_is_double_array(v: Value) -> i32;

    pub fn caml_copy_double(double: f64) -> Value;
    pub fn caml_copy_int32(int: i32) -> Value; // defined in [ints.c]
    pub fn caml_copy_int64(int: i64) -> Value; // defined in [ints.c]
    pub fn caml_copy_nativeint(int: isize) -> Value; // defined in [ints.c]
    pub fn caml_alloc_array(
        value: unsafe extern "C" fn(*const u8) -> Value,
        array: *mut *mut u8,
    ) -> Value;

    pub fn caml_alloc_final(
        size: Size,
        final_fn: extern "C" fn(Value),
        consumed: Size,
        max: Size,
    ) -> Value;
}
