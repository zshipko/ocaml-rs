use core;
use value::Value;

use std::ffi::CString;

/// Release global lock
pub fn release_runtime_system() {
    unsafe {
        core::memory::caml_enter_blocking_section()
    }
}

/// Obtain global lock
pub fn acquire_runtime_system() {
    unsafe {
        core::memory::caml_leave_blocking_section()
    }
}

pub fn failwith<S: AsRef<str>>(arg: S) {
    let s = CString::new(arg.as_ref()).unwrap();
    unsafe {
        core::fail::caml_failwith(s.as_ptr())
    }
}

pub fn failwith_value(msg: Value) {
    unsafe {
        core::fail::caml_failwith_value(msg.value())
    }
}

pub fn invalid_argument<S: AsRef<str>>(arg: S) {
    let s = CString::new(arg.as_ref()).unwrap();
    unsafe {
        core::fail::caml_invalid_argument(s.as_ptr())
    }
}

pub fn invalid_argument_value(msg: Value) {
    unsafe {
        core::fail::caml_invalid_argument_value(msg.value())
    }
}

pub fn raise(bucket: Value){
    unsafe {
        core::fail::caml_raise(bucket.value())
    }
}

pub fn raise_constant(tag: Value) {
    unsafe {
        core::fail::caml_raise_constant(tag.value())
    }
}

pub fn raise_with_arg(tag: Value, arg: Value) {
    unsafe {
        core::fail::caml_raise_with_arg(tag.value(), arg.value())
    }
}

pub fn raise_with_string<S: AsRef<str>>(tag: Value, msg: S) {
    let s = CString::new(msg.as_ref()).unwrap();
    unsafe {
        core::fail::caml_raise_with_string(tag.value(), s.as_ptr())
    }
}


pub fn raise_out_of_memory(){
    unsafe {
        core::fail::caml_raise_out_of_memory()
    }
}

pub fn raise_stack_overflow(){
    unsafe {
        core::fail::caml_raise_stack_overflow()
    }
}

pub fn raise_sys_error(arg1: Value){
    unsafe {
        core::fail::caml_raise_sys_error(arg1.value())
    }
}

pub fn raise_end_of_file(){
    unsafe {
        core::fail::caml_raise_end_of_file()
    }
}

pub fn raise_zero_divide(){
    unsafe {
        core::fail::caml_raise_zero_divide()
    }
}

pub fn raise_not_found(){
    unsafe {
        core::fail::caml_raise_not_found()
    }
}

pub fn array_bound_error(){
    unsafe {
        core::fail::caml_array_bound_error()
    }
}

pub fn raise_sys_blocked_io(){
    unsafe {
        core::fail::caml_raise_sys_blocked_io()
    }
}
