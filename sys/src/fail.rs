use crate::mlvalues::Value;

extern "C" {
    pub fn caml_raise(bucket: Value);
    pub fn caml_raise_constant(tag: Value);
    pub fn caml_raise_with_arg(tag: Value, arg: Value);
    pub fn caml_raise_with_args(tag: Value, nargs: ::std::os::raw::c_int, arg: *mut Value);
    pub fn caml_raise_with_string(tag: Value, msg: *const ::std::os::raw::c_char);
    pub fn caml_failwith(msg: *const ::std::os::raw::c_char);
    pub fn caml_failwith_value(msg: Value);
    pub fn caml_invalid_argument(msg: *const ::std::os::raw::c_char);
    pub fn caml_invalid_argument_value(msg: Value);
    pub fn caml_raise_out_of_memory();
    pub fn caml_raise_stack_overflow();
    pub fn caml_raise_sys_error(arg1: Value);
    pub fn caml_raise_end_of_file();
    pub fn caml_raise_zero_divide();
    pub fn caml_raise_not_found();
    pub fn caml_array_bound_error();
    pub fn caml_raise_sys_blocked_io();
}

pub const OUT_OF_MEMORY_EXN: i32 = 0;
pub const SYS_ERROR_EXN: i32 = 1;
pub const FAILURE_EXN: i32 = 2;
pub const INVALID_EXN: i32 = 3;
pub const END_OF_FILE_EXN: i32 = 4;
pub const ZERO_DIVIDE_EXN: i32 = 5;
pub const NOT_FOUND_EXN: i32 = 6;
pub const MATCH_FAILURE_EXN: i32 = 7;
pub const STACK_OVERFLOW_EXN: i32 = 8;
pub const SYS_BLOCKED_IO: i32 = 9;
pub const ASSERT_FAILURE_EXN: i32 = 10;
pub const UNDEFINIED_RECURSIVE_MODULE_EXN: i32 = 11;
