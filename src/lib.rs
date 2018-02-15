//! Raml is a library for directly interacting with the C OCaml runtime, in Rust.
//! Consquently, ocaml is designed for rust shared objects that expose raw C FFI bindings,
//! which are then either statically or dynamically linked against an OCaml binary, which calls into these raw FFI bindings as if they were
//! regular, so-called "C stubs". Similarly, any OCaml runtime functions, such as `caml_string_length`, will get their definition from the
//! final _OCaml_ binary, with its associated runtime.
//!
//! The benefit of this approach is that it removes any bridging C code, and allows in essence, a direct interface between Rust and OCaml.
//!
//! The macro has the format: `(name, |params...|, <local variables..>, { // code body }, -> optional return value listed in local variables`)
//!
//! A basic example demonstrates their usage:
//!
//! ```rust
//! #[macro_use] extern crate ocaml;
//!
//! // linking against this rust static lib, you can access this function in OCaml via:
//! // `external beef : int -> int = "ml_beef"`
//! caml!(ml_beef, |parameter|, <local>, {
//!     let i = int_val!(parameter);
//!     let res = 0xbeef * i;
//!     println!("about to return  0x{:x} to OCaml runtime", res);
//!     local = val_int!(res);
//! } -> local);
//!
//! // this is only here to satisfy docs
//! fn main() {}
//! ```
//!
//! The macro takes care of _automatically_ declaring `CAMLparam` et. al, as well as `CAMLlocal` and `CAMLreturn`.
//!
//! If you need more fine grained control, `caml_body!` and others are available.
//!
//! Some more examples:
//!
//! ```rust
//! #[macro_use] extern crate ocaml;
//!
//! // these are two functions that OCaml code can access via `external val` declarations
//! caml!(ml_send_int, |v, v2|, <l>, {
//!     let x = int_val!(v);
//!     l = val_int!(0xbeef);
//!     println!("send_int  0x{:x}", x);
//!     // io::stdout().flush();
//! } -> l);
//!
//! caml!(ml_send_two, |v, v2|, {
//!     println!("local root addr: {:p} caml_local_roots: {:#?}, v: {:?}", &ocaml::memory::caml_local_roots, ocaml::memory::caml_local_roots, v);
//!     let x = int_val!(v);
//!     let len = ocaml::mlvalues::caml_string_length(v2);
//!     let ptr = string_val!(v2);
//!     let slice = ::std::slice::from_raw_parts(ptr, len);
//!     let string = ::std::str::from_utf8_unchecked(slice);
//!     println!("got  0x{:x}, {}", x, string);
//! });
//!
//! // this is only here to satisfy docs, you will likely want a staticlib, not a rust executable
//! fn main() {}
//! ```
//!
//! These will be accessible to an OCaml program via `external` declarations, e.g., for the above we could do something like:
//!
//! ```OCaml
//! external send_int : int -> int = "ml_send_int"
//! external send_two : int -> string -> unit = "ml_send_two"
//!
//! let f x = x land 0x0000ffff
//!
//! let _ =
//!   let string = "string thing" in
//!   let deadbeef = 0xdeadbeef in
//!   let res = send_int 0xb1b1eb0b in
//!   Printf.printf "send_int returned: 0x%x\n" res;
//!   flush stdout;
//!   send_two deadbeef string;
//!   send_two (f deadbeef) string
//! ```
//!

mod tag;
mod types;
mod error;

#[macro_use]
pub mod mlvalues;
#[macro_use]
pub mod memory;
pub mod alloc;
pub mod callback;

pub use error::Error;
pub use mlvalues::{Value, is_block, is_long, empty_list};
pub use tag::Tag;
pub use types::{Array, Tuple, List, Str};

