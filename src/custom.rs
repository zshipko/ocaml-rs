#![allow(missing_docs)]

use crate::*;

#[derive(Clone)]
#[repr(C)]
pub struct CustomOps {
    pub identifier: *const ::std::os::raw::c_char,
    pub finalize: ::std::option::Option<unsafe extern "C" fn(v: Value)>,
    pub compare:
        ::std::option::Option<unsafe extern "C" fn(v1: Value, v2: Value) -> ::std::os::raw::c_int>,
    pub hash: ::std::option::Option<unsafe extern "C" fn(v: Value) -> Int>,
    pub serialize: ::std::option::Option<
        unsafe extern "C" fn(v: Value, bsize_32: *mut Uint, bsize_64: *mut Uint),
    >,
    pub deserialize:
        ::std::option::Option<unsafe extern "C" fn(dst: *mut ::std::os::raw::c_void) -> Uint>,
    pub compare_ext:
        ::std::option::Option<unsafe extern "C" fn(v1: Value, v2: Value) -> ::std::os::raw::c_int>,
    pub fixed_length: *const sys::custom::custom_fixed_length,
}

impl Default for CustomOps {
    fn default() -> CustomOps {
        CustomOps {
            identifier: std::ptr::null(),
            finalize: None,
            compare: None,
            hash: None,
            serialize: None,
            deserialize: None,
            compare_ext: None,
            fixed_length: std::ptr::null_mut(),
        }
    }
}

pub struct CustomType {
    pub name: &'static str,
    pub fixed_length: Option<sys::custom::custom_fixed_length>,
    pub ops: CustomOps,
}

pub trait Custom {
    const TYPE: CustomType;
    const USED: usize = 0;
    const MAX: usize = 1;

    fn ops() -> &'static CustomOps {
        Self::TYPE.ops.identifier = Self::TYPE.name.as_ptr() as *const std::os::raw::c_char;
        match Self::TYPE.fixed_length {
            Some(x) => Self::TYPE.ops.fixed_length = &x,
            None => (),
        }
        &Self::TYPE.ops
    }
}

unsafe impl<T: 'static + Clone + Custom> ToValue for T {
    fn to_value(&self) -> Value {
        let mut val: Pointer<T> = Value::alloc_custom(Some((T::USED, T::MAX)));
        val.copy_from(self);
        val.to_value()
    }
}

/// Create a custom OCaml type from an existing Rust type
#[macro_export]
macro_rules! custom {
    ($name:ident { $($k:ident : $v:expr,)* }) => {
        impl $crate::Custom for $name {
            const TYPE: $crate::custom::CustomType = $crate::custom::CustomType {
                name: concat!("rust.", concat!(stringify!(ident), "\0")),
                fixed_length: None,
                ops: $crate::custom::CustomOps {
                    $($k: Some($v),)*
                    .. $crate::custom::CustomOps {
                        identifier: std::ptr::null(),
                        fixed_length: std::ptr::null_mut(),
                        compare: None,
                        compare_ext: None,
                        deserialize: None,
                        finalize: None,
                        hash: None,
                        serialize: None,
                    }
                },
            };
        }
    };
}
