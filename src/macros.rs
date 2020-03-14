#[macro_export]
macro_rules! caml_ffi {
    ($code:tt) => {
        let mut caml_frame = $crate::core::state::local_roots();
        $code;
        return;
    };

    ($code:tt => $result:expr) => {
        let mut caml_frame = $crate::core::state::local_roots();
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
        $crate::caml_param!(@step $idx + 1usize, $caml_roots, $($tail,)*);
    };

    ($($n:expr),*) => {
        let mut caml_roots: $crate::core::memory::CamlRootsBlock = ::std::default::Default::default();
        #[allow(unused_unsafe)]
        {
            caml_roots.next = unsafe { $crate::core::state::local_roots() };
            unsafe {
                $crate::core::state::set_local_roots(&mut caml_roots);
            }
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
        $(let mut $local = $crate::Value::new($crate::core::mlvalues::UNIT);)*
        $crate::caml_param!($($local.0),*);
    }
}

#[macro_export]
/// Defines an OCaml frame
macro_rules! caml_frame {
    (|$($local:ident),*| $code:block) => {
        {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::core::state::local_roots() };
            caml_local!($($local),*);
            let res = $code;
            #[allow(unused_unsafe)]
            unsafe { $crate::core::state::set_local_roots(caml_frame) };
            res
        }
    };
}

#[macro_export]
/// Defines an OCaml FFI body, including any locals, as well as a return if provided; it is up to you to define the parameters.
macro_rules! caml_body {
    (||, <$($local:ident),*>, $code:block) => {
        #[allow(unused_unsafe)]
        let caml_frame = unsafe { $crate::core::state::local_roots() };
        $crate::caml_local!($($local),*);
        $code;
        #[allow(unused_unsafe)]
        unsafe { $crate::core::state::set_local_roots(caml_frame) };
    };

    (|$($param:ident),*|, @code $code:block) => {
        #[allow(unused_unsafe)]
        let caml_frame = unsafe { $crate::core::state::local_roots() };
        $($crate::caml_param!($param); let $param = $crate::Value::new($param);)*
        $code;
        #[allow(unused_unsafe)]
        unsafe { $crate::core::state::set_local_roots(caml_frame) };
    };

    (|$($param:ident),*|, <$($local:ident),*>, $code:block) => {
        #[allow(unused_unsafe)]
        let caml_frame = unsafe { $crate::core::state::local_roots() };
        $($crate::caml_param!($param); let $param = $crate::Value::new($param);)*
        $crate::caml_local!($($local),*);
        $code
        #[allow(unused_unsafe)]
        unsafe { $crate::core::state::set_local_roots(caml_frame) };
    }
}

#[macro_export]
/// Defines an external Rust function for FFI use by an OCaml program, with automatic `CAMLparam`, `CAMLlocal`, and `CAMLreturn` inserted for you.
macro_rules! caml {
    ($name:ident($($param:ident),*) $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            let x: $crate::Value;
            $crate::caml_body!(|$($param),*|, <>, {
                x = (|| -> $crate::Value { $code })();
            });
            return x.0;
        }
    };

    ($name:ident, |$($param:ident),*|, <$($local:ident),*>, $code:block -> $retval:ident) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            $crate::caml_body!(|$($param),*|, <$($local),*>, $code);
            return $crate::core::mlvalues::Value::from($retval);
        }
    };

    ($name:ident, |$($param:ident),*|, <$($local:ident),*>, $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            $crate::caml_body!(|$($param),*|, <$($local),*>, $code);
            return $crate::core::mlvalues::UNIT;
        }
    };

    ($name:ident, |$($param:ident),*|, $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            $crate::caml_body!(|$($param),*|, @code $code);
            return $crate::core::mlvalues::UNIT;
        }
    };

    ($name:ident, |$($param:ident),*|, $code:block -> $retval:ident) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::core::mlvalues::Value,)*) -> $crate::core::mlvalues::Value {
            $crate::caml_body!(|$($param),*|, @code $code);
            return $crate::core::mlvalues::Value::from($retval);
        }
    };

}

#[macro_export]
/// Create an OCaml tuple
macro_rules! tuple {
    (_ $x:expr) => {
        {
            let mut t = $crate::Tuple::new($x.len());
            for (n, i) in $x.into_iter().enumerate() {
                let _ = t.set(n, $crate::ToValue::to_value(&i));
            }
            t
        }
    };
    ($($x:expr),*) => {
        {
            let x =  &[$($x.to_value(),)*];
            let x = tuple!(_ x);
            x
        }
    }
}

#[macro_export]
/// Create an OCaml array
macro_rules! array {
    (_ $x:expr) => {
        {
            let mut a = $crate::Array::new($x.len());
            for (n, i) in $x.into_iter().enumerate() {
                let _ = a.set(n, $crate::ToValue::to_value(&i));
            }
            a
        }
    };
    ($($x:expr),*) => {
        {
            let x =  &[$($x.to_value(),)*];
            let x = array!(_ x);
            x
        }
    }
}

#[macro_export]
/// Create an OCaml list
macro_rules! list {
    (_ $x:expr) => {
        {
            let mut l = $crate::List::new();
            for i in $x.into_iter().rev() {
                l.push_hd($crate::ToValue::to_value(&i));
            }
            l
        }
    };
    ($($x:expr),*) => {
        {
            let x =  &[$($x.to_value(),)*];
            let x = list!(_ x);
            x
        }
    }
}
