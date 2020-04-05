#[macro_export]
/// Defines an external Rust function for FFI use by an OCaml program
macro_rules! caml {
    (fn $name:ident($($param:ident),*) $code:block) => {
        #[allow(unused_mut)]
        #[no_mangle]
        pub unsafe extern "C" fn $name ($(mut $param: $crate::Value,)*) -> $crate::Value {
            caml_body!(($($param.0),*) $code)
        }
    };
}

#[macro_export]
/// Create an OCaml tuple
macro_rules! tuple {
    ($($x:expr),*) => {{
        array!($($x),*)
    }}
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
        let mut l = $crate::List::new();
        for i in (&[$($x),*]).into_iter().rev() {
            l.push_hd($crate::ToValue::to_value(i));
        }
        l
    }};
}
