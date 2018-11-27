#[macro_export]
macro_rules! caml_ffi {
    ($code:tt) => {
        let mut caml_frame = $crate::core::memory::caml_local_roots.clone();
        $code;
        return;
    };

    ($code:tt => $result:expr) => {
        let mut caml_frame = $crate::core::memory::caml_local_roots;
        $code;
        return $crate::core::mlvalues::Value::from($result);
    };
}

#[macro_export]
/// Registers OCaml parameters with the GC
macro_rules! caml_param {

    (@step $idx:expr, $caml_roots:ident,) => {
        $caml_roots.ntables = $idx;
    };

    (@step $idx:expr, $caml_roots:ident, $param:expr, $($tail:expr,)*) => {
        $caml_roots.tables[$idx] = &mut $param;
        caml_param!(@step $idx + 1usize, $caml_roots, $($tail,)*);
    };

    ($($n:expr),*) => {
        let mut caml_roots: $crate::core::memory::CamlRootsBlock = ::std::default::Default::default();
        caml_roots.next = $crate::core::memory::caml_local_roots;
        $crate::core::memory::caml_local_roots = (&mut caml_roots) as *mut $crate::core::memory::CamlRootsBlock;
        caml_roots.nitems = 1; // this is = N when CAMLxparamN is used
        caml_param!(@step 0usize, caml_roots, $($n,)*);
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
        $(let mut $local = $crate::value::Value::new($crate::core::mlvalues::UNIT);)*
        caml_param!($($local.0),*);
    }
}

#[macro_export]
/// Defines an OCaml FFI body, including any locals, as well as a return if provided; it is up to you to define the parameters.
macro_rules! caml_body {

    (||, <$($local:ident),*>, $code:block) => {
        let caml_frame = $crate::core::memory::caml_local_roots;
        caml_local!($($local),*);
        {
            $(let mut $param = $crate::value::Value::new($param);
            {
                let _ = $param;
            })*
            $code;
        }
        $crate::core::memory::caml_local_roots = caml_frame;
    };

    (|$($param:ident),*|, @code $code:block) => {
        let caml_frame = $crate::core::memory::caml_local_roots;
        caml_param!($($param),*);
        {
            $(let mut $param = $crate::value::Value::new($param);
            {
                let _ = $param;
            })*
            $code;
        }
        $crate::core::memory::caml_local_roots = caml_frame;
    };

    (|$($param:ident),*|, <$($local:ident),*>, $code:block) => {
        let caml_frame = $crate::core::memory::caml_local_roots;
        caml_param!($($param),*);
        caml_local!($($local),*);
        {
            $(let mut $param = $crate::value::Value::new($param);
            {
                let _ = $param;
            })*
            $code;
        }
        $crate::core::memory::caml_local_roots = caml_frame;
    }
}

#[macro_export]
/// Defines an external Rust function for FFI use by an OCaml program, with automatic `CAMLparam`, `CAMLlocal`, and `CAMLreturn` inserted for you.
macro_rules! caml {
    ($name:ident, |$($param:ident),*|, <$($local:ident),*>, $code:block -> $retval:ident) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            caml_body!(|$($param),*|, <$($local),*>, $code);
            return $crate::core::mlvalues::Value::from($retval)
        }
    };

    ($name:ident, |$($param:ident),*|, <$($local:ident),*>, $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            caml_body!(|$($param),*|, <$($local),*>, $code);
            return;
        }
    };

    ($name:ident, |$($param:ident),*|, $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) {
            caml_body!(|$($param),*|, @code $code);
            return;
        }
    };

    ($name:ident, |$($param:ident),*|, $code:block -> $retval:ident) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            caml_body!(|$($param),*|, @code $code);
            return $crate::core::mlvalues::Value::from($retval);
        }
    };

}

#[macro_export]
/// Create an OCaml tuple
macro_rules! tuple {
    ($($x:expr),*) => {
        $crate::Tuple::from(&[$($x.to_value(),)*]).into()
    }
}

#[macro_export]
/// Create an OCaml array
macro_rules! array {
    ($($x:expr),*) => {
        $crate::Array::from(&[$($x.to_value(),)*]).into()
    }
}

#[macro_export]
/// Create an OCaml list
macro_rules! list {
    ($($x:expr),*) => {
        $crate::List::from(&[$($x.to_value(),)*]).into()
    }
}
