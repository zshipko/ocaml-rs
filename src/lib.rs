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
//! When constructing an `ocaml::func` any type that implements `FromValue` can be used as a
//! parameter and any type that implements `ToValue` can be used as a return type
//!
//! Here are a few more examples...
//!
//! ```rust,no_run
//! // Automatically derive `ToValue` and `FromValue`
//! #[derive(ocaml::ToValue, ocaml::FromValue)]
//! struct Example<'a> {
//!     name: &'a str,
//!     i: ocaml::Int,
//! }
//!
//! #[ocaml::func]
//! pub fn struct_example(e: Example) -> ocaml::Int {
//!     e.i + 1
//! }
//!
//! #[ocaml::func]
//! pub fn build_tuple(i: ocaml::Int) -> (ocaml::Int, ocaml::Int, ocaml::Int) {
//!     (i + 1, i + 2, i + 3)
//! }
//!
//! #[ocaml::func]
//! pub fn average(arr: ocaml::Array<f64>) -> Result<f64, ocaml::Error> {
//!     let mut sum = 0f64;
//!
//!     for i in 0..arr.len() {
//!         sum += arr.get_double(i)?;
//!     }
//!
//!     Ok(sum / arr.len() as f64)
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

#[cfg(feature = "derive")]
pub use ocaml_derive::{ocaml_bare_func as bare_func, ocaml_func as func, FromValue, ToValue};

#[macro_use]
pub mod macros;

pub mod conv;

mod error;
mod named;
pub mod runtime;
mod types;
pub mod value;

pub use crate::error::Error;
pub use crate::named::named_value;
pub use crate::runtime::*;
pub use crate::types::{Array, Array1, List, Opaque};
pub use crate::value::{FromValue, ToValue, Value};
pub use sys::mlvalues::{Intnat as Int, Uintnat as Uint};
pub use sys::tag::{self, Tag};
