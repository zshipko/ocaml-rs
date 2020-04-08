use crate::sys;
use crate::value::Value;

use std::ffi::CString;

/// Release global lock
pub fn release_lock() {
    unsafe { sys::memory::caml_enter_blocking_section() }
}

/// Obtain global lock
pub fn acquire_lock() {
    unsafe { sys::memory::caml_leave_blocking_section() }
}

/// Raise OCaml `Failure` with the given string
pub fn failwith<S: AsRef<str>>(arg: S) {
    let s = CString::new(arg.as_ref()).unwrap();
    unsafe { sys::fail::caml_failwith(s.as_ptr()) }
}

/// Raise OCaml `Failure` with the given OCaml string value
pub fn failwith_value(msg: Value) {
    unsafe { sys::fail::caml_failwith_value(msg.0) }
}

/// Raise OCaml `Invalid_argument` with the given function name
pub fn invalid_argument<S: AsRef<str>>(arg: S) {
    let s = CString::new(arg.as_ref()).unwrap();
    unsafe { sys::fail::caml_invalid_argument(s.as_ptr()) }
}

/// Raise OCaml `Invalid_argument` with the given OCaml string containing a function name
pub fn invalid_argument_value(msg: Value) {
    unsafe { sys::fail::caml_invalid_argument_value(msg.0) }
}

/// Raise an existing exception
pub fn raise(bucket: Value) {
    unsafe { sys::fail::caml_raise(bucket.0) }
}

/// Raise a constant exception, specified by the given tag
pub fn raise_constant(tag: Value) {
    unsafe { sys::fail::caml_raise_constant(tag.0) }
}

/// Raise an exception with the given tag and argument
pub fn raise_with_arg(tag: Value, arg: Value) {
    unsafe { sys::fail::caml_raise_with_arg(tag.0, arg.0) }
}

/// Raise an exception with the given tag and message
pub fn raise_with_string<S: AsRef<str>>(tag: Value, msg: S) {
    let s = CString::new(msg.as_ref()).unwrap();
    unsafe { sys::fail::caml_raise_with_string(tag.0, s.as_ptr()) }
}

/// Raise an `Out_of_memory` exception
pub fn raise_out_of_memory() {
    unsafe { sys::fail::caml_raise_out_of_memory() }
}

/// Raise a `Stack_overflow` exception
pub fn raise_stack_overflow() {
    unsafe { sys::fail::caml_raise_stack_overflow() }
}

/// Raise a `Sys_error` exception with the provided argument
pub fn raise_sys_error(arg1: Value) {
    unsafe { sys::fail::caml_raise_sys_error(arg1.0) }
}

/// Raise `End_of_file` exception
pub fn raise_end_of_file() {
    unsafe { sys::fail::caml_raise_end_of_file() }
}

/// Raise `Division_by_zero` exception
pub fn raise_zero_divide() {
    unsafe { sys::fail::caml_raise_zero_divide() }
}

/// Raise `Not_found` exception
pub fn raise_not_found() {
    unsafe { sys::fail::caml_raise_not_found() }
}

/// Raise `Array_bound_error`
pub fn array_bound_error() {
    unsafe { sys::fail::caml_array_bound_error() }
}

/// Raise `Sys_blocked_io`
pub fn raise_sys_blocked_io() {
    unsafe { sys::fail::caml_raise_sys_blocked_io() }
}
