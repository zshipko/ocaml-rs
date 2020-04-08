//! ocaml-rs is a library for directly interacting with the C OCaml runtime, in Rust.
//!
//! ## Examples
//!
//! ```rust,no_run
//! // Automatically derive `ToValue` and `FromValue`
//! #[cfg(feature = "derive")]
//! #[derive(ocaml::ToValue, ocaml::FromValue)]
//! struct Example<'a> {
//!     name: &'a str,
//!     i: ocaml::Int,
//! }
//!
//! #[cfg(feature = "derive")]
//! #[ocaml::func]
//! pub fn incr_example(mut e: Example<'static>) -> Example<'static> {
//!     e.i += 1;
//!     e
//! }
//!
//! #[cfg(feature = "derive")]
//! #[ocaml::func]
//! pub fn build_tuple(i: ocaml::Int) -> (ocaml::Int, ocaml::Int, ocaml::Int) {
//!     (i + 1, i + 2, i + 3)
//! }
//!
//! #[cfg(feature = "derive")]
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
//!
//! // A `native_func` must take `ocaml::Value` for every argument or `f64` for every unboxed argument
//! // and return an `ocaml::Value` (or `f64`). `native_func`  minimal overhead compared to wrapping with `func`
//! #[cfg(feature = "derive")]
//! #[ocaml::native_func]
//! pub fn incr(value: ocaml::Value) -> ocaml::Value {
//!     let i = value.int_val();
//!     ocaml::Value::int(i + 1)
//! }
//!
//! // This is equivalent to:
//! #[no_mangle]
//! pub extern "C" fn incr2(value: ocaml::Value) -> ocaml::Value {
//!     ocaml::body!((value) {
//!         let i = value.int_val();
//!         ocaml::Value::int( i + 1)
//!     })
//! }
//!
//! // `ocaml::native_func` is responsible for:
//! // - Ensures that #[no_mangle] and extern "C" are added, in addition to wrapping
//! // - Wraps the function body using `ocaml::body!`
//!
//! // Finally, if your function is marked [@@unboxed] and [@@noalloc] in OCaml then you can avoid
//! // boxing altogether for f64 arguments using a plain C function and a bytecode function
//! // definition:
//! #[no_mangle]
//! pub extern "C" fn incrf(input: f64) -> f64 {
//!     input + 1.0
//! }
//!
//! #[cfg(feature = "derive")]
//! #[ocaml::bytecode_func]
//! pub fn incrf_bytecode(input: f64) -> f64 {
//!     incrf(input)
//! }
//! ```
//!
//! The OCaml stubs would look like this:
//!
//! ```ocaml
//! type example = {
//!     name: string;
//!     i: int;
//! }
//!
//! external incr_example: example -> example = "incr_example"
//! external build_tuple: int -> int * int * int = "build_tuple"
//! external average: float array -> float = "average"
//! external incr: int -> int = "incr"
//! external incr2: int -> int = "incr2"
//! external incrf: float -> float = "incrf_bytecode" "incrf" [@@unboxed] [@@noalloc]
//! ```

/// The `sys` module contains the low-level implementation of the OCaml runtime
pub use ocaml_sys as sys;

#[cfg(feature = "derive")]
pub use ocaml_derive::{
    ocaml_bytecode_func as bytecode_func, ocaml_func as func, ocaml_native_func as native_func,
    FromValue, ToValue,
};

#[macro_use]
pub mod macros;

pub mod conv;

mod error;
mod runtime;
mod types;
pub mod value;

pub use crate::error::Error;
pub use crate::runtime::*;
pub use crate::types::{bigarray, Array, List, Opaque};
pub use crate::value::{FromValue, ToValue, Value};
pub use sys::mlvalues::{Intnat as Int, Uintnat as Uint};
pub use sys::tag::{self, Tag};
