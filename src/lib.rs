//! ocaml-rs is a library for directly interacting with the C OCaml runtime, in Rust.
//!
//! ## Examples
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
//! The OCaml stubs would look like this:
//!
//! ```ocaml
//! type example = {
//!     name: string;
//!     i: int;
//! }
//!
//! external struct_example: example -> int = "struct_example"
//! external build_tuple: int -> int * int * int = "build_tuple"
//! external average: float array -> float = "average"
//! ```
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
