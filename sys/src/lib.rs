#![allow(clippy::missing_safety_doc)]
#![no_std]

pub type Char = cty::c_char;

#[macro_export]
/// Registers OCaml parameters with the GC
macro_rules! caml_param {
    ($($n:expr),*) => {
        let mut caml_roots = $crate::CamlRootsBlock::default();

        let mut n = 0isize;
        $(
            if n == 5 {
                n = 0;
            }

            if n == 0 {
                caml_roots = $crate::CamlRootsBlock::default();
                #[allow(unused_unsafe)]
                unsafe {
                    caml_roots.next = $crate::local_roots();
                    $crate::set_local_roots(&mut caml_roots);
                };
                caml_roots.nitems = 1;
            }

            caml_roots.tables[n as usize] = &$n as *const _ as *mut _;

            n += 1;
            caml_roots.ntables = n;
        )*
    }
}

#[macro_export]
macro_rules! caml_body {
    ($(($($param:expr),*))? $code:block) => {
        {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::local_roots() };
            $(
                $crate::caml_param!($($param),*);
            )?
            #[allow(unused_mut)]
            let mut res = || $code;
            let res = res();
            #[allow(unused_unsafe)]
            unsafe { $crate::set_local_roots(caml_frame) };
            res
        }
    }
}

#[cfg(not(feature = "without-ocamlopt"))]
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/ocaml_version"));

#[cfg(not(feature = "without-ocamlopt"))]
pub const PATH: &str = include_str!(concat!(env!("OUT_DIR"), "/ocaml_path"));

#[cfg(not(feature = "without-ocamlopt"))]
pub const COMPILER: &str = include_str!(concat!(env!("OUT_DIR"), "/ocaml_compiler"));

mod mlvalues;
#[macro_use]
mod memory;
mod alloc;
pub mod bigarray;
mod callback;
mod custom;
mod fail;
mod runtime;
mod state;
mod tag;

pub use self::mlvalues::Value;
pub use self::tag::Tag;
pub use alloc::*;
pub use callback::*;
pub use custom::*;
pub use fail::*;
pub use memory::*;
pub use mlvalues::*;
pub use runtime::*;
pub use state::*;
pub use tag::*;
