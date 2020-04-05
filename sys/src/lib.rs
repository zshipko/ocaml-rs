#![allow(clippy::missing_safety_doc)]

#[macro_export]
/// Registers OCaml parameters with the GC
macro_rules! caml_param {

    (@step $idx:expr, $caml_roots:ident,) => {
        $caml_roots.ntables = $idx;
    };

    (@step $idx:expr, $caml_roots:ident, $param:expr, $($tail:expr,)*) => {
        $caml_roots.tables[$idx] = &mut $param;
        $crate::caml_param!(@step $idx + 1usize, $caml_roots, $($tail,)*);
    };

    ($($n:expr),*) => {
        let mut caml_roots = $crate::memory::CamlRootsBlock::default();
        #[allow(unused_unsafe)]
        unsafe {
            caml_roots.next = $crate::state::local_roots();
            $crate::state::set_local_roots(&mut caml_roots);
        };
        caml_roots.nitems = 1; // this is = N when CAMLxparamN is used
        $crate::caml_param!(@step 0usize, caml_roots, $($n,)*);
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
/// Defines an OCaml frame
macro_rules! caml_frame {
    (|$($local:ident),*| $code:block) => {
        {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::state::local_roots() };
            $crate::caml_local!($($local),*);
            let res = $code;
            #[allow(unused_unsafe)]
            unsafe { $crate::state::set_local_roots(caml_frame) };
            res
        }
    };
}

#[macro_export]
macro_rules! caml_body {
    (($($param:expr),*) $code:block) => {
        {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::state::local_roots() };
            $crate::caml_param!($($param),*);
            let res = || $code;
            let res = res();
            #[allow(unused_unsafe)]
            unsafe { $crate::state::set_local_roots(caml_frame) };
            res
        }
    }
}

pub const VERSION: &str = stringify!(include!(concat!(env!("OUT_DIR"), "version")));

pub mod mlvalues;
#[macro_use]
pub mod memory;
pub mod alloc;
pub mod bigarray;
pub mod callback;
pub mod fail;
pub mod state;
pub mod tag;

pub use self::mlvalues::Value;
pub use self::tag::Tag;
