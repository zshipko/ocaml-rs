//! ocaml-rs is a library for directly interacting with the C OCaml runtime, in Rust.
//!
//! This crate will allow you to write OCaml C stubs directly in Rust. It is suggested to build
//! a static library to link to when compiling your OCaml code.
//!
//! The following in Rust:
//!
//! ```norun
//! caml!(ml_add_10, |arg|, <result>, {
//!     let n = arg.i32_val();
//!     result = Value::i32(n + 10)
//! } -> result);
//! ```
//!
//! is equivalent to:
//!
//! ```c
//! value ml_add_10(value arg) {
//!     CAMLparam1(arg);
//!     CAMLlocal(result);
//!     int n = int_val(arg);
//!     result = val_int(arg + 10)
//!     CAMLreturn(result);
//! }
//! ```
//!
//! using the traditional C bindings.

#[macro_use]
pub mod core;
mod tag;
mod types;
mod named;
pub mod value;
mod error;
pub mod conv;
pub mod runtime;

#[macro_use]
mod macros;

pub use error::Error;
pub use tag::Tag;
pub use types::{Array, Tuple, List, Str};
pub use named::named_value;
pub use value::{ToValue, FromValue, Value};
pub use runtime::*;
