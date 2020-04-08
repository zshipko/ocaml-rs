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
macro_rules! body {
    (($($param:expr),*) $code:block) => {
        $crate::sys::caml_body!(($($param.0),*) $code);
    }
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
