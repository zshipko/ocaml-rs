//! Callbacks from C to OCaml
//! This is also where you initialize the OCaml runtime system via `caml_startup` or `caml_main`
//!

use crate::mlvalues::Value;

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
    pub fn caml_shutdown();
    pub fn caml_named_value(name: *const u8) -> *const Value;

    pub static mut caml_callback_depth: usize;
}

pub const fn is_exception_result(val: Value) -> bool {
    (val as usize) & 3 == 2
}

pub const fn extract_exception(val: Value) -> Value {
    (val as usize) & !3
}
