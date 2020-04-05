#[macro_export]
macro_rules! local {
    ($($local:ident),*) => {
        #[allow(unused_mut)]
        $(let mut $local = $crate::Value($crate::sys::mlvalues::UNIT);)*
        #[allow(unused_unsafe)]
        $crate::sys::caml_param!($($local.0),*);
    }
}

#[macro_export]
/// Defines an external Rust function for FFI use by an OCaml program
macro_rules! caml {
    (fn $name:ident($($param:ident),*) $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern "C" fn $name ($(mut $param: $crate::Value,)*) -> $crate::Value {
            $crate::sys::caml_body!(($($param.0),*) {
                let x = || $code;
                let x = x();
                $crate::ToValue::to_value(&x)
            })
        }
    };
}

#[macro_export]
/// Create an OCaml array
macro_rules! array {
    ($($x:expr),*) => {{
        $crate::ToValue::to_value(&vec![$($crate::ToValue::to_value(&$x)),*])
    }}
}

#[macro_export]
/// Create an OCaml list
macro_rules! list {
    ($($x:expr),*) => {{
        let mut l = $crate::list::empty();
        for i in (&[$($x),*]).into_iter().rev() {
            $crate::list::push_hd(&mut l, $crate::ToValue::to_value(i));
        }
        l
    }};
}
