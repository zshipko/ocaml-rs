use core::mlvalues::{Size, Value};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CustomOperations {
    pub identifier: *mut ::std::os::raw::c_char,
    pub finalize: ::std::option::Option<unsafe extern "C" fn(v: Value)>,
    pub compare:
        ::std::option::Option<unsafe extern "C" fn(v1: Value, v2: Value) -> ::std::os::raw::c_int>,
    pub hash: ::std::option::Option<unsafe extern "C" fn(v: Value) -> isize>,
    pub serialize: ::std::option::Option<
        unsafe extern "C" fn(v: Value, bsize_32: *mut Size, bsize_64: *mut Size),
    >,
    pub deserialize:
        ::std::option::Option<unsafe extern "C" fn(dst: *mut ::std::os::raw::c_void) -> Size>,
    pub compare_ext:
        ::std::option::Option<unsafe extern "C" fn(v1: Value, v2: Value) -> ::std::os::raw::c_int>,
}

extern "C" {
    pub fn caml_alloc_custom(
        ops: *mut CustomOperations,
        size: Size,
        mem: Size,
        max: Size,
    ) -> Value;
}
extern "C" {
    pub fn caml_register_custom_operations(ops: *mut CustomOperations);
}


