#![allow(clippy::missing_safety_doc)]
#![no_std]

#[macro_export]
/// Registers OCaml parameters with the GC
macro_rules! caml_param {
    ($($n:expr),*) => {
        let mut caml_roots = $crate::memory::CamlRootsBlock::default();

        let mut n = 0;
        $(
            if n == 5 {
                n = 0;
            }

            if n == 0 {
                caml_roots = $crate::memory::CamlRootsBlock::default();
                #[allow(unused_unsafe)]
                unsafe {
                    caml_roots.next = $crate::state::local_roots();
                    $crate::state::set_local_roots(&mut caml_roots);
                };
                caml_roots.nitems = 1;
            }

            caml_roots.tables[n] = &$n as *const _ as *mut _;

            n += 1;
            caml_roots.ntables = n;
        )*
    }
}

/// Initializes and registers the given identifier(s) as a local value with the OCaml runtime.
///
/// ## Original C code
///
/// ```c
/// #define CAMLlocal1(x) \
/// value x = Val_unit; \
/// CAMLxparam1 (x)
/// ```
///
#[macro_export]
macro_rules! caml_local {
    ($($local:ident),*) => {
        #[allow(unused_mut)]
        $(let mut $local = $crate::mlvalues::UNIT;)*
        #[allow(unused_unsafe)]
        $crate::caml_param!($($local),*);
    }
}

#[macro_export]
macro_rules! caml_body {
    (($($param:expr),*) $code:block) => {
        {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::state::local_roots() };
            $crate::caml_param!($($param),*);
            #[allow(unused_mut)]
            let mut res = || $code;
            let res = res();
            #[allow(unused_unsafe)]
            unsafe { $crate::state::set_local_roots(caml_frame) };
            res
        }
    }
}

pub const VERSION: &str = stringify!(include!(concat!(env!("OUT_DIR"), "ocaml_version")));
pub const PATH: &str = stringify!(include!(concat!(env!("OUT_DIR"), "ocaml_path")));
pub const COMPILER: &str = stringify!(include!(concat!(env!("OUT_DIR"), "ocaml_compiler")));

pub mod mlvalues;
#[macro_use]
pub mod memory;
pub mod alloc;
pub mod bigarray;
pub mod callback;
pub mod custom;
pub mod fail;
pub mod state;
pub mod tag;

pub use self::mlvalues::Value;
pub use self::tag::Tag;
