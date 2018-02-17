//! ocaml-rs is a library for directly interacting with the C OCaml runtime, in Rust.
//! Consquently, ocaml is designed for rust shared objects that expose raw C FFI bindings,
//! which are then either statically or dynamically linked against an OCaml binary, which calls into these raw FFI bindings as if they were
//! regular, so-called "C stubs". Similarly, any OCaml runtime functions, such as `caml_string_length`, will get their definition from the
//! final _OCaml_ binary, with its associated runtime.
//!

#[macro_use]
pub mod core;
mod tag;
mod types;
mod named;
pub mod value;

#[macro_use]
mod macros;

pub use core::error::Error;
pub use tag::Tag;
pub use types::{Array, Tuple, List, Str};
pub use named::named_value;
pub use value::Value;
