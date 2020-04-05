//! ocaml-rs is a library for directly interacting with the C OCaml runtime, in Rust.
//!
//! **Note:** `ocaml-rs` is still experimental, please report any issues on [github](https://github.com/zshipko/ocaml-rs/issues)
//!
//! This crate will allow you to write OCaml C stubs directly in Rust. It is suggested to build
//! a static library to link to when compiling your OCaml code.
//!
//! The following in Rust:
//!
//! ```norun
//! caml!(ml_add_10(arg) {
//!     caml_local!(x);
//!     let n = arg.i32_val();
//!     x = Value::i32(n + 10);
//!     return x;
//! });
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
//! ```norun
//! caml!(build_tuple(i) {
//!     let i = i.val_i32();
//!     Tuple::from(&[i + 1, i + 2, i + 3])
//! });
//!
//! caml!(average(arr) {
//!     let arr: Vec<f64> = Array::from(arr);
//!     let len = arr.len();
//!     let sum = 0f64;
//!
//!     for i in 0..len {
//!         sum += arr[i];
//!     }
//!
//!     Value::f64(sum / len as f64)
//! });
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

/// The `core` module contains the low-level implementation of the OCaml runtime
pub mod core;

#[macro_use]
pub mod macros;

pub mod conv;

mod error;
mod named;
pub mod runtime;
mod types;
pub mod value;

pub use ocaml_fn_derive::ocaml_func as func;

pub use crate::core::tag::{self, Tag};
pub use crate::error::Error;
pub use crate::named::named_value;
pub use crate::runtime::*;
pub use crate::types::{Array1, List};
pub use crate::value::{FromValue, ToValue, Value};

/// Allocate a new value with the given size and tag.
pub fn alloc(n: usize, tag: Tag) -> Value {
    caml_frame!(|x| {
        x.0 = unsafe { core::alloc::caml_alloc(n, tag) };
        x
    })
}

/// Allocate a new tuple value
pub fn alloc_tuple(n: usize) -> Value {
    caml_frame!(|x| {
        x.0 = unsafe { core::alloc::caml_alloc_tuple(n) };
        x
    })
}

/// Allocate a new small value with the given size and tag
pub fn alloc_small(n: usize, tag: Tag) -> Value {
    caml_frame!(|x| {
        x.0 = unsafe { core::alloc::caml_alloc_small(n, tag) };
        x
    })
}

/// Allocate a new value with a custom finalizer
pub fn alloc_custom<T>(value: T, finalizer: extern "C" fn(core::Value)) -> Value {
    caml_frame!(|x| {
        x.0 = unsafe { core::alloc::caml_alloc_final(std::mem::size_of::<T>(), finalizer, 0, 1) };
        let ptr = x.custom_ptr_val_mut::<T>();
        unsafe { std::ptr::write(ptr, value) };
        x
    })
}
