//! Callbacks from C to OCaml
//! This is also where you initialize the OCaml runtime system via `caml_startup` or `caml_main`
//!

use core::mlvalues::Value;

extern "C" {
    pub fn caml_callback(closure: Value, arg: Value) -> Value;
    pub fn caml_callback2(closure: Value, arg1: Value, arg2: Value) -> Value;
    pub fn caml_callback3(closure: Value, arg1: Value, arg2: Value, arg3: Value) -> Value;
    pub fn caml_callbackN(closure: Value, narg: usize, args: *mut Value) -> Value;

    pub fn caml_callback_exn(closure: Value, arg1: Value) -> Value;
    pub fn caml_callback2_exn(closure: Value, arg1: Value, arg2: Value) -> Value;
    pub fn caml_callback3_exn(closure: Value, arg1: Value, arg2: Value, arg3: Value) -> Value;
    pub fn caml_callbackN_exn(closure: Value, narg: usize, args: *mut Value) -> Value;

    pub fn caml_main(argv: *mut *mut u8);
    pub fn caml_startup(argv: *mut *mut u8);
    pub fn caml_named_value(name: *const u8) -> *mut Value;

    pub static mut caml_callback_depth: usize;
}

#[macro_export]
macro_rules! make_exception_result {
  ($v:ident) => {
    (v as usize) | 2
  }
}

#[macro_export]
macro_rules! is_exception_result {
  ($v:ident) => {
    (v as usize) & 3 == 2
  }
}

#[macro_export]
macro_rules! extract_exception {
  ($v:ident) => {
    (v as usize) & !3
  }
}

/*
typedef void (*caml_named_action) (value*, char *);
CAMLextern void caml_iterate_named_values(caml_named_action f);
*/
