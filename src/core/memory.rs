//! Defines types and macros primarily for interacting with the OCaml GC.
//! In addition, a few extra convenience macros are added, in particular, `caml!` and `caml_body!` which are the primary API endpoints of raml.
//!
//! # `CAMLParam` Macros
//! The following macros are used to declare C local variables and
//! function parameters of type [value].
//!
//! The function body must start with one of the [CAMLparam] macros.
//! If the function has no parameter of type [value], use [CAMLparam0].
//! If the function has 1 to 5 [value] parameters, use the corresponding
//!
//! [CAMLparam] with the parameters as arguments.
//! If the function has more than 5 [value] parameters, use [CAMLparam5]
//! for the first 5 parameters, and one or more calls to the [CAMLxparam]
//! macros for the others.
//!
//! If the function takes an array of [value]s as argument, use
//! [CAMLparamN] to declare it (or [CAMLxparamN] if you already have a
//! call to [CAMLparam] for some other arguments).
//!
//! If you need local variables of type [value], declare them with one
//! or more calls to the [CAMLlocal] macros at the beginning of the
//! function, after the call to CAMLparam.  Use [CAMLlocalN] (at the
//! beginning of the function) to declare an array of [value]s.
//!
//! Your function may raise an exception or return a [value] with the
//! [CAMLreturn] macro.  Its argument is simply the [value] returned by
//! your function.  Do NOT directly return a [value] with the [return]
//! keyword.  If your function returns void, use [CAMLreturn0].
//!
//! All the identifiers beginning with "caml__" are reserved by OCaml.
//! Do not use them for anything (local or global variables, struct or
//! union tags, macros, etc.)
//!

use std::default::Default;
use std::ptr;

use core::mlvalues::{Size, Value};

#[repr(C)]
#[derive(Debug, Clone)]
/// The GC root struct. **WARNING**: You should seriously not mess with this...
///
/// The fields need to be public because the macros need to access them, which means they're out of the module; in a future version, perhaps we will add methods on the struct, and avoid any `pub` exposure of the fields.
pub struct CamlRootsBlock {
    pub next: *mut CamlRootsBlock,
    pub ntables: usize,
    pub nitems: usize,
    pub tables: [*mut Value; 5],
}

impl Default for CamlRootsBlock {
    fn default() -> CamlRootsBlock {
        CamlRootsBlock {
            next: ptr::null_mut(),
            ntables: 0,
            nitems: 0,
            tables: [ptr::null_mut(); 5],
        }
    }
}

/// These receive their implementations when you successfully link against an OCaml binary
extern "C" {
    pub static mut caml_local_roots: *mut CamlRootsBlock;
    pub fn caml_modify(addr: *mut Value, value: Value);
}

/// Stores the `$val` at `$offset` in the `$block`.
///
/// # Example
/// ```norun
/// // stores some_value in the first field in the given block
/// store_field!(some_block, 1, some_value)
/// ```
macro_rules! store_field {
    ($block:expr, $offset:expr, $val:expr) => (
      $crate::core::memory::caml_modify (field!($block, $offset), $val);
    );
}

/// Stores the `value` in the `block` at `offset`.
///
/// ## Original C code
///
/// ```c
/// Store_field(block, offset, val) do{ \
///   mlsize_t caml__temp_offset = (offset); \
///   value caml__temp_val = (val); \
///   caml_modify (&Field ((block), caml__temp_offset), caml__temp_val); \
/// }while(0)
/// ```
///
pub unsafe fn store_field(block: Value, offset: Size, value: Value) {
    store_field!(block, offset as isize, value);
}


