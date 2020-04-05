#[macro_export]
/// Defines an external Rust function for FFI use by an OCaml program
macro_rules! caml {
    ($name:ident($($param:ident),*) $code:block) => {
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
