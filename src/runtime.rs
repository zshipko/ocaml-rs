use crate::value::Value;
use crate::{sys, FromValue};

use std::ffi::CString;

/// Hash variant name
pub fn hash_variant<S: AsRef<str>>(name: S) -> Value {
    unsafe { Value::new(sys::mlvalues::caml_hash_variant(name.as_ref().as_ptr())) }
}

/// Hash variant name as Rust `str`
pub fn hash_variant_str<'a, S: AsRef<str>>(name: S) -> &'a str {
    FromValue::from_value(hash_variant(name))
}

/// Release global lock
pub fn release_runtime_system() {
    unsafe { sys::memory::caml_enter_blocking_section() }
}

/// Obtain global lock
pub fn acquire_runtime_system() {
    unsafe { sys::memory::caml_leave_blocking_section() }
}

pub fn failwith<S: AsRef<str>>(arg: S) {
    let s = CString::new(arg.as_ref()).unwrap();
    unsafe { sys::fail::caml_failwith(s.as_ptr()) }
}

pub fn failwith_value(msg: Value) {
    unsafe { sys::fail::caml_failwith_value(msg.0) }
}

pub fn invalid_argument<S: AsRef<str>>(arg: S) {
    let s = CString::new(arg.as_ref()).unwrap();
    unsafe { sys::fail::caml_invalid_argument(s.as_ptr()) }
}

pub fn invalid_argument_value(msg: Value) {
    unsafe { sys::fail::caml_invalid_argument_value(msg.0) }
}

pub fn raise(bucket: Value) {
    unsafe { sys::fail::caml_raise(bucket.0) }
}

pub fn raise_constant(tag: Value) {
    unsafe { sys::fail::caml_raise_constant(tag.0) }
}

pub fn raise_with_arg(tag: Value, arg: Value) {
    unsafe { sys::fail::caml_raise_with_arg(tag.0, arg.0) }
}

pub fn raise_with_string<S: AsRef<str>>(tag: Value, msg: S) {
    let s = CString::new(msg.as_ref()).unwrap();
    unsafe { sys::fail::caml_raise_with_string(tag.0, s.as_ptr()) }
}

pub fn raise_out_of_memory() {
    unsafe { sys::fail::caml_raise_out_of_memory() }
}

pub fn raise_stack_overflow() {
    unsafe { sys::fail::caml_raise_stack_overflow() }
}

pub fn raise_sys_error(arg1: Value) {
    unsafe { sys::fail::caml_raise_sys_error(arg1.0) }
}

pub fn raise_end_of_file() {
    unsafe { sys::fail::caml_raise_end_of_file() }
}

pub fn raise_zero_divide() {
    unsafe { sys::fail::caml_raise_zero_divide() }
}

pub fn raise_not_found() {
    unsafe { sys::fail::caml_raise_not_found() }
}

pub fn array_bound_error() {
    unsafe { sys::fail::caml_array_bound_error() }
}

pub fn raise_sys_blocked_io() {
    unsafe { sys::fail::caml_raise_sys_blocked_io() }
}
