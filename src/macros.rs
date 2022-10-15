static PANIC_HANDLER_INIT: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

#[doc(hidden)]
pub fn initial_setup() {
    if PANIC_HANDLER_INIT
        .compare_exchange(
            false,
            true,
            core::sync::atomic::Ordering::Relaxed,
            core::sync::atomic::Ordering::Relaxed,
        )
        .is_err()
    {
        return;
    }

    unsafe {
        ocaml_boxroot_sys::boxroot_setup();
    }

    #[cfg(not(feature = "no-panic-hook"))]
    {
        ::std::panic::set_hook(Box::new(|info| unsafe {
            let err = info.payload();
            let msg = if err.is::<&str>() {
                err.downcast_ref::<&str>().unwrap().to_string()
            } else if err.is::<String>() {
                err.downcast_ref::<String>().unwrap().clone()
            } else {
                format!("{:?}", err)
            };

            if let Some(err) = crate::Value::named("Rust_exception") {
                crate::Error::raise_value(err, &msg);
            }

            crate::Error::raise_failure(&msg)
        }))
    }
}

/// `body!` is needed to help the OCaml runtime to manage garbage collection, it should
/// be used to wrap the body of each function exported to OCaml. Panics from Rust code
/// will automatically be unwound/caught here (unless the `no-std` feature is enabled)
///
/// ```rust
/// #[no_mangle]
/// pub unsafe extern "C" fn example(a: ocaml::Value, b: ocaml::Value) -> ocaml::Value {
///     ocaml::body!(gc: {
///         let a = a.int_val();
///         let b = b.int_val();
///         ocaml::Value::int(a + b)
///     })
/// }
/// ```
#[macro_export]
macro_rules! body {
    ($gc:ident: $code:block) => {{
        let $gc = unsafe { $crate::Runtime::recover_handle() };

        // Ensure panic handler is initialized
        $crate::initial_setup();

        {
            $code
        }
    }};
}

#[macro_export]
/// Convenience macro to create an OCaml array
macro_rules! array {
    ($($x:expr),*) => {{
        $crate::ToValue::to_value(&vec![$($crate::ToValue::to_value(&$x)),*])
    }}
}

#[macro_export]
/// Convenience macro to create an OCaml list
macro_rules! list {
    ($($x:expr),*) => {{
        let mut l = $crate::list::empty();
        for i in (&[$($x),*]).into_iter().rev() {
            $crate::list::push_hd(&mut l, $crate::ToValue::to_value(i));
        }
        l
    }};
}

#[macro_export]
/// Import OCaml functions
macro_rules! import {
    ($vis:vis fn $name:ident($($arg:ident: $t:ty),*) $(-> $r:ty)?) => {
        $vis unsafe fn $name(rt: &$crate::Runtime, $($arg: $t),*) -> Result<$crate::interop::default_to_unit!($($r)?), $crate::Error> {
            use $crate::{ToValue, FromValue};
            type R = $crate::interop::default_to_unit!($($r)?);
            let ocaml_rs_named_func = match $crate::Value::named(stringify!($name)) {
                Some(x) => x,
                None => {
                    let msg = concat!(
                        stringify!($name),
                        " has not been registered using Callback.register"
                    );
                    return Err($crate::Error::Message(msg));
                },
            };
            $(let $arg = $arg.to_value(rt);)*
            let __unit = [$crate::Value::unit().raw()];
            let __args = [$($arg.raw()),*];
            let mut args = __args.as_slice();
            if args.is_empty() {
                args = &__unit;
            }
            let x = ocaml_rs_named_func.call_n(args)?;
            Ok(R::from_value(x))
        }
    };
    ($($vis:vis fn $name:ident($($arg:ident: $t:ty),*) $(-> $r:ty)?;)+) => {
        $(
            $crate::import!($vis fn $name($($arg: $t),*) $(-> $r)?);
        )*
    }
}

#[macro_export]
/// Convert OCaml value into a callable closure
///
/// For example, if you have an OCaml closure stored in `f` that accepts two `int` parameters,
/// and returns a string, then you can create a Rust closure like this:
/// ```rust
/// #[ocaml::func]
/// #[ocaml::sig("(int -> int -> string) -> int -> int -> string")]
/// pub fn call_function(f: ocaml::Value, a: ocaml::Int, b: ocaml::Int) -> Result<String, ocaml::Error> {
///     let f = ocaml::function!(f, (a: ocaml::Int, b: ocaml::Int) -> String);
///     f(gc, &a, &b)
/// }
/// ```
macro_rules! function {
    ($x:expr, ($($argname:ident: $arg:ty),*) -> $r:ty) => {
        |gc: &$crate::Runtime, $($argname: &$arg),*| -> Result<$r, $crate::Error> {
            let args = [$($crate::ToValue::to_value($argname, gc)),*];
            #[allow(unused_unsafe)]
            unsafe { $crate::Value::call(&$x, gc, args) }
        }
    };
}
