//! ocaml-rs is a library for directly interacting with the C OCaml runtime, in Rust.
//!
//! **Note:** `ocaml-rs` is still experimental, please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)
//!
//! This crate will allow you to write OCaml C stubs directly in Rust. It is suggested to build
//! a static library to link to when compiling your OCaml code.
//!
//! The following in Rust:
//!
//! ```rust,no_run
//! #[ocaml::func]
//! pub fn ml_add_10(arg: ocaml::Int) -> ocaml::Int {
//!     arg + 10
//! };
//! ```
//!
//! is equivalent to:
//!
//! ```c
//! value ml_add_10(value arg) {
//!     CAMLparam1(arg);
//!     CAMLlocal(result);
//!     int n = Int_val(arg);
//!     result = Val_int(arg + 10)
//!     CAMLreturn(result);
//! }
//! ```
//!
//! using the traditional C bindings.
//!
//! Here are a few more examples...
//!
//! ```rust,no_run
//! #[ocaml::func]
//! pub fn build_tuple(i: ocaml::Int) -> (ocaml::Int, ocaml::Int, ocaml::Int) {
//!     (i + 1, i + 2, i + 3)
//! }
//!
//! #[ocaml::func]
//! pub fn average(arr: ocaml::Value) -> Result<f64, ocaml::Error> {
//!     let len = ocaml::array::len(arr);
//!     let mut sum = 0f64;
//!
//!     for i in 0..len {
//!         sum += ocaml::array::get_double(arr, i)?;
//!     }
//!
//!     Ok(sum / len as f64)
//! }
//! ```
//!
//! In OCaml the stubs for these functions looks like this:
//!
//! ```ocaml
//! external build_tuple: int -> int * int * int = "build_tuple"
//! external average: float array -> float = "average"
//! ```
//!
//! For more examples see [./example](https://github.com/zshipko/ocaml-rs/blob/master/example)
//! or [ocaml-vec](https://github.com/zshipko/ocaml-vec).
//!
//!

/// The `sys` module contains the low-level implementation of the OCaml runtime
pub use ocaml_sys as sys;

#[macro_use]
pub mod macros;

pub mod conv;

mod error;
mod named;
pub mod runtime;
mod types;
pub mod value;

#[cfg(feature = "derive")]
pub use ocaml_fn_derive::ocaml_func as func;

pub use crate::error::Error;
pub use crate::named::named_value;
pub use crate::runtime::*;
pub use crate::types::Array1;
pub use crate::value::{FromValue, ToValue, Value};
pub use sys::mlvalues::{Intnat as Int, Uintnat as Uint};
pub use sys::tag::{self, Tag};

/// Allocate a new value with the given size and tag.
pub fn alloc(n: usize, tag: Tag) -> Value {
    Value(sys::caml_frame!(|x| {
        x = unsafe { sys::alloc::caml_alloc(n, tag) };
        x
    }))
}

/// Allocate a new tuple value
pub fn alloc_tuple(n: usize) -> Value {
    Value(sys::caml_frame!(|x| {
        x = unsafe { sys::alloc::caml_alloc_tuple(n) };
        x
    }))
}

/// Allocate a new small value with the given size and tag
pub fn alloc_small(n: usize, tag: Tag) -> Value {
    Value(sys::caml_frame!(|x| {
        x = unsafe { sys::alloc::caml_alloc_small(n, tag) };
        x
    }))
}

/// Allocate a new value with a custom finalizer
pub fn alloc_custom<T>(value: T, finalizer: extern "C" fn(Value)) -> Value {
    let x = Value(sys::caml_frame!(|x| {
        x = unsafe {
            sys::alloc::caml_alloc_final(
                std::mem::size_of::<T>(),
                std::mem::transmute(finalizer),
                0,
                1,
            )
        };
        x
    }));

    let ptr = x.custom_ptr_val_mut::<T>();
    unsafe { std::ptr::write(ptr, value) };
    x
}

pub mod list {
    use crate::*;

    #[inline(always)]
    pub const fn empty() -> Value {
        Value(sys::mlvalues::EMPTY_LIST)
    }

    /// Returns the number of items in `self`
    pub fn len(value: Value) -> usize {
        let mut length = 0;
        let mut tmp = value;
        while tmp.0 != sys::mlvalues::EMPTY_LIST {
            tmp = tmp.field(1);
            length += 1;
        }
        length
    }

    /// Add an element to the front of the list
    pub fn push_hd<T: ToValue>(a: &mut Value, v: T) {
        let tmp = sys::caml_frame!(|x, tmp| {
            x = a.0;
            unsafe {
                tmp = crate::sys::alloc::caml_alloc_small(2, 0);
                crate::sys::memory::store_field(tmp, 0, v.to_value().0);
                crate::sys::memory::store_field(tmp, 1, x);
                tmp
            }
        });

        a.0 = tmp;
    }

    /// List head
    pub fn hd<T: FromValue>(value: Value) -> Option<T> {
        if value == empty() {
            return None;
        }

        Some(T::from_value(value.field(0)))
    }

    /// List tail
    pub fn tl(value: Value) -> Value {
        if value == empty() {
            return empty();
        }

        value.field(1)
    }

    /// List as vector
    pub fn to_vec<T: FromValue>(mut value: Value) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        while value != empty() {
            let val = value.field(0);
            vec.push(T::from_value(val));
            value = value.field(1);
        }
        vec
    }
}

pub mod array {
    use crate::*;
    pub fn new(n: usize) -> Value {
        let x = sys::caml_frame!(|x| {
            x = unsafe { sys::alloc::caml_alloc(n, 0) };
            x
        });
        Value(x)
    }

    /// Check if Array contains only doubles
    pub fn is_double_array(value: Value) -> bool {
        unsafe { sys::alloc::caml_is_double_array(value.0) == 1 }
    }

    /// Array length
    pub fn len(value: Value) -> usize {
        unsafe { sys::mlvalues::caml_array_length(value.0) }
    }

    pub fn is_empty(value: Value) -> bool {
        len(value) == 0
    }

    /// Set value to double array
    pub fn set_double(value: &mut Value, i: usize, f: f64) -> Result<(), Error> {
        if i < len(*value) {
            if !is_double_array(*value) {
                return Err(Error::NotDoubleArray);
            }

            unsafe {
                let ptr = value.ptr_val::<f64>().add(i) as *mut f64;
                *ptr = f;
            };

            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get a value from a double array
    pub fn get_double(value: Value, i: usize) -> Result<f64, Error> {
        if i < len(value) {
            if !is_double_array(value) {
                return Err(Error::NotDoubleArray);
            }

            Ok(unsafe { get_double_unchecked(value, i) })
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get a value from a double array without checking if the array is actually a double array
    pub unsafe fn get_double_unchecked(value: Value, i: usize) -> f64 {
        *value.ptr_val::<f64>().add(i)
    }

    /// Set array index
    pub fn set<T: ToValue>(value: &mut Value, i: usize, v: T) -> Result<(), Error> {
        if i < len(*value) {
            value.store_field(i, v);
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get array index
    pub fn get<T: FromValue>(value: Value, i: usize) -> Result<T, Error> {
        if i < len(value) {
            Ok(unsafe { get_unchecked::<T>(value, i) })
        } else {
            Err(Error::OutOfBounds)
        }
    }

    /// Get array index without bounds checking
    pub unsafe fn get_unchecked<T: FromValue>(value: Value, i: usize) -> T {
        T::from_value(value.field(i))
    }
}
