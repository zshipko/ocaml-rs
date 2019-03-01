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
//!     int n = Int_val(arg);
//!     result = Val_int(arg + 10)
//!     CAMLreturn(result);
//! }
//! ```
//!
//! using the traditional C bindings.
//!
//! For a more complete example see:
//! [https://github.com/zshipko/ocaml-vec](https://github.com/zshipko/ocaml-vec)

#[macro_use]
mod macros;

#[macro_use]
pub mod core;
pub mod conv;
mod error;
mod named;
pub mod runtime;
mod tag;
mod types;
pub mod value;

pub use crate::error::Error;
pub use crate::named::named_value;
pub use crate::runtime::*;
pub use crate::tag::Tag;
pub use crate::types::{Array, Array1, List, Str, Tuple};
pub use crate::value::{FromValue, ToValue, Value};
